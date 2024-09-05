use crate::editor_state::*;
use crate::prelude::*;
use macroquad::prelude::*;
pub enum EditorMOI {}
pub fn run_systems(state: &mut EditorState) {
    //handle input, moving the cursor, changing the brush type, etc
    //process that input to make changes to the map
    //process the player saving/writing out the map if they request to do so
}
fn input(state: &mut EditorState) {
    match state.brush_shape {
        BrushShape::None => {
            //if the brushshape is nothing then all they can do is look and select different brushes
        }
        BrushShape::Reticule(pos) => {
            //if the brush shape is a reticule they can move the reticule and place tiles depending on what they've selected
            if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::Kp4) {
                //move the reticule left
            } else if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::Kp8) {
                //move the reticule up
            } else if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::Kp6) {
                //move the reticule right
            } else if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::Kp2) {
                //move the reticule down
            } else if is_key_pressed(KeyCode::Key1) {
                //select the wall brushtype
            } else if is_key_pressed(KeyCode::Key2) {
                //select the floor brushtype
            } else if is_key_pressed(KeyCode::Key3) {
                //select the crate brushtype
            } else if is_key_pressed(KeyCode::Key4) {
                //select the loading zone brushtype
            } else if is_key_pressed(KeyCode::Key5) {
                //select the player brushtype
            } else if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::KpEnter) {
                //paint the selected brushtype onto the position of the reticule
            } else if is_key_pressed(KeyCode::Escape) {
                //go from the reticule brushshape and back to None
            }
        }
    }
}
