use macroquad::prelude::*;
use sokoban_state::SokobanState;
mod prelude {
    pub const SCREEN_WIDTH: i32 = 19;
    pub const SCREEN_HEIGHT: i32 = 17;
    pub const TILE_WIDTH: i32 = 32;
    pub const TILE_HEIGHT: i32 = 32;
    pub use bracket_geometry::prelude::*;
    pub use std::collections::HashMap;
}
use crate::prelude::*;
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
    //generate the texture atlas for the game
    let texture_atlas = make_texture_atlas().await;
    //initialize the gamestate from file as a start but will become a more sophisticated level loading system later
    let mut gamestate = SokobanState::from_file("levels/test.txt".to_string(), texture_atlas);
    loop {
        game_systems::run_systems(&mut gamestate);
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
