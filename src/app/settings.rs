#[derive(Debug, Clone)]
pub struct AppSettings {
    pub autosave_interval_seconds: u64,
    pub tab_strip_height: f32,
    pub status_bar_height: f32,
    pub canvas_inner_margin: f32,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            autosave_interval_seconds: 30,
            tab_strip_height: 33.0,
            status_bar_height: 20.0,
            canvas_inner_margin: 1.0,
        }
    }
}
