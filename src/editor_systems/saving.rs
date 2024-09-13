use std::ops::Mul;

use crate::editor_state::*;
use crate::prelude::*;
use macroquad::prelude::*;
use macroquad::ui::*;
use widgets::InputText;

pub fn system(state: &mut EditorState) {
    //
    clear_background(GRAY);
    let mut buffer = String::new();
    if state.map_name.is_some() {
        buffer = state.map_name.as_ref().unwrap().clone();
    }
    //make an input box for the name of the saved level
    InputText::new(hash!())
        .position(Vec2::new(50.0, 50.0))
        .size(Vec2::new(200.0, 30.0))
        .ui(&mut root_ui(), &mut buffer);
    state.map_name = Some(buffer);
    //check to make sure the name doesn't have any spaces I guess
    //or implement a whitespace deleting feature of the write out function
    //this is silly
}
