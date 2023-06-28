use crate::VERSION;

pub fn get_window_title(title: &str) -> String {
    return format!("[FutureOS v{} – {}]", VERSION, title);
}

pub fn get_default_title() -> String {
    return format!("[FutureOS v{}]", VERSION);
}