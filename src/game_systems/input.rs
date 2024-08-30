use crate::game_systems::MessageOfIntent;
use crate::prelude::*;
use macroquad::prelude::*;

pub fn system() -> MessageOfIntent {
    //instead of needing to capture the keypress from the context extract it and run it through a match statement
    //macroquad is able to detect input with a function you can incorporate as part of a giant if/if else chain

    if is_key_pressed(KeyCode::Left) && is_key_down(KeyCode::LeftControl)
        || is_key_pressed(KeyCode::Left) && is_key_down(KeyCode::RightControl)
    {
        return MessageOfIntent::Rewind;
    } else if is_key_pressed(KeyCode::Right) && is_key_down(KeyCode::LeftControl)
        || is_key_pressed(KeyCode::Right) && is_key_down(KeyCode::RightControl)
    {
        return MessageOfIntent::Forward;
    } else if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::Kp4) {
        return MessageOfIntent::MovePlayer(IVec2::new(-1, 0));
    } else if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::Kp6) {
        return MessageOfIntent::MovePlayer(IVec2::new(1, 0));
    } else if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::Kp8) {
        return MessageOfIntent::MovePlayer(IVec2::new(0, -1));
    } else if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::Kp2) {
        return MessageOfIntent::MovePlayer(IVec2::new(0, 1));
    } else if is_key_pressed(KeyCode::Q) && is_key_down(KeyCode::LeftControl)
        || is_key_pressed(KeyCode::Q) && is_key_down(KeyCode::RightControl)
    {
        return MessageOfIntent::Quit;
    } else if is_key_pressed(KeyCode::R) && is_key_down(KeyCode::LeftControl)
        || is_key_pressed(KeyCode::R) && is_key_down(KeyCode::RightControl)
    {
        return MessageOfIntent::Reset;
    } else {
        return MessageOfIntent::None;
    }
}
