use crate::templates::settings::settings_page;
use axum::response::Html;
use shared::ModerationSettings;

pub fn moderation_settings(settings: Option<ModerationSettings>) -> Html<String> {
    let s = settings.as_ref();
    let enabled = s.map(|s| s.enabled).unwrap_or(false);
    let check_bad_words = s.map(|s| s.check_bad_words).unwrap_or(true);
    let check_bad_names = s.map(|s| s.check_bad_names).unwrap_or(true);
    let check_nsfw = s.map(|s| s.check_nsfw_avatars).unwrap_or(true);
    let log_channel = s.and_then(|s| s.log_channel_id.as_deref()).unwrap_or("");
    let mute_role = s.and_then(|s| s.mute_role_id.as_deref()).unwrap_or("");
    let warn_threshold = s.map(|s| s.warn_threshold).unwrap_or(3);
    let auto_mute = s.map(|s| s.auto_mute).unwrap_or(true);
    let language = s.map(|s| s.language.as_str()).unwrap_or("en");

    let content = format!(
        r#"<form id="moderation-settings-form" class="space-y-6">
            <div class="flex items-center justify-between p-4 bg-gray-700 rounded-lg">
                <div>
                    <h3 class="font-medium">Enable Auto-Moderation</h3>
                    <p class="text-sm text-gray-400">Automatically moderate messages and profiles</p>
                </div>
                <label class="relative inline-flex items-center cursor-pointer">
                    <input type="checkbox" name="enabled" {} class="sr-only peer">
                    <div class="w-11 h-6 bg-gray-600 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-indigo-600"></div>
                </label>
            </div>

            <div>
                <label class="block text-gray-400 mb-3">Moderation Features</label>
                <div class="space-y-3">
                    <label class="flex items-center gap-3 p-3 bg-gray-700 rounded-lg cursor-pointer hover:bg-gray-600">
                        <input type="checkbox" name="check_bad_words" {} class="w-5 h-5 rounded">
                        <div>
                            <span class="font-medium">Bad Words Filter</span>
                            <p class="text-sm text-gray-400">Delete messages containing profanity</p>
                        </div>
                    </label>
                    <label class="flex items-center gap-3 p-3 bg-gray-700 rounded-lg cursor-pointer hover:bg-gray-600">
                        <input type="checkbox" name="check_bad_names" {} class="w-5 h-5 rounded">
                        <div>
                            <span class="font-medium">Bad Names Filter</span>
                            <p class="text-sm text-gray-400">Check usernames and nicknames for inappropriate content</p>
                        </div>
                    </label>
                    <label class="flex items-center gap-3 p-3 bg-gray-700 rounded-lg cursor-pointer hover:bg-gray-600">
                        <input type="checkbox" name="check_nsfw_avatars" {} class="w-5 h-5 rounded">
                        <div>
                            <span class="font-medium">NSFW Avatar Detection</span>
                            <p class="text-sm text-gray-400">Flag inappropriate profile pictures</p>
                        </div>
                    </label>
                </div>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div>
                    <label class="block text-gray-400 mb-2">Log Channel</label>
                    <select name="log_channel_id" id="log-channel-select" 
                            class="w-full bg-gray-700 rounded-lg p-3 text-white">
                        <option value="">-- Select a channel --</option>
                    </select>
                    <p class="text-xs text-gray-500 mt-1" id="channel-loading">Loading channels...</p>
                </div>
                <div>
                    <label class="block text-gray-400 mb-2">Mute Role</label>
                    <select name="mute_role_id" id="mute-role-select" 
                            class="w-full bg-gray-700 rounded-lg p-3 text-white">
                        <option value="">-- Select a role --</option>
                    </select>
                    <p class="text-xs text-gray-500 mt-1" id="role-loading">Loading roles...</p>
                </div>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                <div>
                    <label class="block text-gray-400 mb-2">Warn Threshold</label>
                    <input type="number" name="warn_threshold" value="{}" min="1" max="10"
                           class="w-full bg-gray-700 rounded-lg p-3 text-white">
                    <p class="text-xs text-gray-500 mt-1">Warnings before auto-mute</p>
                </div>
                <div>
                    <label class="block text-gray-400 mb-2">Language</label>
                    <select name="language" class="w-full bg-gray-700 rounded-lg p-3 text-white">
                        <option value="en" {}>English</option>
                        <option value="de" {}>German (Deutsch)</option>
                    </select>
                </div>
                <div class="flex items-end">
                    <label class="flex items-center gap-3 p-3 bg-gray-700 rounded-lg cursor-pointer w-full">
                        <input type="checkbox" name="auto_mute" {} class="w-5 h-5 rounded">
                        <span class="font-medium">Auto-Mute</span>
                    </label>
                </div>
            </div>

            <div id="result" class="hidden p-4 rounded-lg"></div>

            <button type="submit" class="w-full bg-indigo-600 hover:bg-indigo-700 px-6 py-3 rounded-lg font-medium transition">
                Save Moderation Settings
            </button>
        </form>

        <script>
        (async () => {{
            try {{
                const res = await fetch('/api/guild/resources');
                const data = await res.json();
                
                const channelSelect = document.getElementById('log-channel-select');
                const roleSelect = document.getElementById('mute-role-select');
                const currentChannel = "{}";
                const currentRole = "{}";
                
                data.channels.forEach(ch => {{
                    const opt = document.createElement('option');
                    opt.value = ch.id;
                    opt.textContent = ch.icon + ' ' + ch.name;
                    if (ch.id === currentChannel) opt.selected = true;
                    channelSelect.appendChild(opt);
                }});
                
                document.getElementById('channel-loading').textContent = data.channels.length + ' channels available';
                document.getElementById('channel-loading').className = 'text-xs text-green-500 mt-1';
                
                data.roles.sort((a, b) => b.position - a.position).forEach(r => {{
                    const opt = document.createElement('option');
                    opt.value = r.id;
                    opt.textContent = r.name;
                    opt.style.color = r.color;
                    if (r.id === currentRole) opt.selected = true;
                    roleSelect.appendChild(opt);
                }});
                
                document.getElementById('role-loading').textContent = data.roles.length + ' roles available';
                document.getElementById('role-loading').className = 'text-xs text-green-500 mt-1';
            }} catch (err) {{
                document.getElementById('channel-loading').textContent = 'Failed to load channels';
                document.getElementById('role-loading').textContent = 'Failed to load roles';
            }}
        }})();
        </script>"#,
        if enabled { "checked" } else { "" },
        if check_bad_words { "checked" } else { "" },
        if check_bad_names { "checked" } else { "" },
        if check_nsfw { "checked" } else { "" },
        warn_threshold,
        if language == "en" { "selected" } else { "" },
        if language == "de" { "selected" } else { "" },
        if auto_mute { "checked" } else { "" },
        log_channel,
        mute_role
    );

    settings_page("Moderation", "moderation", &content)
}
