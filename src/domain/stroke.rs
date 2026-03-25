#[derive(
    Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash, Default,
)]
pub enum DrawPrimitive {
    #[default]
    Line,
    Point,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DrawStroke {
    pub id: u64,
    #[serde(default)]
    pub primitive: DrawPrimitive,
    pub size_index: u8,
    pub color_index: u8,
    pub cells: Vec<[i32; 2]>,
}
