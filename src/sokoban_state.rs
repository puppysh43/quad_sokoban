use crate::map::*;
use crate::prelude::*;
use macroquad::prelude::*;

pub struct SokobanState {
    pub texture_atlas: HashMap<String, Texture2D>,
    pub map: Map,
    pub player: Point,
    pub crates: HashMap<Point, Crate>,
    pub movecount: u32,
    pub moves: Vec<Move>,
    pub quitting: bool,
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
    pub player: Point,
    pub crates: HashMap<Point, Crate>,
    pub movecount: u32,
}
impl Move {
    pub fn new(player: Point, crates: HashMap<Point, Crate>, movecount: u32) -> Self {
        Self {
            player,
            crates,
            movecount,
        }
    }
}

impl SokobanState {
    pub fn from_file(path: String, texture_atlas: HashMap<String, Texture2D>) -> Self {
        let (map, player_spawn, crates) = read_data_from_string(path);
        Self {
            texture_atlas,
            map,
            player: player_spawn,
            crates,
            movecount: 0,
            moves: Vec::new(),
            quitting: false,
        }
    }
    pub fn get_current_move(&self) -> Move {
        Move::new(self.player.clone(), self.crates.clone(), self.movecount)
    }
}
use std::fs;
fn read_data_from_string(path: String) -> (Map, Point, HashMap<Point, Crate>) {
    //get the raw string from file
    let mut raw_data =
        fs::read_to_string(path).expect("failed to properly read the raw map data string");
    //then trim all the whitespace and make everything uppercase
    raw_data = raw_data.to_uppercase();
    raw_data.retain(|c| !c.is_whitespace());
    let mut map = Map::new();
    let mut player_spawn = Point::new(0, 0);
    let mut crates: HashMap<Point, Crate> = HashMap::new();
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

pub fn index_to_point(idx: usize) -> Point {
    let index = idx as i32;
    let x = index % SCREEN_WIDTH;
    let y = index / SCREEN_WIDTH;
    Point::new(x, y)
}
