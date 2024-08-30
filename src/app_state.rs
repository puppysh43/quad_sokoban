#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AppMode {
    Sokoban,
    Menu,
    // Editor,
}
pub struct AppState {
    // is_playing: bool,
    pub app_mode: AppMode,
    pub quitting: bool,
    pub current_level: i32,
}
impl AppState {
    pub fn new() -> Self {
        AppState {
            app_mode: AppMode::Menu,
            quitting: false,
            current_level: 1,
        }
    }
}
