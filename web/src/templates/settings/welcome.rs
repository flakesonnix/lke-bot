use crate::templates::settings::settings_page;
use axum::response::Html;
use shared::WelcomeSettings;

pub fn welcome_settings(settings: Option<WelcomeSettings>) -> Html<String> {
    let s = settings.as_ref();
    let welcome_enabled = s.map(|s| s.welcome_enabled).unwrap_or(false);
    let welcome_channel = s
        .and_then(|s| s.welcome_channel_id.as_deref())
        .unwrap_or("");
    let welcome_message = s
        .map(|s| s.welcome_message.as_str())
        .unwrap_or("Welcome to the server, {user}!");
    let welcome_dm = s.map(|s| s.welcome_dm).unwrap_or(false);
    let goodbye_enabled = s.map(|s| s.goodbye_enabled).unwrap_or(false);
    let goodbye_channel = s
        .and_then(|s| s.goodbye_channel_id.as_deref())
        .unwrap_or("");
    let goodbye_message = s
        .map(|s| s.goodbye_message.as_str())
        .unwrap_or("Goodbye, {user.name}!");
    let auto_role = s.and_then(|s| s.auto_role_id.as_deref()).unwrap_or("");

    let content = format!(
        r#"<div class="space-y-8">
            <div class="bg-gray-800 rounded-lg p-6">
                <h3 class="text-lg font-semibold mb-4 flex items-center gap-2">
                    <span>👋</span> Welcome Messages
                </h3>
                
                <form id="welcome-settings-form" class="space-y-6">
                    <div class="flex items-center justify-between p-4 bg-gray-700 rounded-lg">
                        <div>
                            <h4 class="font-medium">Enable Welcome Messages</h4>
                            <p class="text-sm text-gray-400">Send a message when members join</p>
                        </div>
                        <label class="relative inline-flex items-center cursor-pointer">
                            <input type="checkbox" name="welcome_enabled" {} class="sr-only peer">
                            <div class="w-11 h-6 bg-gray-600 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-indigo-600"></div>
                        </label>
                    </div>

                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                        <div>
                            <label class="block text-gray-400 mb-2">Welcome Channel</label>
                            <select name="welcome_channel_id" id="welcome-channel-select" 
                                    class="w-full bg-gray-700 rounded-lg p-3 text-white">
                                <option value="">-- Select a channel --</option>
                            </select>
                            <p class="text-xs text-gray-500 mt-1" id="welcome-channel-loading">Loading channels...</p>
                        </div>

                        <div>
                            <label class="block text-gray-400 mb-2">Auto Role</label>
                            <select name="auto_role_id" id="auto-role-select" 
                                    class="w-full bg-gray-700 rounded-lg p-3 text-white">
                                <option value="">-- No auto role --</option>
                            </select>
                            <p class="text-xs text-gray-500 mt-1" id="role-loading">Loading roles...</p>
                        </div>
                    </div>

                    <div>
                        <label class="block text-gray-400 mb-2">Welcome Message</label>
                        <textarea name="welcome_message" rows="3"
                                  class="w-full bg-gray-700 rounded-lg p-3 text-white"
                                  placeholder="Welcome to the server, {{user}}!">{}</textarea>
                        <p class="text-xs text-gray-500 mt-1">
                            Placeholders: &#123;user&#125;, &#123;user.name&#125;, &#123;server&#125;, &#123;server.member_count&#125;
                        </p>
                    </div>

                    <div class="flex items-center gap-3 p-3 bg-gray-700 rounded-lg">
                        <input type="checkbox" name="welcome_dm" id="welcome-dm" {} class="w-5 h-5 rounded">
                        <label for="welcome-dm" class="cursor-pointer">
                            <span class="font-medium">Also send via DM</span>
                            <p class="text-sm text-gray-400">Send the welcome message as a direct message</p>
                        </label>
                    </div>
                </form>
            </div>

            <div class="bg-gray-800 rounded-lg p-6">
                <h3 class="text-lg font-semibold mb-4 flex items-center gap-2">
                    <span>👋</span> Goodbye Messages
                </h3>
                
                <form id="goodbye-settings-form" class="space-y-6">
                    <div class="flex items-center justify-between p-4 bg-gray-700 rounded-lg">
                        <div>
                            <h4 class="font-medium">Enable Goodbye Messages</h4>
                            <p class="text-sm text-gray-400">Send a message when members leave</p>
                        </div>
                        <label class="relative inline-flex items-center cursor-pointer">
                            <input type="checkbox" name="goodbye_enabled" {} class="sr-only peer">
                            <div class="w-11 h-6 bg-gray-600 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-indigo-600"></div>
                        </label>
                    </div>

                    <div>
                        <label class="block text-gray-400 mb-2">Goodbye Channel</label>
                        <select name="goodbye_channel_id" id="goodbye-channel-select" 
                                class="w-full bg-gray-700 rounded-lg p-3 text-white">
                            <option value="">-- Select a channel --</option>
                        </select>
                        <p class="text-xs text-gray-500 mt-1" id="goodbye-channel-loading">Loading channels...</p>
                    </div>

                    <div>
                        <label class="block text-gray-400 mb-2">Goodbye Message</label>
                        <textarea name="goodbye_message" rows="3"
                                  class="w-full bg-gray-700 rounded-lg p-3 text-white"
                                  placeholder="Goodbye, {{user.name}}!">{}</textarea>
                        <p class="text-xs text-gray-500 mt-1">
                            Placeholders: &#123;user.name&#125;, &#123;server&#125;, &#123;server.member_count&#125;
                        </p>
                    </div>
                </form>
            </div>

            <div id="result" class="hidden p-4 rounded-lg"></div>

            <button id="save-btn" class="w-full bg-indigo-600 hover:bg-indigo-700 px-6 py-3 rounded-lg font-medium transition">
                Save Welcome Settings
            </button>
        </div>

        <script>
        (async () => {{
            try {{
                const res = await fetch('/api/guild/resources');
                const data = await res.json();
                
                const welcomeSelect = document.getElementById('welcome-channel-select');
                const goodbyeSelect = document.getElementById('goodbye-channel-select');
                const roleSelect = document.getElementById('auto-role-select');
                
                const currentWelcome = "{}";
                const currentGoodbye = "{}";
                const currentRole = "{}";
                
                const addChannelOptions = (select, current) => {{
                    data.channels.forEach(ch => {{
                        const opt = document.createElement('option');
                        opt.value = ch.id;
                        opt.textContent = ch.icon + ' ' + ch.name;
                        if (ch.id === current) opt.selected = true;
                        select.appendChild(opt);
                    }});
                }};
                
                addChannelOptions(welcomeSelect, currentWelcome);
                addChannelOptions(goodbyeSelect, currentGoodbye);
                
                document.getElementById('welcome-channel-loading').textContent = data.channels.length + ' channels';
                document.getElementById('goodbye-channel-loading').textContent = data.channels.length + ' channels';
                
                data.roles.sort((a, b) => b.position - a.position).forEach(r => {{
                    const opt = document.createElement('option');
                    opt.value = r.id;
                    opt.textContent = r.name;
                    opt.style.color = r.color;
                    if (r.id === currentRole) opt.selected = true;
                    roleSelect.appendChild(opt);
                }});
                
                document.getElementById('role-loading').textContent = data.roles.length + ' roles';
            }} catch (err) {{
                document.getElementById('welcome-channel-loading').textContent = 'Failed to load';
                document.getElementById('goodbye-channel-loading').textContent = 'Failed to load';
                document.getElementById('role-loading').textContent = 'Failed to load';
            }}
        }})();

        document.getElementById('save-btn').addEventListener('click', async () => {{
            const welcomeForm = document.getElementById('welcome-settings-form');
            const goodbyeForm = document.getElementById('goodbye-settings-form');
            
            const data = {{
                welcome_enabled: welcomeForm.welcome_enabled.checked,
                welcome_channel_id: welcomeForm.welcome_channel_id.value || null,
                welcome_message: welcomeForm.welcome_message.value,
                welcome_dm: welcomeForm.welcome_dm.checked,
                goodbye_enabled: goodbyeForm.goodbye_enabled.checked,
                goodbye_channel_id: goodbyeForm.goodbye_channel_id.value || null,
                goodbye_message: goodbyeForm.goodbye_message.value,
                auto_role_id: welcomeForm.auto_role_id.value || null
            }};
            
            const result = document.getElementById('result');
            result.classList.remove('hidden');
            result.className = 'p-4 rounded-lg bg-blue-600/20 text-blue-400';
            result.textContent = 'Saving...';
            
            try {{
                const res = await fetch('/api/welcome/settings', {{
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
        if welcome_enabled { "checked" } else { "" },
        welcome_message,
        if welcome_dm { "checked" } else { "" },
        if goodbye_enabled { "checked" } else { "" },
        goodbye_message,
        welcome_channel,
        goodbye_channel,
        auto_role
    );

    settings_page("Welcome & Goodbye", "welcome", &content)
}
