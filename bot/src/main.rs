//! Bot entry point – Poise slash command handling

use anyhow::Result;
use poise::serenity_prelude::{
    ActivityData, ChannelId, FullEvent, GatewayIntents, GuildId, OnlineStatus,
};
use poise::{self, FrameworkOptions, PrefixFrameworkOptions};
use songbird::SerenityInit;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::watch;

use shared::{
    config::Config, db, repository::{BotSettingsRepository, LevelRepository, WelcomeRepository},
    BotSettings,
};

use events::{LevelingHandler, WelcomeHandler};

pub struct BotState {
    pub settings_repo: BotSettingsRepository,
    pub level_repo: LevelRepository,
    pub welcome_repo: WelcomeRepository,
    pub activity_rx: watch::Receiver<Option<ActivityData>>,
}

pub type Data = Arc<BotState>;
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

async fn event_handler(
    ctx: &poise::serenity_prelude::Context,
    event: &FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        FullEvent::Message { new_message } => {
            LevelingHandler::handle_message(ctx, new_message, data).await?;
        }
        FullEvent::GuildMemberAddition { new_member } => {
            WelcomeHandler::handle_member_join(ctx, new_member, data).await?;
        }
        FullEvent::GuildMemberRemoval { guild_id, user, .. } => {
            WelcomeHandler::handle_member_leave(ctx, *guild_id, user, data).await?;
        }
        _ => {}
    }
    Ok(())
}

fn settings_to_activity(settings: &BotSettings) -> Option<ActivityData> {
    if !settings.activity_enabled {
        return None;
    }

    match settings.activity_type.as_str() {
        "playing" => Some(ActivityData::playing(&settings.activity_name)),
        "streaming" => {
            let url = settings.activity_url.as_deref().unwrap_or("");
            ActivityData::streaming(&settings.activity_name, url).ok()
        }
        "listening" => Some(ActivityData::listening(&settings.activity_name)),
        "watching" => Some(ActivityData::watching(&settings.activity_name)),
        "competing" => Some(ActivityData::competing(&settings.activity_name)),
        _ => Some(ActivityData::playing(&settings.activity_name)),
    }
}

async fn activity_watcher(
    ctx: poise::serenity_prelude::Context,
    repo: BotSettingsRepository,
    tx: watch::Sender<Option<ActivityData>>,
    running: Arc<AtomicBool>,
) {
    let mut current_hash: u64 = 0;

    while running.load(Ordering::Relaxed) {
        tokio::time::sleep(Duration::from_secs(10)).await;

        match repo.get().await {
            Ok(settings) => {
                let new_hash = {
                    let mut hasher = DefaultHasher::new();
                    settings.activity_enabled.hash(&mut hasher);
                    settings.activity_type.hash(&mut hasher);
                    settings.activity_name.hash(&mut hasher);
                    settings.activity_url.hash(&mut hasher);
                    hasher.finish()
                };

                if new_hash != current_hash {
                    current_hash = new_hash;

                    if let Some(activity) = settings_to_activity(&settings) {
                        println!("Updating activity: {} {}", settings.activity_type, settings.activity_name);
                        ctx.set_presence(Some(activity.clone()), OnlineStatus::Online);
                        let _ = tx.send(Some(activity));
                    } else {
                        println!("Clearing activity");
                        ctx.set_presence(None, OnlineStatus::Online);
                        let _ = tx.send(None);
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to fetch bot settings: {}", e);
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let cfg = Config::from_env();
    let voice_channel_id = cfg.general_channel_id;
    let guild_id = cfg.guild_id;

    let pool = db::init_db(&cfg.database_url).await?;
    let settings_repo = BotSettingsRepository::new(pool.clone());

    let initial_settings = settings_repo.get().await.ok();
    let initial_activity = initial_settings
        .as_ref()
        .and_then(settings_to_activity);

    let (activity_tx, activity_rx) = watch::channel(initial_activity.clone());

    let framework = poise::Framework::builder()
        .options(FrameworkOptions {
            commands: vec![
                commands::ping(),
                commands::userinfo(),
                commands::help(),
                commands::roll(),
                commands::rank(),
                commands::leaderboard(),
                commands::setxp(),
                commands::addxp(),
            ],
            prefix_options: PrefixFrameworkOptions {
                prefix: Some("!".into()),
                ..Default::default()
            },
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(move |ctx, ready, framework| {
            let settings_repo = BotSettingsRepository::new(pool.clone());
            let voice_id = voice_channel_id;
            let guild = guild_id;
            let activity_tx = activity_tx.clone();

            Box::pin(async move {
                println!("🤖 Logged in as {}", ready.user.name);

                if let Some(ref activity) = initial_activity {
                    ctx.set_presence(Some(activity.clone()), OnlineStatus::Online);
                }

                if let Some(guild_id) = guild {
                    let guild_id = GuildId::new(guild_id);
                    poise::builtins::register_in_guild(
                        ctx,
                        &framework.options().commands,
                        guild_id,
                    )
                    .await?;

                    if let Some(channel_id) = voice_id {
                        let channel = ChannelId::new(channel_id);
                        let manager = songbird::get(ctx)
                            .await
                            .expect("Songbird Voice client placed in at initialization");
                        
                        if let Err(e) = manager.join(guild_id, channel).await {
                            eprintln!("Failed to join voice channel: {:?}", e);
                        } else {
                            println!("Joined voice channel {}", channel_id);
                        }
                    }
                }

                let running = Arc::new(AtomicBool::new(true));
                tokio::spawn(activity_watcher(
                    ctx.clone(),
                    settings_repo,
                    activity_tx,
                    running,
                ));

                Ok(Arc::new(BotState {
                    settings_repo: BotSettingsRepository::new(pool.clone()),
                    level_repo: LevelRepository::new(pool.clone()),
                    welcome_repo: WelcomeRepository::new(pool.clone()),
                    activity_rx,
                }))
            })
        })
        .build();

    let intents = GatewayIntents::non_privileged() 
        | GatewayIntents::MESSAGE_CONTENT 
        | GatewayIntents::GUILD_MEMBERS;

    let mut client = poise::serenity_prelude::ClientBuilder::new(&cfg.discord_token, intents)
        .framework(framework)
        .register_songbird()
        .await?;

    client.start().await?;
    Ok(())
}

mod commands;
mod events;
