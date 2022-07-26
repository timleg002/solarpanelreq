pub fn current_time() -> String {
    // YYYY-MM-DD HH:MM:SS
    chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
}