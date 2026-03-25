use super::constants::GRID_STEP;
use eframe::egui;

pub fn screen_to_grid_cell(
    pos: egui::Pos2,
    canvas_rect: egui::Rect,
    pan: egui::Vec2,
    zoom: f32,
) -> [i32; 2] {
    let world = to_world(canvas_rect, pos, pan, zoom.max(0.001));
    [
        (world.x / GRID_STEP).floor() as i32,
        (world.y / GRID_STEP).floor() as i32,
    ]
}

pub fn to_world(
    canvas_rect: egui::Rect,
    pos: egui::Pos2,
    pan: egui::Vec2,
    zoom: f32,
) -> egui::Vec2 {
    egui::vec2(
        (pos.x - canvas_rect.left() - pan.x) / zoom.max(0.001),
        (pos.y - canvas_rect.top() - pan.y) / zoom.max(0.001),
    )
}
