use crate::app_state::*;
use crate::prelude::*;
use crate::sokoban_state::*;
use macroquad::prelude::*;
use macroquad::ui::*;

pub fn run_systems(app_state: &mut AppState, sokoban_state: &mut SokobanState) {
    //peepeepoopoo
    //I guess instead of having a bunch of iterative systems it'll be one big stupid UI thing.
    match app_state.app_mode {
        AppMode::Menu(menu_mode) => {
            match menu_mode {
                //first screen the app opens to
                MenuMode::Root => {
                    //button to take the player to the player to the campaign screen
                    if root_ui().button(Vec2::new(50.0, 50.0), String::from("Play Campaign")) {
                        //go to the campaign screen with the max level as the currently selected level already
                        app_state.app_mode =
                            AppMode::Menu(MenuMode::Campaign(app_state.max_campaign_level));
                    }
                    if root_ui().button(Vec2::new(50.0, 100.0), String::from("Launch Editor")) {
                        //go to the editor launch screen
                        app_state.app_mode = AppMode::Menu(MenuMode::EditorMenu);
                    }
                    if root_ui().button(Vec2::new(50.0, 150.0), "Quit Game") {
                        app_state.quitting = true;
                    }
                }
                MenuMode::Campaign(current_level) => {
                    //Button that launches the campaign level currently selected
                    if root_ui().button(Vec2::new(50.0, 50.0), String::from("Play Selected Level"))
                    {
                        //load the level currently selected within the ui
                        load_campaign_level(sokoban_state, current_level);
                        //set the current level var to the one selected in the UI so that it can be incremented and tracked properly once the player is in game
                        app_state.current_campaign_level = current_level;
                        //finally don't forget to set the appmode to sokoban so it'll actually start the game!
                        app_state.app_mode = AppMode::Sokoban;
                    }
                    root_ui().label(Vec2::new(50.0, 70.0), "Select Campaign Level");
                    //button that scrolls back through campaign level selection
                    if root_ui().button(Vec2::new(50.0, 90.0), String::from("<")) {
                        if current_level > 1 {
                            app_state.app_mode =
                                AppMode::Menu(MenuMode::Campaign(current_level - 1));
                        }
                    }
                    draw_text(
                        format!("{}", current_level).as_str(),
                        70.0,
                        90.0,
                        20.0,
                        YELLOW,
                    );
                    //button that scrolls forward through the available levels
                    if root_ui().button(Vec2::new(90.0, 90.0), ">") {
                        if current_level < app_state.max_campaign_level {
                            app_state.app_mode =
                                AppMode::Menu(MenuMode::Campaign(current_level + 1))
                        }
                    }
                }
                MenuMode::CustomLevel => {
                    //
                }
                MenuMode::EditorMenu => {
                    //for now just have a button to actually launch the editor state
                    if root_ui().button(Vec2::new(50.0, 50.0), String::from("Launch Level Editor"))
                    {
                        app_state.app_mode = AppMode::Editor;
                    }
                }
            }
        }
        _ => {
            //do nothing b/c this is impossible
        }
    }
}
