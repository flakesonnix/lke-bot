mod bot;
mod leveling;
mod moderation;
mod music;
mod tickets;
mod tts;

pub use bot::bot_settings;
pub use leveling::leveling_settings;
pub use moderation::moderation_settings;
pub use music::music_settings;
pub use tickets::ticket_settings;
pub use tts::tts_settings;

use crate::templates::base_html;
use axum::response::Html;

pub fn nav(active: &str) -> String {
    let items = [
        ("dashboard", "Dashboard", "/dashboard"),
        ("bot", "Bot Activity", "/settings/bot"),
        ("leveling", "Leveling", "/settings/leveling"),
        ("tickets", "Tickets", "/settings/tickets"),
        ("moderation", "Moderation", "/settings/moderation"),
        ("tts", "Text-to-Speech", "/settings/tts"),
        ("music", "Music", "/settings/music"),
    ];

    let nav_items: String = items
        .iter()
        .map(|(key, label, href)| {
            if *key == active {
                format!(
                    r#"<a href="{}" class="px-4 py-2 bg-indigo-600 rounded">{}</a>"#,
                    href, label
                )
            } else {
                format!(
                    r#"<a href="{}" class="px-4 py-2 bg-gray-700 hover:bg-gray-600 rounded">{}</a>"#,
                    href, label
                )
            }
        })
        .collect();

    format!(
        r#"<nav class="flex flex-wrap gap-2 mb-6">{}</nav>"#,
        nav_items
    )
}

pub fn settings_page(title: &str, active: &str, content: &str) -> Html<String> {
    let nav = nav(active);
    let html = format!(
        r#"<div class="max-w-4xl mx-auto">
            <h1 class="text-3xl font-bold mb-4">Settings</h1>
            {}
            <div class="bg-gray-800 rounded-lg p-6">
                <h2 class="text-xl font-semibold mb-4">{}</h2>
                {}
            </div>
        </div>"#,
        nav, title, content
    );
    Html(base_html(title, &html, true))
}
