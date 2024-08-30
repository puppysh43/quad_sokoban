use crate::prelude::*;
use crate::sokoban_state::*;
use macroquad::prelude::*;

///Victory screen system that will render out a simple congratulations message and quit if the player hits enter
pub fn system(state: &mut SokobanState) {
    clear_background(BLACK);
    draw_text("Congratulations! You won!", 0.0, 20.0, 20.0, WHITE);
    draw_text("Press ENTER to quit", 0.0, 40.0, 20.0, WHITE);
    if is_key_down(KeyCode::Enter) {
        state.quitting = true;
    }
}
