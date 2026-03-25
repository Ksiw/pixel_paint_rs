use crate::domain::{DrawStroke, PaintTab};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PaintDocument {
    pub format_version: u32,
    pub active_tab_id: u64,
    pub next_tab_id: u64,
    pub next_stroke_id: u64,
    pub tabs: Vec<PaintTab>,
}

impl PaintDocument {
    pub fn new_with_title(title: String) -> Self {
        let first_tab = PaintTab::new(1, title);
        Self {
            format_version: 1,
            active_tab_id: first_tab.id,
            next_tab_id: 2,
            next_stroke_id: 1,
            tabs: vec![first_tab],
        }
    }

    pub fn active_tab(&self) -> Option<&PaintTab> {
        self.tabs.iter().find(|tab| tab.id == self.active_tab_id)
    }

    pub fn active_tab_strokes(&self) -> Option<&[DrawStroke]> {
        self.active_tab().map(|tab| tab.draw_strokes.as_slice())
    }

    pub fn active_tab_strokes_mut(&mut self) -> Option<&mut Vec<DrawStroke>> {
        self.tabs
            .iter_mut()
            .find(|tab| tab.id == self.active_tab_id)
            .map(|tab| &mut tab.draw_strokes)
    }
}
