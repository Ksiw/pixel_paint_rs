use crate::app::{EditorState, PaintTool};
use crate::domain::{DrawPrimitive, PaintDocument};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// DTO слоя persistence. Эти структуры описывают сериализуемое состояние,
// а не являются runtime-моделью UI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSession {
    pub document: PaintDocument,
    pub current_file_path: Option<PathBuf>,
    pub language: String,
    pub editor: EditorSession,
    pub window_size: Option<[f32; 2]>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorSession {
    pub zoom: f32,
    pub pan: [f32; 2],
    pub canvas_size: Option<[f32; 2]>,
    pub canvas_clip_min: Option<[f32; 2]>,
    pub tool: PaintToolDto,
    pub primitive: DrawPrimitiveDto,
    pub size_index: u8,
    pub color_index: u8,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PaintToolDto {
    Draw,
    Erase,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DrawPrimitiveDto {
    Line,
    Point,
}

impl From<&EditorState> for EditorSession {
    fn from(value: &EditorState) -> Self {
        Self {
            zoom: value.zoom,
            pan: [value.pan.x, value.pan.y],
            canvas_size: value.canvas_size,
            canvas_clip_min: value.canvas_clip_min,
            tool: match value.tool {
                PaintTool::Draw => PaintToolDto::Draw,
                PaintTool::Erase => PaintToolDto::Erase,
            },
            primitive: match value.primitive {
                DrawPrimitive::Line => DrawPrimitiveDto::Line,
                DrawPrimitive::Point => DrawPrimitiveDto::Point,
            },
            size_index: value.size_index,
            color_index: value.color_index,
        }
    }
}

impl EditorSession {
    pub fn into_editor_state(self) -> EditorState {
        EditorState {
            zoom: self.zoom.max(0.1),
            pan: eframe::egui::vec2(self.pan[0], self.pan[1]),
            canvas_size: self.canvas_size,
            canvas_clip_min: self.canvas_clip_min,
            tool: match self.tool {
                PaintToolDto::Draw => PaintTool::Draw,
                PaintToolDto::Erase => PaintTool::Erase,
            },
            primitive: match self.primitive {
                DrawPrimitiveDto::Line => DrawPrimitive::Line,
                DrawPrimitiveDto::Point => DrawPrimitive::Point,
            },
            size_index: self.size_index.min(3),
            color_index: self.color_index,
            ..EditorState::default()
        }
    }
}
