use crate::templates::settings::settings_page;
use axum::response::Html;
use shared::MusicSettings;

pub fn music_settings(settings: Option<MusicSettings>) -> Html<String> {
    let s = settings.as_ref();
    let guest_mode = s.map(|s| s.guest_mode).unwrap_or(false);
    let stats_visible = s.map(|s| s.stats_visible).unwrap_or(true);
    let stats_for_guests = s.map(|s| s.stats_for_guests).unwrap_or(false);
    let max_queue = s.map(|s| s.max_queue_size).unwrap_or(100);
    let default_volume = s.map(|s| s.default_volume).unwrap_or(50);

    let content = format!(
        r#"<form id="music-settings-form" class="space-y-6">
            <div class="flex items-center justify-between p-4 bg-gray-700 rounded-lg">
                <div>
                    <h3 class="font-medium">Guest Mode</h3>
                    <p class="text-sm text-gray-400">Allow non-admins to control music</p>
                </div>
                <label class="relative inline-flex items-center cursor-pointer">
                    <input type="checkbox" name="guest_mode" {} class="sr-only peer">
                    <div class="w-11 h-6 bg-gray-600 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-indigo-600"></div>
                </label>
            </div>

            <div>
                <label class="block text-gray-400 mb-3">Stats Visibility</label>
                <div class="space-y-3">
                    <label class="flex items-center gap-3 p-3 bg-gray-700 rounded-lg cursor-pointer hover:bg-gray-600">
                        <input type="checkbox" name="stats_visible" {} class="w-5 h-5 rounded">
                        <div>
                            <span class="font-medium">Show Stats</span>
                            <p class="text-sm text-gray-400">Display music statistics on dashboard</p>
                        </div>
                    </label>
                    <label class="flex items-center gap-3 p-3 bg-gray-700 rounded-lg cursor-pointer hover:bg-gray-600">
                        <input type="checkbox" name="stats_for_guests" {} class="w-5 h-5 rounded">
                        <div>
                            <span class="font-medium">Stats for Guests</span>
                            <p class="text-sm text-gray-400">Allow guests to view music statistics</p>
                        </div>
                    </label>
                </div>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div>
                    <label class="block text-gray-400 mb-2">Max Queue Size</label>
                    <input type="number" name="max_queue_size" value="{}" min="1" max="500"
                           class="w-full bg-gray-700 rounded-lg p-3 text-white">
                    <p class="text-xs text-gray-500 mt-1">Maximum songs in queue</p>
                </div>
                <div>
                    <label class="block text-gray-400 mb-2">Default Volume: {}%</label>
                    <input type="range" name="default_volume" value="{}" min="0" max="100" 
                           class="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer mt-3">
                </div>
            </div>

            <div class="p-4 bg-gray-700/50 rounded-lg">
                <h4 class="font-medium mb-2">Supported Sources</h4>
                <div class="flex flex-wrap gap-2">
                    <span class="px-3 py-1 bg-orange-600/20 text-orange-400 rounded-full text-sm">SoundCloud</span>
                    <span class="px-3 py-1 bg-red-600/20 text-red-400 rounded-full text-sm">YouTube</span>
                    <span class="px-3 py-1 bg-blue-600/20 text-blue-400 rounded-full text-sm">Spotify</span>
                    <span class="px-3 py-1 bg-gray-600/50 text-gray-400 rounded-full text-sm">Direct URLs</span>
                </div>
            </div>

            <button type="submit" class="w-full bg-indigo-600 hover:bg-indigo-700 px-6 py-3 rounded-lg font-medium transition">
                Save Music Settings
            </button>
        </form>"#,
        if guest_mode { "checked" } else { "" },
        if stats_visible { "checked" } else { "" },
        if stats_for_guests { "checked" } else { "" },
        max_queue,
        default_volume,
        default_volume
    );

    settings_page("Music", "music", &content)
}
