use crate::editor_state::*;
use crate::map::map_idx;
use crate::prelude::*;
use macroquad::audio::*;
use macroquad::prelude::*;

mod input;
mod paint_render;
mod saving;

pub enum EditorMOI {
    None,
    MoveReticule(IVec2),
    PaintTile,
    Save,
}
pub fn run_systems(state: &mut EditorState) {
    match input::system(state) {
        EditorMOI::None => {
            //do nothing I guess
        }
        //user is attempting to move the reticule
        EditorMOI::MoveReticule(delta) => {
            process_move(state, delta);
        }
        //user is requesting to change the type of a tile
        EditorMOI::PaintTile => {
            //if the user has requested to paint a tile get the pos of the reticule from the control state
            //then apply the selected tiletype of the brush from the state

            if let EditorControlState::Reticule(pos) = state.control_state {
                if state.brush_type == BrushType::Player {
                    //if the user is trying to place down a player then make sure there isn't one already
                    let mut existing_player = false;
                    for tile in state.map.tiles.iter() {
                        if tile == &BrushType::Player {
                            existing_player = true;
                        }
                    }
                    if !existing_player {
                        state.map.tiles[map_idx(pos.x, pos.y)] = state.brush_type.clone();
                    } else {
                        play_sound_once(state.sound_atlas.get("wall collision").unwrap())
                    }
                } else {
                    //any other type of tile can occur as many times as needed
                    state.map.tiles[map_idx(pos.x, pos.y)] = state.brush_type.clone();
                }
            }
        }
        //the user is attempting to write out the current map in the editor to the path
        //specified in the app name field of the editor state
        EditorMOI::Save => {
            //this is gonna be the real nightmare
            if state.map_name.is_some() {
                editor_map_to_txt(&state.map, state.map_name.clone().unwrap());
            }
            //change control state to root and reset the map state
            state.control_state = EditorControlState::Root;
            state.map = EditorMap::new();
            state.map_name = Some(String::new());
        }
    }
    //still need to render the screen but also need to properly implement the UI stuff for the saving process
    if state.control_state == EditorControlState::Saving {
        //do the saving rendering and ui and stuff
        saving::system(state);
    } else {
        paint_render::system(state);
    }

    //handle input, moving the cursor, changing the brush type, etc
    //process that input to make changes to the map
    //process the player saving/writing out the map if they request to do so
}

fn process_move(state: &mut EditorState, delta: IVec2) {
    //use an if let statement to capture the reticule position from the control state enum
    if let EditorControlState::Reticule(pos) = state.control_state {
        let new_pos = IVec2::new(pos.x + delta.x, pos.y + delta.y);
        //if the new position is actually in the bounds of the map change the position of the reticule
        if state.map.in_bounds(new_pos) {
            state.control_state = EditorControlState::Reticule(new_pos);
        }
    }
}

//takes in a map and a path to save it to
fn editor_map_to_txt(map: &EditorMap, name: String) {
    let mut line = String::new();
    let mut final_output = String::new();
    for tile in map.tiles.iter() {
        match tile {
            BrushType::Wall => {
                line.push('#');
            }
            BrushType::Floor => {
                line.push('.');
            }
            BrushType::Crate => {
                line.push('X');
            }
            BrushType::LoadingZone => {
                line.push('O');
            }
            BrushType::Player => {
                line.push('@');
            }
        }
        //if the collected character is enough to be the same length as the screen width
        if line.len() == MAP_WIDTH as usize {
            //add it to the main string with a new character file attached.
            final_output.push_str(&line);
            final_output.push_str("\n");
            line.clear();
        }
    }
    //make the correct path and then use it to write out
    let path = format!("levels/custom/{name}.txt");
    std::fs::write(path, final_output).expect("Failed to write file.");
}
