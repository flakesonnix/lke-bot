/// Display information about a user
#[poise::command(slash_command)]
pub async fn userinfo(
    ctx: crate::Context<'_>,
    #[description = "User to inspect"] user: Option<poise::serenity_prelude::User>,
) -> Result<(), crate::Error> {
    let user = user.as_ref().unwrap_or_else(|| ctx.author());

    let discriminator = user
        .discriminator
        .map(|d| format!("#{:04}", d.get()))
        .unwrap_or_default();

    let reply = format!(
        "**User Information**\n\
        Username: {}\n\
        ID: {}\n\
        Discriminator: {}\n\
        Bot: {}\n\
        Avatar URL: {}",
        user.name,
        user.id,
        discriminator,
        user.bot,
        user.avatar_url().unwrap_or_default()
    );

    ctx.say(reply).await?;
    Ok(())
}
