use crate::templates::settings::settings_page;
use axum::response::Html;
use shared::TicketSettings;

pub fn ticket_settings(settings: Option<TicketSettings>) -> Html<String> {
    let s = settings.as_ref();
    let enabled = s.map(|s| s.enabled).unwrap_or(false);
    let category_id = s.and_then(|s| s.category_id.as_deref()).unwrap_or("");
    let support_role_id = s.and_then(|s| s.support_role_id.as_deref()).unwrap_or("");
    let log_channel_id = s.and_then(|s| s.log_channel_id.as_deref()).unwrap_or("");
    let max_days = s.map(|s| s.max_open_days).unwrap_or(3);

    let content = format!(
        r#"<form id="ticket-settings-form" class="space-y-6">
            <div class="flex items-center justify-between p-4 bg-gray-700 rounded-lg">
                <div>
                    <h3 class="font-medium">Enable Ticket System</h3>
                    <p class="text-sm text-gray-400">Allow users to create support tickets</p>
                </div>
                <label class="relative inline-flex items-center cursor-pointer">
                    <input type="checkbox" name="enabled" {} class="sr-only peer">
                    <div class="w-11 h-6 bg-gray-600 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-indigo-600"></div>
                </label>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div>
                    <label class="block text-gray-400 mb-2">Log Channel</label>
                    <select name="log_channel_id" id="log-channel-select" 
                            class="w-full bg-gray-700 rounded-lg p-3 text-white">
                        <option value="">-- Select a channel --</option>
                    </select>
                    <p class="text-xs text-gray-500 mt-1" id="log-channel-loading">Loading channels...</p>
                </div>

                <div>
                    <label class="block text-gray-400 mb-2">Support Role</label>
                    <select name="support_role_id" id="support-role-select" 
                            class="w-full bg-gray-700 rounded-lg p-3 text-white">
                        <option value="">-- Select a role --</option>
                    </select>
                    <p class="text-xs text-gray-500 mt-1" id="role-loading">Loading roles...</p>
                </div>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div>
                    <label class="block text-gray-400 mb-2">Category ID</label>
                    <input type="text" name="category_id" value="{}" 
                           class="w-full bg-gray-700 rounded-lg p-3 text-white" 
                           placeholder="Channel category for tickets">
                    <p class="text-xs text-gray-500 mt-1">Tickets will be created in this category</p>
                </div>

                <div>
                    <label class="block text-gray-400 mb-2">Max Open Days</label>
                    <input type="number" name="max_open_days" value="{}" min="1" max="30"
                           class="w-full bg-gray-700 rounded-lg p-3 text-white">
                    <p class="text-xs text-gray-500 mt-1">Auto-close tickets after this many days</p>
                </div>
            </div>

            <div class="p-4 bg-gray-700/50 rounded-lg">
                <h4 class="font-medium mb-2">How it works</h4>
                <ul class="text-sm text-gray-400 space-y-1">
                    <li>• Users click a button to create a ticket</li>
                    <li>• A private channel is created in the category</li>
                    <li>• Support role gets access to help the user</li>
                    <li>• Tickets auto-close after max days or when resolved</li>
                    <li>• Transcript is saved and logged</li>
                </ul>
            </div>

            <div id="result" class="hidden p-4 rounded-lg"></div>

            <button type="submit" class="w-full bg-indigo-600 hover:bg-indigo-700 px-6 py-3 rounded-lg font-medium transition">
                Save Ticket Settings
            </button>
        </form>

        <script>
        (async () => {{
            try {{
                const res = await fetch('/api/guild/resources');
                const data = await res.json();
                
                const channelSelect = document.getElementById('log-channel-select');
                const roleSelect = document.getElementById('support-role-select');
                const currentChannel = "{}";
                const currentRole = "{}";
                
                data.channels.forEach(ch => {{
                    const opt = document.createElement('option');
                    opt.value = ch.id;
                    opt.textContent = ch.icon + ' ' + ch.name;
                    if (ch.id === currentChannel) opt.selected = true;
                    channelSelect.appendChild(opt);
                }});
                
                document.getElementById('log-channel-loading').textContent = data.channels.length + ' channels available';
                document.getElementById('log-channel-loading').className = 'text-xs text-green-500 mt-1';
                
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
                document.getElementById('log-channel-loading').textContent = 'Failed to load channels';
                document.getElementById('role-loading').textContent = 'Failed to load roles';
            }}
        }})();
        </script>"#,
        if enabled { "checked" } else { "" },
        category_id,
        max_days,
        log_channel_id,
        support_role_id
    );

    settings_page("Ticket System", "tickets", &content)
}
