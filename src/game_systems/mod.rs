use crate::map::*;
use crate::prelude::*;
use crate::sokoban_state::*;
use macroquad::prelude::*;
mod input;
pub enum MessageOfIntent {
    None,
    MovePlayer(Point), //point is a delta not the exact location
    Quit,
    // RestartLevel,
    // Rewind,
    // Forward,
}
pub fn run_systems(state: &mut SokobanState) {
    //get player input as a message of intent
    let moi = input::player_input();
    //process message of intent
    match moi {
        MessageOfIntent::None => do_nothing(state),
        MessageOfIntent::MovePlayer(delta) => process_move(state, delta),
        MessageOfIntent::Quit => quit_game(state),
    }
    render(state);

    //check gamestate for victory condition if so do victory state
}
fn do_nothing(state: &mut SokobanState) {
    //do nothing
}
fn process_move(state: &mut SokobanState, delta: Point) {
    //get player position
    let player_pos = state.player;
    //calculate the player's new position w/ the delta
    let new_player_pos = Point::new(player_pos.x + delta.x, player_pos.y + delta.y);
    //make a variable to be filled once it checks if the move collides with any crates.
    let mut moving_crate: Option<Crate> = None;
    //make a temp variable to check if the player has moved
    let mut has_moved = false;
    //if there is a crate in the spot the player is moving to mark it as being present
    if state.crates.contains_key(&new_player_pos) {
        moving_crate = Some(state.crates.remove(&new_player_pos).unwrap());
    }
    //if there is a crate where the player is apply the delta to the crate position and see if
    //the crate can also safely move to where it would go
    if moving_crate.is_some() {
        if state.map.can_enter_tile(new_player_pos) {
            let new_crate_pos = Point::new(new_player_pos.x + delta.x, new_player_pos.y + delta.y);
            println!(
                "new_player_pos: x-{} y-{}, new_crate_pos: x-{} y-{}",
                new_player_pos.x, new_player_pos.y, new_crate_pos.x, new_crate_pos.y
            );
            if state.map.can_enter_tile(new_crate_pos) && !state.crates.contains_key(&new_crate_pos)
            {
                println!(
                    "can_enter_tile = {}",
                    state.map.can_enter_tile(new_crate_pos)
                );
                //if so move the player and adjust the crate position accordingly
                state.player = new_player_pos;
                state.crates.insert(
                    Point::new(new_player_pos.x + delta.x, new_player_pos.y + delta.y),
                    moving_crate.unwrap(),
                );
                has_moved = true;
            } else {
                state.crates.insert(new_player_pos, moving_crate.unwrap());
                //do not move the player at all
            }
        }
        //if there is no crate being moved just make sure the player can move and move them!
    } else if state.map.can_enter_tile(new_player_pos) && moving_crate.is_none() {
        state.player = new_player_pos;
        has_moved = true;
    }
    //then take a snapshot of the state as if
    if has_moved {
        let move_made = Move::new(state.player, state.crates.clone(), state.movecount);
        state.moves.push(move_made);
    }
}
fn quit_game(state: &mut SokobanState) {
    state.quitting = true;
}
fn render(state: &mut SokobanState) {
    clear_background(WHITE);
    //rendering function one that needs to be MOST overhauled
    //first render the game map
    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            let pt = Point::new(x, y);
            let idx = map_idx(x, y);
            if state.map.in_bounds(pt) {
                match state.map.tiles[idx] {
                    TileType::Wall => {
                        draw_texture(
                            state.texture_atlas.get("wall").unwrap(),
                            (x * TILE_WIDTH) as f32,
                            (y * TILE_HEIGHT) as f32,
                            WHITE,
                        );
                    }
                    TileType::Floor => {
                        draw_texture(
                            state.texture_atlas.get("floor").unwrap(),
                            (x * TILE_WIDTH) as f32,
                            (y * TILE_HEIGHT) as f32,
                            WHITE,
                        );
                    }
                    TileType::LoadingSquare => {
                        draw_texture(
                            state.texture_atlas.get("cratespot").unwrap(),
                            (x * TILE_WIDTH) as f32,
                            (y * TILE_HEIGHT) as f32,
                            WHITE,
                        );
                    }
                }
            }
        }
    }
    //then render the crates
    for crate_pos in state.crates.keys() {
        draw_texture(
            state.texture_atlas.get("crate").unwrap(),
            (crate_pos.x * TILE_WIDTH) as f32,
            (crate_pos.y * TILE_HEIGHT) as f32,
            WHITE,
        );
    }
    //then render the player
    draw_texture(
        state.texture_atlas.get("player").unwrap(),
        (state.player.x * TILE_WIDTH) as f32,
        (state.player.y * TILE_HEIGHT) as f32,
        WHITE,
    );
}
