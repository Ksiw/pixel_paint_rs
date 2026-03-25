use super::constants::{GRID_STEP, LINE_SIZES, POINT_SIZES};
use crate::domain::PaintDocument;
use crate::ui::paint_palette::COLOR32_PALETTE;
use crate::ui::render_core::{
    RenderTarget, ScreenPoint, ViewTransform, paint_grid as core_paint_grid,
    paint_strokes as core_paint_strokes,
};
use eframe::egui;

pub fn paint_grid(painter: &egui::Painter, canvas_rect: egui::Rect, pan: egui::Vec2, zoom: f32) {
    let mut target = EguiRenderTarget { painter };
    core_paint_grid(
        &mut target,
        canvas_rect.width(),
        canvas_rect.height(),
        ViewTransform {
            pan: [pan.x + canvas_rect.left(), pan.y + canvas_rect.top()],
            zoom,
            grid_step: GRID_STEP,
        },
    );
}

pub fn paint_strokes(
    painter: &egui::Painter,
    document: &PaintDocument,
    canvas_rect: egui::Rect,
    pan: egui::Vec2,
    zoom: f32,
) {
    let Some(tab) = document.active_tab() else {
        return;
    };
    let mut target = EguiRenderTarget { painter };
    core_paint_strokes(
        &mut target,
        &tab.draw_strokes,
        ViewTransform {
            pan: [pan.x + canvas_rect.left(), pan.y + canvas_rect.top()],
            zoom,
            grid_step: GRID_STEP,
        },
        LINE_SIZES,
        POINT_SIZES,
    );
}

struct EguiRenderTarget<'a> {
    painter: &'a egui::Painter,
}

impl RenderTarget for EguiRenderTarget<'_> {
    fn draw_grid_line(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, major: bool) {
        let minor_col = egui::Color32::from_rgba_premultiplied(0, 0, 0, 22);
        let major_col = egui::Color32::from_rgba_premultiplied(0, 0, 0, 40);
        self.painter.line_segment(
            [egui::pos2(x0, y0), egui::pos2(x1, y1)],
            egui::Stroke::new(
                if major { 0.8 } else { 0.6 },
                if major { major_col } else { minor_col },
            ),
        );
    }

    fn draw_line_point(&mut self, center: ScreenPoint, radius: f32, color_index: u8) {
        let color = COLOR32_PALETTE[(color_index as usize).min(COLOR32_PALETTE.len() - 1)];
        self.painter
            .circle_filled(egui::pos2(center.x, center.y), radius, color);
    }

    fn draw_line_segment(
        &mut self,
        from: ScreenPoint,
        to: ScreenPoint,
        thickness: f32,
        color_index: u8,
    ) {
        let color = COLOR32_PALETTE[(color_index as usize).min(COLOR32_PALETTE.len() - 1)];
        self.painter.line_segment(
            [egui::pos2(from.x, from.y), egui::pos2(to.x, to.y)],
            egui::Stroke::new(thickness, color),
        );
    }

    fn draw_point_rect(&mut self, center: ScreenPoint, side: f32, color_index: u8) {
        let color = COLOR32_PALETTE[(color_index as usize).min(COLOR32_PALETTE.len() - 1)];
        let rect =
            egui::Rect::from_center_size(egui::pos2(center.x, center.y), egui::vec2(side, side));
        self.painter.rect_filled(rect, 0.0, color);
    }
}
