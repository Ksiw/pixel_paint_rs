use crate::app::{PaintTool, PixelPaintApp};
use crate::domain::DrawPrimitive;
use crate::ui::paint_palette::COLOR32_PALETTE;
use eframe::egui;

pub fn show_tool_section(ui: &mut egui::Ui, app: &mut PixelPaintApp) -> bool {
    let mut dirty = false;
    if ui
        .selectable_value(
            &mut app.editor.tool,
            PaintTool::Draw,
            app.localization.get("tool_draw"),
        )
        .changed()
    {
        dirty = true;
    }
    if ui
        .selectable_value(
            &mut app.editor.tool,
            PaintTool::Erase,
            app.localization.get("tool_erase"),
        )
        .changed()
    {
        dirty = true;
    }
    dirty
}

pub fn show_primitive_section(ui: &mut egui::Ui, app: &mut PixelPaintApp) -> bool {
    let mut dirty = false;
    if ui
        .selectable_value(
            &mut app.editor.primitive,
            DrawPrimitive::Line,
            app.localization.get("primitive_line"),
        )
        .changed()
    {
        dirty = true;
    }
    if ui
        .selectable_value(
            &mut app.editor.primitive,
            DrawPrimitive::Point,
            app.localization.get("primitive_point"),
        )
        .changed()
    {
        dirty = true;
    }
    dirty
}

pub fn show_size_section(ui: &mut egui::Ui, app: &mut PixelPaintApp) -> bool {
    let mut dirty = false;
    egui::ComboBox::from_id_salt("paint_size_combo")
        .selected_text((app.editor.size_index + 1).to_string())
        .show_ui(ui, |ui| {
            for size_index in 0..=3 {
                if ui
                    .selectable_value(
                        &mut app.editor.size_index,
                        size_index,
                        (size_index + 1).to_string(),
                    )
                    .changed()
                {
                    dirty = true;
                }
            }
        });
    dirty
}

pub fn show_palette_section(ui: &mut egui::Ui, app: &mut PixelPaintApp) -> bool {
    let mut dirty = false;
    ui.label(app.localization.get("color"));
    for (idx, color) in COLOR32_PALETTE.iter().enumerate() {
        let (rect, response) = ui.allocate_exact_size(egui::vec2(18.0, 18.0), egui::Sense::click());
        ui.painter().rect_filled(rect, 0.0, *color);
        let stroke = if app.editor.color_index as usize == idx {
            egui::Stroke::new(2.0, ui.visuals().widgets.active.bg_stroke.color)
        } else {
            egui::Stroke::new(1.0, egui::Color32::from_black_alpha(70))
        };
        ui.painter()
            .rect_stroke(rect, 0.0, stroke, egui::StrokeKind::Inside);
        if response.clicked() {
            app.editor.color_index = idx as u8;
            dirty = true;
        }
    }
    dirty
}

pub fn show_view_section(ui: &mut egui::Ui, app: &mut PixelPaintApp) -> bool {
    if ui.button(app.localization.get("center_view")).clicked() {
        app.editor.zoom = 1.0;
        app.editor.pan = egui::Vec2::ZERO;
        return true;
    }
    false
}
