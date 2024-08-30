use crate::app_state::*;
use crate::prelude::*;
use crate::sokoban_state::*;
use macroquad::prelude::*;
use macroquad::ui::*;

pub fn run_systems(app_state: &mut AppState, sokoban_state: &mut SokobanState) {
    //peepeepoopoo
    app_state.app_mode = AppMode::Sokoban;
    //I guess instead of having a bunch of iterative systems it'll be one big stupid UI thing.
}
