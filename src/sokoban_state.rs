use crate::map::*;
use crate::prelude::*;
use macroquad::audio::*;
use macroquad::prelude::*;

pub struct SokobanState {
    pub texture_atlas: HashMap<String, Texture2D>,
    pub sound_atlas: HashMap<String, Sound>,
    pub map: Map,
    pub player: IVec2,
    pub crates: HashMap<IVec2, Crate>,
    pub movecount: u32,
    pub moves: Vec<Move>,
    pub game_state: GameState,
}

#[derive(Copy, Debug, PartialEq, Clone)]
pub enum GameState {
    Playing,
    Quitting,
    Continuing,
    Won,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Crate {
    pub id: u32,
}
impl Crate {
    pub fn new(id: u32) -> Self {
        Crate { id }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Move {
    pub player: IVec2,
    pub crates: HashMap<IVec2, Crate>,
    pub movecount: u32,
}
impl Move {
    pub fn new(player: IVec2, crates: HashMap<IVec2, Crate>, movecount: u32) -> Self {
        Self {
            player,
            crates,
            movecount,
        }
    }
}

impl SokobanState {
    pub fn from_file(
        path: String,
        texture_atlas: HashMap<String, Texture2D>,
        sound_atlas: HashMap<String, Sound>,
    ) -> Self {
        let (map, player_spawn, crates) = read_data_from_string(path);
        Self {
            texture_atlas,
            sound_atlas,
            map,
            player: player_spawn,
            crates,
            movecount: 0,
            moves: Vec::new(),
            game_state: GameState::Playing,
        }
    }
    pub fn update_from_file(&mut self, path: String) {
        let (new_map, new_player_spawn, new_crates) = read_data_from_string(path);
        self.map = new_map;
        self.player = new_player_spawn;
        self.crates = new_crates;
        self.movecount = 0;
        self.moves.clear();
        self.game_state = GameState::Playing;
    }
    pub fn get_current_move(&self) -> Move {
        Move::new(self.player.clone(), self.crates.clone(), self.movecount)
    }
}
use std::fs;
fn read_data_from_string(path: String) -> (Map, IVec2, HashMap<IVec2, Crate>) {
    //get the raw string from file
    let mut raw_data =
        fs::read_to_string(path).expect("failed to properly read the raw map data string");
    //then trim all the whitespace and make everything uppercase
    raw_data = raw_data.to_uppercase();
    raw_data.retain(|c| !c.is_whitespace());
    let mut map = Map::new();
    let mut player_spawn = IVec2::new(0, 0);
    let mut crates: HashMap<IVec2, Crate> = HashMap::new();
    let mut crate_num = 0;
    let mut index = 0;
    for char in raw_data.chars() {
        match char {
            '#' => {
                map.tiles[index as usize] = TileType::Wall;
            }
            '.' => {
                map.tiles[index as usize] = TileType::Floor;
            }
            'X' => {
                map.tiles[index as usize] = TileType::Floor;
                crates.insert(index_to_point(index), Crate::new(crate_num));
                crate_num += 1;
                //need to get an index to point function working
            }
            'O' => {
                map.tiles[index as usize] = TileType::LoadingSquare;
            }
            '@' => {
                map.tiles[index as usize] = TileType::Floor;
                player_spawn = index_to_point(index);
            }
            _ => {
                println!("Unrecognized character in raw map data")
            }
        }
        index += 1;
    }
    return (map, player_spawn, crates);
}

pub fn index_to_point(idx: usize) -> IVec2 {
    let index = idx as i32;
    let x = index % MAP_WIDTH;
    let y = index / MAP_WIDTH;
    IVec2::new(x, y)
}
///takes the current level and loads the relevant gamedata from that file
pub fn load_campaign_level(gamestate: &mut SokobanState, current_level: i32) {
    //maybe instead of plain numbers it will have "level_" in front of it but for now it's just numbers
    let path = format!("levels/campaign/{current_level}.txt");
    gamestate.update_from_file(path);
}
