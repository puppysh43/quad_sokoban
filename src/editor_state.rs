use crate::prelude::*;
use macroquad::audio::*;
use macroquad::prelude::*;
///This struct will hold all the data needed for the level editor
#[derive(Clone, Debug)]
pub struct EditorState {
    ///holds the raw map currently being worked on
    pub map: EditorMap,
    ///holds the name if present of the level being worked on
    pub map_name: Option<String>,
    ///current brush shape
    pub control_state: EditorControlState,
    ///current brush tile type
    pub brush_type: BrushType,
    ///is the mouse enabled in the editor
    pub mouse_enabled: bool,
    ///texture atlas
    pub texture_atlas: HashMap<String, Texture2D>,
    ///sound atlas
    pub sound_atlas: HashMap<String, Sound>,
}
impl EditorState {
    pub fn new(
        texture_atlas: HashMap<String, Texture2D>,
        sound_atlas: HashMap<String, Sound>,
    ) -> Self {
        Self {
            map: EditorMap::new(),
            map_name: None,
            control_state: EditorControlState::Root,
            brush_type: BrushType::Floor,
            mouse_enabled: false,
            texture_atlas,
            sound_atlas,
        }
    }
}
///The kind of brush being used and the information needed to apply that brush to the map
#[derive(Clone, Debug, PartialEq)]
pub enum EditorControlState {
    ///For when no brush is selected, like when the
    Root,
    Reticule(IVec2),
    Saving,
}

///The kind of tile or entity that's being placed by the brush
#[derive(Clone, Debug, PartialEq)]
pub enum BrushType {
    Wall,
    Floor,
    Crate,
    LoadingZone,
    ///Only available as a single tile reticule brush for obvious reasons
    Player,
}
///actually holds the map data to be edited and displayed in the level editor
///before being dumped into a game-readable text format upon saving
#[derive(Clone, Debug, PartialEq)]
pub struct EditorMap {
    pub tiles: Vec<BrushType>,
}

impl EditorMap {
    pub fn new() -> Self {
        Self {
            tiles: vec![BrushType::Wall; NUM_TILES],
        }
    }
    pub fn in_bounds(&self, point: IVec2) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }
}
