use crate::map::*;
use crate::prelude::*;
use crate::sokoban_state::*;
use macroquad::prelude::*;

pub fn system(state: &mut SokobanState) {
    clear_background(WHITE);
    //first render the game map
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let pt = IVec2::new(x, y);
            let idx = map_idx(x, y);
            if state.map.in_bounds(pt) {
                match state.map.tiles[idx] {
                    TileType::Wall => {
                        draw_texture(
                            state.texture_atlas.get("wall").unwrap(),
                            (x * TILE_WIDTH) as f32,
                            (y * TILE_HEIGHT) as f32,
                            WHITE,
                        );
                    }
                    TileType::Floor => {
                        draw_texture(
                            state.texture_atlas.get("floor").unwrap(),
                            (x * TILE_WIDTH) as f32,
                            (y * TILE_HEIGHT) as f32,
                            WHITE,
                        );
                    }
                    TileType::LoadingSquare => {
                        draw_texture(
                            state.texture_atlas.get("cratespot").unwrap(),
                            (x * TILE_WIDTH) as f32,
                            (y * TILE_HEIGHT) as f32,
                            WHITE,
                        );
                    }
                }
            }
        }
    }
    //then render the crates
    for crate_pos in state.crates.keys() {
        draw_texture(
            state.texture_atlas.get("crate").unwrap(),
            (crate_pos.x * TILE_WIDTH) as f32,
            (crate_pos.y * TILE_HEIGHT) as f32,
            WHITE,
        );
    }
    //then render the player
    draw_texture(
        state.texture_atlas.get("player").unwrap(),
        (state.player.x * TILE_WIDTH) as f32,
        (state.player.y * TILE_HEIGHT) as f32,
        WHITE,
    );
}
