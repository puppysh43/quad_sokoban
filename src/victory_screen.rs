use crate::prelude::*;
use crate::sokoban_state::*;
use crate::AppState;
use macroquad::prelude::*;

///Victory screen system that will render out a simple congratulations message and quit if the player hits enter
pub fn system(state: &mut SokobanState, app_state: &mut AppState) {
    clear_background(BLACK);
    draw_text("Congratulations! You won!", 0.0, 20.0, 20.0, WHITE);
    draw_text(
        "Press ENTER to continue or ESC/Q to quit",
        0.0,
        40.0,
        20.0,
        WHITE,
    );
    if is_key_down(KeyCode::Enter) {
        state.game_state = GameState::Continuing;
        app_state.max_campaign_level += 1;
    }
    if is_key_down(KeyCode::Q) || is_key_down(KeyCode::Escape) {
        state.game_state = GameState::Quitting;
        app_state.max_campaign_level += 1;
    }
}
