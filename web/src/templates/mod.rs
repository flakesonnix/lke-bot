mod dashboard;
mod index;
pub mod settings;

pub use dashboard::dashboard;
pub use index::index;

pub fn base_html(title: &str, content: &str, logged_in: bool) -> String {
    let nav_links = if logged_in {
        r#"<div class="flex gap-4">
                <a href="/dashboard" class="text-gray-300 hover:text-white">Dashboard</a>
                <a href="/logout" class="text-gray-300 hover:text-white">Logout</a>
            </div>"#
    } else {
        r#"<div class="flex gap-4">
                <a href="/auth/discord" class="bg-indigo-600 hover:bg-indigo-700 text-white font-semibold py-2 px-4 rounded-lg transition-colors">Login</a>
            </div>"#
    };

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title}</title>
    <script src="https://cdn.tailwindcss.com"></script>
</head>
<body class="bg-gray-900 text-gray-100 min-h-screen">
    <nav class="bg-gray-800 border-b border-gray-700">
        <div class="max-w-7xl mx-auto px-4 py-3 flex justify-between items-center">
            <a href="/" class="text-xl font-bold text-indigo-400">LKE Bot</a>
            {nav_links}
        </div>
    </nav>
    <main class="max-w-7xl mx-auto px-4 py-8">
        {content}
    </main>
</body>
</html>"#
    )
}
