use crate::domain::{DrawPrimitive, DrawStroke};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PaintTool {
    #[default]
    Draw,
    Erase,
}

#[derive(Debug, Clone)]
pub struct EditorState {
    pub zoom: f32,
    pub pan: eframe::egui::Vec2,
    pub canvas_size: Option<[f32; 2]>,
    pub canvas_clip_min: Option<[f32; 2]>,
    pub tool: PaintTool,
    pub primitive: DrawPrimitive,
    pub size_index: u8,
    pub color_index: u8,
    pub last_paint_cell: Option<[i32; 2]>,
    pub draw_undo_stack: Vec<Vec<DrawStroke>>,
    pub draw_redo_stack: Vec<Vec<DrawStroke>>,
    pub draw_action_snapshot: Option<Vec<DrawStroke>>,
    pub draw_action_changed: bool,
    pub renaming_tab_id: Option<u64>,
    pub renaming_tab_just_started: bool,
    pub tab_title_buffer: String,
}

impl Default for EditorState {
    fn default() -> Self {
        Self {
            zoom: 1.0,
            pan: eframe::egui::Vec2::ZERO,
            canvas_size: None,
            canvas_clip_min: None,
            tool: PaintTool::Draw,
            primitive: DrawPrimitive::Line,
            size_index: 0,
            color_index: 0,
            last_paint_cell: None,
            draw_undo_stack: Vec::new(),
            draw_redo_stack: Vec::new(),
            draw_action_snapshot: None,
            draw_action_changed: false,
            renaming_tab_id: None,
            renaming_tab_just_started: false,
            tab_title_buffer: String::new(),
        }
    }
}
