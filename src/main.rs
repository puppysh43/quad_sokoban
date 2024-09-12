use macroquad::audio::*;
use macroquad::conf::UpdateTrigger;
use macroquad::prelude::*;
use sokoban_state::*;
mod prelude {
    pub const SCREEN_WIDTH: i32 = 19;
    pub const SCREEN_HEIGHT: i32 = 17;
    pub const TILE_WIDTH: i32 = 32;
    pub const TILE_HEIGHT: i32 = 32;
    pub use std::collections::HashMap;
    pub const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;
}
use crate::app_state::*;
use crate::prelude::*;
use editor_state::*;
use std::fs;
use std::path::Path;
mod app_state;
mod app_systems;
mod editor_state;
mod editor_systems;
mod game_systems;
mod map;
mod sokoban_state;
mod victory_screen;

fn window_conf() -> macroquad::conf::Conf {
    macroquad::conf::Conf {
        miniquad_conf: macroquad::prelude::Conf {
            window_title: "Sokoban Soup".to_string(),
            window_width: 608,
            window_height: 544,
            high_dpi: false,
            fullscreen: false,
            sample_count: 1,
            window_resizable: false,
            icon: None,
            ..Default::default()
        },
        update_on: Some(UpdateTrigger::default()),
        default_filter_mode: FilterMode::Nearest,
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    //make the necessary appstate instance to launch the program.
    let mut app_state = AppState::new(1);
    //if a save file exists then load it
    let save_file = fs::read_to_string("save/save.ron");
    // if fs::read("save/save.ron").is_ok() {
    //read the file and change the max level to the number extracted

    // }
    if save_file.is_ok() {
        let save: Save = ron::from_str(&save_file.unwrap()).unwrap();
        app_state.max_campaign_level = save.max_level();
    }

    //generate the texture atlas for the game
    let texture_atlas = make_texture_atlas().await;
    //generate the sound atlas for the game
    let sound_atlas = make_sound_atlas().await;
    //initialize a base game state using a default. the actual level data will be saved later
    let mut gamestate = SokobanState::from_file(
        "levels/test.txt".to_string(),
        texture_atlas.clone(),
        sound_atlas.clone(),
    );
    //initialize the editor state
    let mut editorstate = EditorState::new(texture_atlas.clone(), sound_atlas.clone());
    //temp line to test campaign
    // gamestate.update_from_file("levels/campaign/1.txt".to_string());
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
                editor_systems::run_systems(&mut editorstate);
                //nothing
            }
        }
        //if the player has won and not quit increment the max level and reset the winning status
        //this loop break lets the user quit
        if app_state.quitting {
            //if the player is quitting then save their maxlevel
            let save = Save::new(app_state.max_campaign_level);
            let buffer = ron::to_string(&save).unwrap();
            if fs::write(Path::new("save/save.ron"), buffer.clone()).is_ok() {
                fs::write(Path::new("save/save.ron"), buffer).unwrap();
            }
            break;
        }
        next_frame().await
    }
}

/*
fn save_world<P: AsRef<Path>>(path: P, world: World) -> Result<()> {
    let mut f = File::create(path)?;
    let buf = serde_json::to_vec(&world)?;
    f.write_all(&buf[..])?;
    Ok(())
}
*/

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
    let reticule: Texture2D = load_texture("resources/reticule.png")
        .await
        .expect("Failed to load texture.");

    let texture_atlas = HashMap::from([
        (String::from("crate"), boxcrate),
        (String::from("cratespot"), boxspot),
        (String::from("floor"), floor),
        (String::from("player"), player),
        (String::from("wall"), wall),
        (String::from("reticule"), reticule),
    ]);
    build_textures_atlas();
    return texture_atlas;
}

async fn make_sound_atlas() -> HashMap<String, Sound> {
    set_pc_assets_folder("resources");
    let wall_collision = load_sound("wall.wav").await.unwrap();
    let crate_in_spot = load_sound("correct.wav").await.unwrap();
    let sound_atlas = HashMap::from([
        (String::from("wall collision"), wall_collision),
        (String::from("crate in spot"), crate_in_spot),
    ]);
    return sound_atlas;
}
