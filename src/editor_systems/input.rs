use crate::editor_state::*;
use crate::editor_systems::*;
use crate::prelude::*;
use macroquad::prelude::*;

pub fn system(state: &mut EditorState) -> EditorMOI {
    match state.control_state {
        EditorControlState::Root => {
            if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::KpEnter) {
                //press enter to start editing
                state.control_state = EditorControlState::Reticule(IVec2::new(0, 0));
                return EditorMOI::None;
            } else if is_key_pressed(KeyCode::LeftControl) && is_key_pressed(KeyCode::S) {
                //use control s to try and save
                state.control_state = EditorControlState::Saving;
                return EditorMOI::None;
            } else {
                return EditorMOI::None;
            }
        }
        EditorControlState::Reticule(_pos) => {
            //if the brush shape is a reticule they can move the reticule and place tiles depending on what they've selected
            if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::Kp4) {
                //move the reticule left
                return EditorMOI::MoveReticule(IVec2::new(-1, 0));
            } else if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::Kp8) {
                //move the reticule up
                return EditorMOI::MoveReticule(IVec2::new(0, -1));
            } else if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::Kp6) {
                //move the reticule right
                return EditorMOI::MoveReticule(IVec2::new(1, 0));
            } else if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::Kp2) {
                //move the reticule down
                return EditorMOI::MoveReticule(IVec2::new(0, 1));
            } else if is_key_pressed(KeyCode::Key1) {
                //select the wall brushtype
                state.brush_type = BrushType::Wall;
                return EditorMOI::None;
            } else if is_key_pressed(KeyCode::Key2) {
                //select the floor brushtype
                state.brush_type = BrushType::Floor;
                return EditorMOI::None;
            } else if is_key_pressed(KeyCode::Key3) {
                //select the crate brushtype
                state.brush_type = BrushType::Crate;
                return EditorMOI::None;
            } else if is_key_pressed(KeyCode::Key4) {
                //select the loading zone brushtype
                state.brush_type = BrushType::LoadingZone;
                return EditorMOI::None;
            } else if is_key_pressed(KeyCode::Key5) {
                //select the player brushtype
                state.brush_type = BrushType::Player;
                return EditorMOI::None;
            } else if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::KpEnter) {
                //paint the selected brushtype onto the position of the reticule
                return EditorMOI::PaintTile;
            } else if is_key_pressed(KeyCode::Escape) {
                state.control_state = EditorControlState::Root;
                return EditorMOI::None;
                //go from the reticule brushshape and back to None
            } else {
                //otherwise just do nothing
                return EditorMOI::None;
            }
        }
        EditorControlState::Saving => {
            if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::KpEnter) {
                //if the user pressed enter try to save it
                return EditorMOI::Save;
            } else {
                return EditorMOI::None;
            }
        }
    }
}
