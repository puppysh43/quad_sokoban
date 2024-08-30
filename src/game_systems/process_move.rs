use crate::prelude::*;
use crate::sokoban_state::*;
use macroquad::prelude::*;
pub fn system(state: &mut SokobanState, delta: IVec2) {
    //get player position
    let player_pos = state.player;
    //calculate the player's new position w/ the delta
    let new_player_pos = IVec2::new(player_pos.x + delta.x, player_pos.y + delta.y);
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
            let new_crate_pos = IVec2::new(new_player_pos.x + delta.x, new_player_pos.y + delta.y);
            if state.map.can_enter_tile(new_crate_pos) && !state.crates.contains_key(&new_crate_pos)
            {
                //if so move the player and adjust the crate position accordingly
                state.player = new_player_pos;
                state.crates.insert(
                    IVec2::new(new_player_pos.x + delta.x, new_player_pos.y + delta.y),
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
    //if the player was able to make a legitimate move then increment the movecount and capture the move made
    if has_moved {
        state.movecount += 1;
        state.moves.push(state.get_current_move());
    }
}
