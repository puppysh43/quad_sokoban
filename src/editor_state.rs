use crate::prelude::*;
use macroquad::prelude::*;
///This struct will hold all the data needed for the level editor
pub struct EditorState {
    ///holds the raw map currently being worked on
    pub map: EditorMap,
    ///holds the name if present of the level being worked on
    pub map_name: Option<String>,
    ///current brush shape
    pub brush_shape: BrushShape,
    ///current brush tile type
    pub brush_type: BrushType,
    ///is the mouse enabled in the editor
    pub mouse_enabled: bool,
}

///The kind of brush being used and the information needed to apply that brush to the map
pub enum BrushShape {
    ///For when no brush is selected, like when the
    None,
    Reticule(IVec2),
    Rectangle(RectangleData),
}
///Struct that defines a rectangle selection
pub struct RectangleData {
    ///what kind of tile is being painted into this rectangle
    pub top_left_vertice: Option<IVec2>,
    pub bottom_right_vertice: Option<IVec2>,
    // pub is_hollow: bool,
}

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
pub struct EditorMap {
    pub tiles: Vec<BrushType>,
}

impl EditorMap {
    //peepeepoopoo
}
