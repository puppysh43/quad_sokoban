use std::fmt::Display;

use macroquad::prelude::*;
use sokoban_state::SokobanState;
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
mod game_systems;
mod map;
mod sokoban_state;

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
    let mut app_state = AppState::new();
    //generate the texture atlas for the game
    let texture_atlas = make_texture_atlas().await;
    //initialize a base game state using a default. the actual level data will be saved later
    let mut gamestate = SokobanState::from_file("levels/test.txt".to_string(), texture_atlas);
    //temp line to test campaign
    gamestate.update_from_file("levels/campaign/1.txt".to_string());
    loop {
        if app_state.app_mode == AppMode::Menu {
            app_systems::run_systems(&mut app_state, &mut gamestate);
        }
        while app_state.app_mode == AppMode::Sokoban {
            game_systems::run_systems(&mut gamestate);
            if gamestate.quitting {
                app_state.app_mode = AppMode::Menu;
                break;
            }
            next_frame().await
        }
        //if the player has won and not quit increment the max level and reset the winning status
        if gamestate.has_won && app_state.current_level < 50 {
            app_state.current_level += 1;
            gamestate.has_won = false;
            //function to update the game's map information to the current campaign map
            load_campaign_level(&mut gamestate, app_state.current_level);
        }
        //this loop break lets the user quit
        if app_state.quitting {
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

///takes the current level and
fn load_campaign_level(gamestate: &mut SokobanState, current_level: i32) {
    //maybe instead of plain numbers it will have "level_" in front of it but for now it's just numbers
    let path = format!("levels/campaign/{current_level}.txt");
    gamestate.update_from_file(path);
}
