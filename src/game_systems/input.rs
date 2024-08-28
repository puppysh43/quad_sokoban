use crate::game_systems::MessageOfIntent;
use crate::prelude::*;
use macroquad::prelude::*;

pub fn player_input() -> MessageOfIntent {
    //instead of needing to capture the keypress from the context extract it and run it through a match statement
    //macroquad is able to detect input with a function you can incorporate as part of a giant if/if else chain
    if is_key_pressed(KeyCode::Left) {
        return MessageOfIntent::MovePlayer(Point::new(-1, 0));
    } else if is_key_pressed(KeyCode::Right) {
        return MessageOfIntent::MovePlayer(Point::new(1, 0));
    } else if is_key_pressed(KeyCode::Up) {
        return MessageOfIntent::MovePlayer(Point::new(0, -1));
    } else if is_key_pressed(KeyCode::Down) {
        return MessageOfIntent::MovePlayer(Point::new(0, 1));
    } else if is_key_pressed(KeyCode::Q) {
        return MessageOfIntent::Quit;
    } else {
        return MessageOfIntent::None;
    }
}
