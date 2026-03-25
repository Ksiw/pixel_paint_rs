use crate::domain::DrawPrimitive;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct StyledCell {
    pub primitive: DrawPrimitive,
    pub size_index: u8,
    pub color_index: u8,
    pub cell: [i32; 2],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DrawHistoryAction {
    Draw {
        tab_id: u64,
        primitive: DrawPrimitive,
        size_index: u8,
        color_index: u8,
        cells: Vec<[i32; 2]>,
    },
    Erase {
        tab_id: u64,
        cells: Vec<StyledCell>,
    },
}

#[derive(Debug, Clone, Default)]
pub enum PendingDrawAction {
    #[default]
    None,
    Draw {
        primitive: DrawPrimitive,
        size_index: u8,
        color_index: u8,
        cells: Vec<[i32; 2]>,
    },
    Erase {
        cells: Vec<StyledCell>,
    },
}
