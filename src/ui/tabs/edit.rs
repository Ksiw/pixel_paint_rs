use eframe::egui;

pub enum TabTitleEditAction {
    None,
    Commit,
    Cancel,
}

pub fn draw_tab_title_editor(
    ui: &mut egui::Ui,
    title_buffer: &mut String,
    baseline_y: f32,
    request_focus: bool,
) -> TabTitleEditAction {
    let desired_size = egui::vec2(140.0, 28.0);
    let (rect, _) = ui.allocate_exact_size(desired_size, egui::Sense::hover());
    let visuals = ui.visuals();
    let tab_rect = egui::Rect::from_min_max(rect.min, egui::pos2(rect.max.x, baseline_y));
    let rounding = egui::CornerRadius {
        nw: 6,
        ne: 6,
        sw: 0,
        se: 0,
    };
    ui.painter()
        .rect_filled(tab_rect, rounding, visuals.panel_fill);
    super::widgets::paint_tab_outline(
        ui.painter(),
        tab_rect,
        egui::Stroke::new(1.0, visuals.widgets.active.bg_stroke.color),
        6.0,
    );
    ui.painter().line_segment(
        [
            egui::pos2(tab_rect.left() + 1.0, baseline_y),
            egui::pos2(tab_rect.right() - 1.0, baseline_y),
        ],
        egui::Stroke::new(3.0, visuals.panel_fill),
    );
    let edit_rect = tab_rect.shrink2(egui::vec2(8.0, 4.0));
    let response = ui.put(
        edit_rect,
        egui::TextEdit::singleline(title_buffer)
            .id_salt("paint_tab_title_editor")
            .frame(false)
            .clip_text(false)
            .desired_width(edit_rect.width()),
    );
    if request_focus && !response.has_focus() {
        response.request_focus();
    }
    let enter = ui.input(|i| i.key_pressed(egui::Key::Enter));
    let escape = ui.input(|i| i.key_pressed(egui::Key::Escape));
    if escape {
        return TabTitleEditAction::Cancel;
    }
    if enter || (response.lost_focus() && !response.clicked()) {
        return TabTitleEditAction::Commit;
    }
    TabTitleEditAction::None
}
