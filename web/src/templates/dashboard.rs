use crate::session::{DiscordGuild, DiscordUser};
use axum::response::{Html, IntoResponse};
use shared::User;

pub fn dashboard(
    discord_user: DiscordUser,
    db_user: Option<User>,
    user_count: i64,
    guilds: Vec<DiscordGuild>,
) -> impl IntoResponse {
    let avatar_url = discord_user.avatar_url().unwrap_or_else(|| {
        format!(
            "https://cdn.discordapp.com/embed/avatars/{}.png",
            discord_user
                .discriminator
                .as_ref()
                .and_then(|d| d.parse::<u8>().ok())
                .unwrap_or(0)
                % 5
        )
    });

    let admin_guilds: Vec<_> = guilds.iter().filter(|g| g.has_admin()).collect();

    let user_info = db_user
        .map(|u| {
            format!(
                r#"<div class="mt-4 text-sm text-gray-400">
                    <p>Account created: {}</p>
                    <p>Last updated: {}</p>
                </div>"#,
                u.created_at, u.updated_at
            )
        })
        .unwrap_or_default();

    let guilds_section = if !admin_guilds.is_empty() {
        let guild_items: String = admin_guilds
            .iter()
            .map(|g| {
                let icon = g
                    .icon_url()
                    .unwrap_or_else(|| "https://cdn.discordapp.com/embed/avatars/0.png".to_string());
                format!(
                    r#"<div class="flex items-center gap-3 p-3 bg-gray-700 rounded-lg">
                        <img src="{}" class="w-12 h-12 rounded-full">
                        <div class="flex-1">
                            <h4 class="font-semibold">{}</h4>
                            <p class="text-sm text-gray-400">Admin</p>
                        </div>
                        <span class="px-3 py-1 bg-green-600/20 text-green-400 rounded text-sm">Admin</span>
                    </div>"#,
                    icon, g.name
                )
            })
            .collect();

        format!(
            r#"<div class="bg-gray-800 rounded-lg p-6 mb-6">
                <h3 class="text-xl font-semibold mb-4">Your Servers</h3>
                <div class="space-y-3">{}</div>
            </div>"#,
            guild_items
        )
    } else {
        String::new()
    };

    let settings_nav = super::settings::nav("dashboard");

    let content = format!(
        r#"<div class="max-w-4xl mx-auto">
            <h1 class="text-3xl font-bold mb-4">Dashboard</h1>
            {}
            
            <div class="bg-gray-800 rounded-lg p-6 mb-6">
                <div class="flex items-center gap-6">
                    <img src="{avatar_url}" alt="Avatar" class="w-24 h-24 rounded-full ring-2 ring-indigo-500">
                    <div>
                        <h2 class="text-2xl font-semibold">{username}</h2>
                        <p class="text-gray-400">Discord ID: {discord_id}</p>
                        {discriminator}
                    </div>
                </div>
                {user_info}
            </div>
            
            <div class="bg-gray-800 rounded-lg p-6 mb-6">
                <h3 class="text-xl font-semibold mb-4">Statistics</h3>
                <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
                    <div class="bg-gray-700 rounded p-4 text-center">
                        <p class="text-gray-400 text-sm">Total Users</p>
                        <p class="text-3xl font-bold text-indigo-400">{user_count}</p>
                    </div>
                    <div class="bg-gray-700 rounded p-4 text-center">
                        <p class="text-gray-400 text-sm">Admin Servers</p>
                        <p class="text-3xl font-bold text-green-400">{admin_count}</p>
                    </div>
                    <div class="bg-gray-700 rounded p-4 text-center">
                        <p class="text-gray-400 text-sm">Bot Latency</p>
                        <p class="text-3xl font-bold text-yellow-400" id="bot-latency">--</p>
                    </div>
                    <div class="bg-gray-700 rounded p-4 text-center">
                        <p class="text-gray-400 text-sm">API Status</p>
                        <p class="text-3xl font-bold text-green-400">✓</p>
                    </div>
                </div>
            </div>

            {guilds_section}

            <div class="bg-gray-800 rounded-lg p-6">
                <h3 class="text-xl font-semibold mb-4">Quick Actions</h3>
                <div class="grid grid-cols-2 md:grid-cols-3 gap-4">
                    <a href="/settings/bot" class="p-4 bg-gray-700 rounded-lg hover:bg-gray-600 transition text-center">
                        <span class="text-2xl">🤖</span>
                        <p class="mt-2 font-medium">Bot Activity</p>
                    </a>
                    <a href="/settings/tickets" class="p-4 bg-gray-700 rounded-lg hover:bg-gray-600 transition text-center">
                        <span class="text-2xl">🎫</span>
                        <p class="mt-2 font-medium">Tickets</p>
                    </a>
                    <a href="/settings/moderation" class="p-4 bg-gray-700 rounded-lg hover:bg-gray-600 transition text-center">
                        <span class="text-2xl">🛡️</span>
                        <p class="mt-2 font-medium">Moderation</p>
                    </a>
                    <a href="/settings/tts" class="p-4 bg-gray-700 rounded-lg hover:bg-gray-600 transition text-center">
                        <span class="text-2xl">🔊</span>
                        <p class="mt-2 font-medium">Text-to-Speech</p>
                    </a>
                    <a href="/settings/music" class="p-4 bg-gray-700 rounded-lg hover:bg-gray-600 transition text-center">
                        <span class="text-2xl">🎵</span>
                        <p class="mt-2 font-medium">Music</p>
                    </a>
                    <a href="/logout" class="p-4 bg-gray-700 rounded-lg hover:bg-gray-600 transition text-center">
                        <span class="text-2xl">🚪</span>
                        <p class="mt-2 font-medium">Logout</p>
                    </a>
                </div>
            </div>
        </div>

        <script>
            async function updateLatency() {{
                const start = Date.now();
                try {{
                    await fetch('/api/ping');
                    const latency = Date.now() - start;
                    document.getElementById('bot-latency').textContent = latency + 'ms';
                }} catch (e) {{
                    document.getElementById('bot-latency').textContent = 'Error';
                }}
            }}
            updateLatency();
            setInterval(updateLatency, 30000);
        </script>"#,
        settings_nav,
        avatar_url = avatar_url,
        username = discord_user.username,
        discord_id = discord_user.id,
        discriminator = discord_user
            .discriminator
            .as_ref()
            .map(|d| format!("<p class=\"text-gray-500 text-sm\">#{}</p>", d))
            .unwrap_or_default(),
        user_info = user_info,
        user_count = user_count,
        admin_count = admin_guilds.len(),
        guilds_section = guilds_section,
    );

    Html(super::base_html("Dashboard", &content, true))
}
