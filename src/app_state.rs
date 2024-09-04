use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq)]
///State enum to track what aspect of the complete window/app is being currently run
pub enum AppMode {
    ///Actively in a sokoban game
    Sokoban,
    ///In the main menu deciding whether to play or edit and the needed parameters
    Menu(MenuMode),
    ///Using the level editor to edit a specific map
    Editor,
}
#[derive(Copy, Clone, Debug, PartialEq)]
///State enum to track where in the menu the user is while not playing or editing a level
pub enum MenuMode {
    ///The root screen where the player can choose to play the campaign, play a custom level,
    ///edit a level, or quit.
    Root,
    ///The screen where the player can select which level of the campaign they want to play
    ///value contained inside is the currently selected level.
    Campaign(i32),
    ///The screen where the player can select which custom level they want to play
    CustomLevel,
    ///The screen where the user can set the parameters of using the level editor
    EditorMenu,
}
pub struct AppState {
    pub app_mode: AppMode,
    pub quitting: bool,
    pub current_campaign_level: i32,
    pub max_campaign_level: i32,
    pub custom_level: Option<String>,
    //something to keep track of what editor is being selected to load into the editor
}
impl AppState {
    pub fn new(max_campaign_level: i32) -> Self {
        AppState {
            app_mode: AppMode::Menu(MenuMode::Root),
            quitting: false,
            current_campaign_level: 1,
            max_campaign_level,
            custom_level: None,
        }
    }
}

///This is the struct that will be serialized and deserialized to track whatever is needed
///(as of right now simply the highest campaign level reached)
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Save {
    max_level: i32,
}
impl Save {
    pub fn max_level(&self) -> i32 {
        self.max_level
    }
    pub fn new(max_level: i32) -> Self {
        Self { max_level }
    }
}
