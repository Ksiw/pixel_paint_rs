use crate::app::EditorState;
use crate::domain::PaintDocument;
use crate::localization::LocalizationManager;
use eframe::egui;

pub fn show(
    ui: &mut egui::Ui,
    localization: &LocalizationManager,
    document: &mut PaintDocument,
    editor: &mut EditorState,
) -> bool {
    let mut changed = false;
    let strip_height = 33.0;
    let top_padding = 4.0;
    let full_rect = ui.max_rect();
    let strip_rect = egui::Rect::from_min_size(
        egui::pos2(full_rect.left(), ui.cursor().top()),
        egui::vec2(full_rect.width(), strip_height),
    );
    let _ = ui.allocate_rect(strip_rect, egui::Sense::hover());
    ui.painter()
        .rect_filled(strip_rect, 0.0, ui.visuals().panel_fill);
    let baseline_y = strip_rect.bottom() - 1.0;
    ui.painter().line_segment(
        [
            egui::pos2(full_rect.left(), baseline_y),
            egui::pos2(full_rect.right(), baseline_y),
        ],
        egui::Stroke::new(1.0, ui.visuals().widgets.noninteractive.bg_stroke.color),
    );

    let mut strip_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(strip_rect.translate(egui::vec2(0.0, top_padding)))
            .layout(egui::Layout::left_to_right(egui::Align::Min)),
    );
    egui::ScrollArea::horizontal()
        .id_salt("paint_tab_strip")
        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
        .scroll_source(egui::scroll_area::ScrollSource::DRAG)
        .show(&mut strip_ui, |ui| {
            ui.add_space(f32::from(ui.style().spacing.window_margin.left));
            if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                editor.renaming_tab_id = None;
            }
            let tabs = document.tabs.clone();
            for tab in tabs {
                if editor.renaming_tab_id == Some(tab.id) {
                    let action = super::edit::draw_tab_title_editor(
                        ui,
                        &mut editor.tab_title_buffer,
                        baseline_y,
                        editor.renaming_tab_just_started,
                    );
                    if editor.renaming_tab_just_started {
                        editor.renaming_tab_just_started = false;
                    }
                    match action {
                        super::edit::TabTitleEditAction::Commit => {
                            if document.rename_tab(tab.id, editor.tab_title_buffer.clone()) {
                                editor.tab_title_buffer.clear();
                                changed = true;
                            }
                            editor.renaming_tab_id = None;
                        }
                        super::edit::TabTitleEditAction::Cancel => editor.renaming_tab_id = None,
                        super::edit::TabTitleEditAction::None => {}
                    }
                } else {
                    let response = super::widgets::draw_tab_button(
                        ui,
                        &tab.title,
                        document.active_tab_id == tab.id,
                        baseline_y,
                    );
                    if response.clicked() {
                        document.set_active_tab(tab.id);
                        editor.last_paint_cell = None;
                        changed = true;
                    }
                    if response.double_clicked() {
                        editor.renaming_tab_id = Some(tab.id);
                        editor.renaming_tab_just_started = true;
                        editor.tab_title_buffer = tab.title.clone();
                    }
                    response.context_menu(|ui| {
                        if ui.button(localization.get("tab_rename")).clicked() {
                            editor.renaming_tab_id = Some(tab.id);
                            editor.renaming_tab_just_started = true;
                            editor.tab_title_buffer = tab.title.clone();
                            ui.close();
                        }
                        if ui.button(localization.get("tab_delete")).clicked() {
                            if document.remove_tab_or_create_new(
                                tab.id,
                                format!("{} 1", localization.get("sheet_name")),
                            ) {
                                editor.last_paint_cell = None;
                                changed = true;
                            }
                            ui.close();
                        }
                    });
                }
            }
            ui.add_space(2.0);
            if super::widgets::draw_tab_add_button(ui, baseline_y).clicked() {
                let _ = document.add_tab_with_title(format!(
                    "{} {}",
                    localization.get("sheet_name"),
                    document.tabs.len() + 1
                ));
                changed = true;
            }
        });
    changed
}
