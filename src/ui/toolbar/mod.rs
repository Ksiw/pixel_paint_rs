mod sections;
mod widgets;

use crate::app::PixelPaintApp;
use eframe::egui;

pub fn show(ui: &mut egui::Ui, app: &mut PixelPaintApp) {
    let mut should_mark_dirty = false;
    ui.horizontal_wrapped(|ui| {
        should_mark_dirty |= sections::show_tool_section(ui, app);
        widgets::toolbar_separator(ui);
        should_mark_dirty |= sections::show_primitive_section(ui, app);
        widgets::toolbar_separator(ui);
        should_mark_dirty |= sections::show_size_section(ui, app);
        widgets::toolbar_separator(ui);
        should_mark_dirty |= sections::show_palette_section(ui, app);
        widgets::toolbar_separator(ui);
        should_mark_dirty |= sections::show_view_section(ui, app);
    });
    if should_mark_dirty {
        app.mark_session_dirty();
    }
}
