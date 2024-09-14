use crate::editor_state::*;
use crate::map::*;
use crate::prelude::*;
use macroquad::prelude::*;
pub fn system(state: &mut EditorState) {
    clear_background(WHITE);
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let pt = IVec2::new(x, y);
            let idx = map_idx(x, y);
            if state.map.in_bounds(pt) {
                match state.map.tiles[idx] {
                    BrushType::Wall => {
                        draw_texture(
                            state.texture_atlas.get("wall").unwrap(),
                            (x * TILE_WIDTH) as f32,
                            (y * TILE_HEIGHT) as f32,
                            WHITE,
                        );
                    }
                    BrushType::Floor => {
                        draw_texture(
                            state.texture_atlas.get("floor").unwrap(),
                            (x * TILE_WIDTH) as f32,
                            (y * TILE_HEIGHT) as f32,
                            WHITE,
                        );
                    }
                    BrushType::Crate => {
                        draw_texture(
                            state.texture_atlas.get("floor").unwrap(),
                            (x * TILE_WIDTH) as f32,
                            (y * TILE_HEIGHT) as f32,
                            WHITE,
                        );

                        draw_texture(
                            state.texture_atlas.get("crate").unwrap(),
                            (x * TILE_WIDTH) as f32,
                            (y * TILE_HEIGHT) as f32,
                            WHITE,
                        );
                    }
                    BrushType::LoadingZone => {
                        draw_texture(
                            state.texture_atlas.get("cratespot").unwrap(),
                            (x * TILE_WIDTH) as f32,
                            (y * TILE_HEIGHT) as f32,
                            WHITE,
                        );
                    }
                    BrushType::Player => {
                        draw_texture(
                            state.texture_atlas.get("player").unwrap(),
                            (x * TILE_WIDTH) as f32,
                            (y * TILE_HEIGHT) as f32,
                            WHITE,
                        );
                    }
                }
            }
        }
    }
    if let EditorControlState::Reticule(pos) = state.control_state {
        draw_texture(
            state.texture_atlas.get("reticule").unwrap(),
            (pos.x * TILE_WIDTH) as f32,
            (pos.y * TILE_HEIGHT) as f32,
            Color::from_rgba(57, 255, 20, 255),
        );
    }
}
