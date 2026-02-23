use crate::templates::settings::settings_page;
use axum::response::Html;
use shared::TtsSettings;

pub fn tts_settings(settings: Option<TtsSettings>) -> Html<String> {
    let s = settings.as_ref();
    let enabled = s.map(|s| s.enabled).unwrap_or(false);
    let voice = s.map(|s| s.default_voice.as_str()).unwrap_or("en-US");
    let language = s.map(|s| s.default_language.as_str()).unwrap_or("en");
    let speed = s.map(|s| s.speed).unwrap_or(1.0);
    let pitch = s.map(|s| s.pitch).unwrap_or(1.0);
    let volume = s.map(|s| s.volume).unwrap_or(1.0);

    let content = format!(
        r#"<form id="tts-settings-form" class="space-y-6">
            <div class="flex items-center justify-between p-4 bg-gray-700 rounded-lg">
                <div>
                    <h3 class="font-medium">Enable TTS</h3>
                    <p class="text-sm text-gray-400">Read voice chat messages aloud</p>
                </div>
                <label class="relative inline-flex items-center cursor-pointer">
                    <input type="checkbox" name="enabled" {} class="sr-only peer">
                    <div class="w-11 h-6 bg-gray-600 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-indigo-600"></div>
                </label>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div>
                    <label class="block text-gray-400 mb-2">Default Voice</label>
                    <select name="default_voice" class="w-full bg-gray-700 rounded-lg p-3 text-white">
                        <option value="en-US" {}>English (US)</option>
                        <option value="en-GB" {}>English (UK)</option>
                        <option value="de-DE" {}>German</option>
                        <option value="fr-FR" {}>French</option>
                        <option value="es-ES" {}>Spanish</option>
                        <option value="it-IT" {}>Italian</option>
                        <option value="ja-JP" {}>Japanese</option>
                        <option value="ko-KR" {}>Korean</option>
                        <option value="zh-CN" {}>Chinese</option>
                    </select>
                </div>
                <div>
                    <label class="block text-gray-400 mb-2">Language</label>
                    <select name="default_language" class="w-full bg-gray-700 rounded-lg p-3 text-white">
                        <option value="en" {}>English</option>
                        <option value="de" {}>German</option>
                        <option value="fr" {}>French</option>
                        <option value="es" {}>Spanish</option>
                        <option value="it" {}>Italian</option>
                        <option value="ja" {}>Japanese</option>
                    </select>
                </div>
            </div>

            <div class="space-y-4">
                <div>
                    <label class="block text-gray-400 mb-2">Speed: {:.1}x</label>
                    <input type="range" name="speed" value="{:.1}" min="0.5" max="2" step="0.1" 
                           class="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer">
                    <div class="flex justify-between text-xs text-gray-500 mt-1">
                        <span>0.5x (Slow)</span>
                        <span>1.0x (Normal)</span>
                        <span>2.0x (Fast)</span>
                    </div>
                </div>

                <div>
                    <label class="block text-gray-400 mb-2">Pitch: {:.1}</label>
                    <input type="range" name="pitch" value="{:.1}" min="0.5" max="2" step="0.1" 
                           class="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer">
                    <div class="flex justify-between text-xs text-gray-500 mt-1">
                        <span>Low</span>
                        <span>Normal</span>
                        <span>High</span>
                    </div>
                </div>

                <div>
                    <label class="block text-gray-400 mb-2">Volume: {:.0}%</label>
                    <input type="range" name="volume" value="{:.1}" min="0" max="1" step="0.1" 
                           class="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer">
                    <div class="flex justify-between text-xs text-gray-500 mt-1">
                        <span>Mute</span>
                        <span>100%</span>
                    </div>
                </div>
            </div>

            <div class="p-4 bg-gray-700/50 rounded-lg">
                <h4 class="font-medium mb-2">Permissions</h4>
                <p class="text-sm text-gray-400">Configure who can use TTS via the /tts command or dashboard.</p>
            </div>

            <button type="submit" class="w-full bg-indigo-600 hover:bg-indigo-700 px-6 py-3 rounded-lg font-medium transition">
                Save TTS Settings
            </button>
        </form>"#,
        if enabled { "checked" } else { "" },
        if voice == "en-US" { "selected" } else { "" },
        if voice == "en-GB" { "selected" } else { "" },
        if voice == "de-DE" { "selected" } else { "" },
        if voice == "fr-FR" { "selected" } else { "" },
        if voice == "es-ES" { "selected" } else { "" },
        if voice == "it-IT" { "selected" } else { "" },
        if voice == "ja-JP" { "selected" } else { "" },
        if voice == "ko-KR" { "selected" } else { "" },
        if voice == "zh-CN" { "selected" } else { "" },
        if language == "en" { "selected" } else { "" },
        if language == "de" { "selected" } else { "" },
        if language == "fr" { "selected" } else { "" },
        if language == "es" { "selected" } else { "" },
        if language == "it" { "selected" } else { "" },
        if language == "ja" { "selected" } else { "" },
        speed,
        speed,
        pitch,
        pitch,
        volume,
        volume
    );

    settings_page("Text-to-Speech", "tts", &content)
}
