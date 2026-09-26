use ratatui::style::Color;

pub const TITLE_COLOR: Color = Color::Rgb(255, 100, 50);   // Flame orange
pub const ACCENT_COLOR: Color = Color::Rgb(255, 160, 60);
pub const BORDER_COLOR: Color = Color::Rgb(80, 80, 100);
pub const ACTIVE_BORDER: Color = Color::Rgb(255, 140, 60);
pub const TEXT_COLOR: Color = Color::Gray;
pub const MUTED_COLOR: Color = Color::DarkGray;

pub const STATUS_SUCCESS: Color = Color::Green;   // 2xx
pub const STATUS_REDIRECT: Color = Color::Cyan;   // 3xx
pub const STATUS_CLIENT_ERR: Color = Color::Yellow; // 4xx
pub const STATUS_SERVER_ERR: Color = Color::Red;  // 5xx

pub const DEFAULT_TIMEOUT_SECS: u64 = 30;
pub const HISTORY_MAX: usize = 50;