use super::DrawStroke;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PaintTab {
    pub id: u64,
    pub title: String,
    pub canvas_width: f32,
    pub canvas_height: f32,
    pub draw_strokes: Vec<DrawStroke>,
}

impl PaintTab {
    pub fn new(id: u64, title: String) -> Self {
        Self {
            id,
            title,
            canvas_width: 3200.0,
            canvas_height: 2000.0,
            draw_strokes: Vec::new(),
        }
    }
}
