use crate::templates::settings::settings_page;
use axum::response::Html;
use shared::{LevelSettings, UserLevel};

pub fn leveling_settings(
    settings: Option<LevelSettings>,
    leaderboard: Vec<UserLevel>,
) -> Html<String> {
    let s = settings.as_ref();
    let enabled = s.map(|s| s.enabled).unwrap_or(true);
    let xp_per_message = s.map(|s| s.xp_per_message).unwrap_or(15);
    let cooldown = s.map(|s| s.cooldown_seconds).unwrap_or(60);
    let announce_channel = s
        .and_then(|s| s.announce_channel_id.as_deref())
        .unwrap_or("");
    let level_up_msg = s
        .map(|s| s.level_up_message.as_str())
        .unwrap_or("🎉 {user} has reached level {level}!");

    let leaderboard_html = if leaderboard.is_empty() {
        "<p class='text-gray-400'>No users have earned XP yet.</p>".to_string()
    } else {
        leaderboard
            .iter()
            .take(10)
            .enumerate()
            .map(|(i, ul)| {
                let medal = match i {
                    0 => "🥇",
                    1 => "🥈",
                    2 => "🥉",
                    _ => &format!("#{}", i + 1),
                };
                format!(
                    r#"<div class="flex justify-between items-center p-2 {} rounded">
                    <span>{} <span class="text-gray-400">&lt;@{}&gt;</span></span>
                    <span class="text-sm">Level {} · {} XP</span>
                </div>"#,
                    if i % 2 == 0 { "bg-gray-700/50" } else { "" },
                    medal,
                    ul.user_id,
                    ul.level,
                    ul.total_xp
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    };

    let content = format!(
        r#"<form id="leveling-settings-form" class="space-y-6">
            <div class="flex items-center justify-between p-4 bg-gray-700 rounded-lg">
                <div>
                    <h3 class="font-medium">Enable Leveling System</h3>
                    <p class="text-sm text-gray-400">Users earn XP from messages and gain levels</p>
                </div>
                <label class="relative inline-flex items-center cursor-pointer">
                    <input type="checkbox" name="enabled" {} class="sr-only peer">
                    <div class="w-11 h-6 bg-gray-600 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-indigo-600"></div>
                </label>
            </div>

            <div class="grid grid-cols-2 gap-4">
                <div>
                    <label class="block text-gray-400 mb-2">XP per Message</label>
                    <input type="number" name="xp_per_message" value="{}" min="1" max="100"
                           class="w-full bg-gray-700 rounded-lg p-3 text-white">
                </div>
                <div>
                    <label class="block text-gray-400 mb-2">Cooldown (seconds)</label>
                    <input type="number" name="cooldown_seconds" value="{}" min="0" max="300"
                           class="w-full bg-gray-700 rounded-lg p-3 text-white">
                </div>
            </div>

            <div>
                <label class="block text-gray-400 mb-2">Level-up Announcement Channel</label>
                <select name="announce_channel_id" id="channel-select" 
                        class="w-full bg-gray-700 rounded-lg p-3 text-white">
                    <option value="">-- Select a channel --</option>
                </select>
                <p class="text-xs text-gray-500 mt-1" id="channel-loading">Loading channels...</p>
            </div>

            <div>
                <label class="block text-gray-400 mb-2">Level-up Message</label>
                <textarea name="level_up_message" rows="2"
                          class="w-full bg-gray-700 rounded-lg p-3 text-white"
                          placeholder="Use {{user}} and {{level}} placeholders">{}</textarea>
                <p class="text-xs text-gray-500 mt-1">Placeholders: &#123;user&#125;, &#123;level&#125;</p>
            </div>

            <div id="result" class="hidden p-4 rounded-lg"></div>

            <button type="submit" class="w-full bg-indigo-600 hover:bg-indigo-700 px-6 py-3 rounded-lg font-medium transition">
                Save Leveling Settings
            </button>
        </form>

        <div class="mt-8">
            <h3 class="text-lg font-medium mb-4">🏆 Leaderboard</h3>
            <div class="bg-gray-800 rounded-lg p-4 space-y-2">
                {}
            </div>
        </div>

        <script>
        (async () => {{
            try {{
                const res = await fetch('/api/guild/resources');
                const data = await res.json();
                
                const select = document.getElementById('channel-select');
                const loading = document.getElementById('channel-loading');
                const currentChannel = "{}";
                
                data.channels.forEach(ch => {{
                    const opt = document.createElement('option');
                    opt.value = ch.id;
                    opt.textContent = ch.icon + ' ' + ch.name;
                    if (ch.id === currentChannel) opt.selected = true;
                    select.appendChild(opt);
                }});
                
                loading.textContent = data.channels.length + ' channels available';
                loading.className = 'text-xs text-green-500 mt-1';
            }} catch (err) {{
                document.getElementById('channel-loading').textContent = 'Failed to load channels';
                document.getElementById('channel-loading').className = 'text-xs text-red-500 mt-1';
            }}
        }})();

        document.getElementById('leveling-settings-form').addEventListener('submit', async (e) => {{
            e.preventDefault();
            const form = e.target;
            const data = {{
                enabled: form.enabled.checked,
                xp_per_message: parseInt(form.xp_per_message.value),
                cooldown_seconds: parseInt(form.cooldown_seconds.value),
                announce_channel_id: form.announce_channel_id.value || null,
                level_up_message: form.level_up_message.value
            }};
            
            const result = document.getElementById('result');
            result.classList.remove('hidden');
            result.className = 'p-4 rounded-lg bg-blue-600/20 text-blue-400';
            result.textContent = 'Saving...';
            
            try {{
                const res = await fetch('/api/leveling/settings', {{
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
        xp_per_message,
        cooldown,
        level_up_msg,
        leaderboard_html,
        announce_channel
    );

    settings_page("Leveling System", "leveling", &content)
}
