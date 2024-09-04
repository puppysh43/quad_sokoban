use macroquad::prelude::*;
use sokoban_state::*;
mod prelude {
    pub const SCREEN_WIDTH: i32 = 19;
    pub const SCREEN_HEIGHT: i32 = 17;
    pub const TILE_WIDTH: i32 = 32;
    pub const TILE_HEIGHT: i32 = 32;
    pub use std::collections::HashMap;
}
use crate::app_state::*;
use crate::prelude::*;
mod app_state;
mod app_systems;
mod editor_state;
mod game_systems;
mod map;
mod sokoban_state;
mod victory_screen;

fn window_conf() -> Conf {
    Conf {
        window_title: "Sokoban Soup".to_string(),
        window_width: 608,
        window_height: 544,
        high_dpi: false,
        fullscreen: false,
        sample_count: 1,
        window_resizable: false,
        icon: None,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    //make the necessary appstate instance to launch the program.
    //currently takes a fed number will later read it from save data

    //first check if there's a save file in the save folder. if so load it if not then generate a new
    //appstate with a max level of 1

    let mut app_state = AppState::new(2);
    //generate the texture atlas for the game
    let texture_atlas = make_texture_atlas().await;
    //initialize a base game state using a default. the actual level data will be saved later
    let mut gamestate = SokobanState::from_file("levels/test.txt".to_string(), texture_atlas);
    //temp line to test campaign
    gamestate.update_from_file("levels/campaign/1.txt".to_string());
    loop {
        match app_state.app_mode {
            AppMode::Menu(_) => {
                app_systems::run_systems(&mut app_state, &mut gamestate);
            }
            AppMode::Sokoban => {
                loop {
                    match gamestate.game_state {
                        GameState::Playing => {
                            game_systems::run_systems(&mut gamestate);
                        }
                        GameState::Quitting => {
                            app_state.app_mode = AppMode::Menu(MenuMode::Root);
                            break;
                        }
                        GameState::Continuing => {
                            break;
                        }
                        GameState::Won => {
                            victory_screen::system(&mut gamestate);
                        }
                    }
                    next_frame().await
                }
                if gamestate.game_state == GameState::Continuing
                    && app_state.current_campaign_level < 50
                {
                    app_state.current_campaign_level += 1;
                    gamestate.game_state = GameState::Playing;
                    //function to update the game's map information to the current campaign map
                    load_campaign_level(&mut gamestate, app_state.current_campaign_level);
                }
            }
            AppMode::Editor => {
                //nothing
            }
        }
        //if the player has won and not quit increment the max level and reset the winning status
        //this loop break lets the user quit
        if app_state.quitting {
            //if the player is quitting then save their maxlevel
            break;
        }
        next_frame().await
    }
}

async fn make_texture_atlas() -> HashMap<String, Texture2D> {
    let boxcrate: Texture2D = load_texture("resources/box.png")
        .await
        .expect("Failed to load texture.");
    let boxspot: Texture2D = load_texture("resources/box_spot.png")
        .await
        .expect("Failed to load texture.");
    let floor: Texture2D = load_texture("resources/floor.png")
        .await
        .expect("Failed to load texture.");
    let player: Texture2D = load_texture("resources/player.png")
        .await
        .expect("Failed to load texture.");
    let wall: Texture2D = load_texture("resources/wall.png")
        .await
        .expect("Failed to load texture.");
    let texture_atlas = HashMap::from([
        (String::from("crate"), boxcrate),
        (String::from("cratespot"), boxspot),
        (String::from("floor"), floor),
        (String::from("player"), player),
        (String::from("wall"), wall),
    ]);
    build_textures_atlas();
    return texture_atlas;
}
