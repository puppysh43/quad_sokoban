use crate::map::*;
use crate::prelude::*;
use crate::sokoban_state::*;
use macroquad::prelude::*;

mod input;
mod process_move;
mod render;

pub enum MessageOfIntent {
    None,
    MovePlayer(IVec2), //point is a delta not the exact location
    Quit,
    Reset,
    Rewind,
    Forward, //future feature
}
pub fn run_systems(state: &mut SokobanState) {
    //If the player hasn't won the game yet run all the game systems until they do win
    // if state.game_state == GameState::Playing {
    //if the game is running for the first time take a snapshot of the current state for the rewind feature aka move 0
    if state.moves.is_empty() {
        let first_move = state.get_current_move();
        state.moves.push(first_move);
    }
    //get player input as a message of intent
    let moi = input::system();
    //process message of intent and do the corresponding actions
    match moi {
        MessageOfIntent::None => do_nothing(state),
        MessageOfIntent::MovePlayer(delta) => process_move::system(state, delta),
        MessageOfIntent::Quit => quit_game(state),
        MessageOfIntent::Reset => reset_level(state),
        MessageOfIntent::Rewind => rewind(state),
        MessageOfIntent::Forward => forward(state), //currently does nothing
    }
    render::system(state);
    //check gamestate for victory condition if so do victory state
    check_victory(state);
    // } else {
    //if the player has won show the victory screen and let them press enter to quit
    // victory_screen::system(state);
    // }
}
fn do_nothing(state: &mut SokobanState) {
    //do nothing
}
fn quit_game(state: &mut SokobanState) {
    state.game_state = GameState::Quitting;
}
///resets the level by reverting it to the state of the original move captured on level startup
fn reset_level(state: &mut SokobanState) {
    let first_move = state.moves[0].clone();
    state.player = first_move.player;
    state.crates = first_move.crates;
    state.movecount = 0;
    state.moves.clear();
}
///this is an unsophisticated rewind function that obliterates the move as it restores it - meaning players cannot
///freely scroll through all of their moves before deciding where to change course. This works but needs to be changed
fn rewind(state: &mut SokobanState) {
    let previous_move = state.moves.pop().clone().unwrap();
    state.player = previous_move.player;
    state.crates = previous_move.crates;
    state.movecount = previous_move.movecount;
}

fn forward(state: &mut SokobanState) {
    //this will move back forward through list of moves
}

fn check_victory(state: &mut SokobanState) {
    //iterate through all crates and check if all current positions correspond to a loading dock tile
    let mut has_won = true;
    for (pos, _crate) in state.crates.iter() {
        if state.map.tiles[map_idx(pos.x, pos.y)] != TileType::LoadingSquare {
            has_won = false;
        }
    }
    if has_won {
        state.game_state = GameState::Won;
    }
}
