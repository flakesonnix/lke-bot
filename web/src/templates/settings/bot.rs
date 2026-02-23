use crate::templates::settings::settings_page;
use axum::response::Html;
use shared::BotSettings;

pub fn bot_settings(settings: Option<BotSettings>) -> Html<String> {
    let s = settings.as_ref();
    let enabled = s.map(|s| s.activity_enabled).unwrap_or(false);
    let activity_type = s.map(|s| s.activity_type.as_str()).unwrap_or("playing");
    let activity_name = s.map(|s| s.activity_name.as_str()).unwrap_or("with code");
    let activity_url = s.and_then(|s| s.activity_url.as_deref()).unwrap_or("");

    let content = format!(
        r#"<form id="bot-settings-form" class="space-y-6">
            <div class="flex items-center justify-between p-4 bg-gray-700 rounded-lg">
                <div>
                    <h3 class="font-medium">Enable Activity</h3>
                    <p class="text-sm text-gray-400">Show a status activity for the bot</p>
                </div>
                <label class="relative inline-flex items-center cursor-pointer">
                    <input type="checkbox" name="activity_enabled" {} class="sr-only peer">
                    <div class="w-11 h-6 bg-gray-600 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-indigo-600"></div>
                </label>
            </div>

            <div>
                <label class="block text-gray-400 mb-2">Activity Type</label>
                <select name="activity_type" class="w-full bg-gray-700 rounded-lg p-3 text-white">
                    <option value="playing" {}>🎮 Playing</option>
                    <option value="streaming" {}>📺 Streaming</option>
                    <option value="listening" {}>🎧 Listening</option>
                    <option value="watching" {}>👀 Watching</option>
                    <option value="competing" {}>🏆 Competing</option>
                </select>
            </div>

            <div>
                <label class="block text-gray-400 mb-2">Activity Name</label>
                <input type="text" name="activity_name" value="{}" 
                       class="w-full bg-gray-700 rounded-lg p-3 text-white" 
                       placeholder="e.g., with code">
            </div>

            <div>
                <label class="block text-gray-400 mb-2">Stream URL (only for Streaming type)</label>
                <input type="url" name="activity_url" value="{}" 
                       class="w-full bg-gray-700 rounded-lg p-3 text-white" 
                       placeholder="https://twitch.tv/username">
            </div>

            <div id="result" class="hidden p-4 rounded-lg"></div>

            <button type="submit" class="w-full bg-indigo-600 hover:bg-indigo-700 px-6 py-3 rounded-lg font-medium transition">
                Save Bot Settings
            </button>
        </form>

        <script>
        document.getElementById('bot-settings-form').addEventListener('submit', async (e) => {{
            e.preventDefault();
            const form = e.target;
            const data = {{
                activity_enabled: form.activity_enabled.checked,
                activity_type: form.activity_type.value,
                activity_name: form.activity_name.value,
                activity_url: form.activity_url.value || null
            }};
            
            const result = document.getElementById('result');
            result.classList.remove('hidden');
            result.className = 'p-4 rounded-lg bg-blue-600/20 text-blue-400';
            result.textContent = 'Saving...';
            
            try {{
                const res = await fetch('/api/settings', {{
                    method: 'POST',
                    headers: {{ 'Content-Type': 'application/json' }},
                    body: JSON.stringify(data)
                }});
                const json = await res.json();
                
                if (json.success) {{
                    result.className = 'p-4 rounded-lg bg-green-600/20 text-green-400';
                }} else {{
                    result.className = 'p-4 rounded-lg bg-red-600/20 text-red-400';
                }}
                result.textContent = json.message;
            }} catch (err) {{
                result.className = 'p-4 rounded-lg bg-red-600/20 text-red-400';
                result.textContent = 'Failed to save: ' + err.message;
            }}
        }});
        </script>"#,
        if enabled { "checked" } else { "" },
        if activity_type == "playing" {
            "selected"
        } else {
            ""
        },
        if activity_type == "streaming" {
            "selected"
        } else {
            ""
        },
        if activity_type == "listening" {
            "selected"
        } else {
            ""
        },
        if activity_type == "watching" {
            "selected"
        } else {
            ""
        },
        if activity_type == "competing" {
            "selected"
        } else {
            ""
        },
        activity_name,
        activity_url
    );

    settings_page("Bot Activity", "bot", &content)
}
