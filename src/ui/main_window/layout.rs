use super::file_actions;
use crate::app::PixelPaintApp;
use crate::localization::SUPPORTED_LANGUAGES;
use eframe::egui;

pub fn show(ctx: &egui::Context, app: &mut PixelPaintApp) {
    show_menu_bar(ctx, app);
    show_toolbar(ctx, app);
    show_tabs(ctx, app);
    show_status_bar(ctx, app);
    show_canvas(ctx, app);
}

fn show_menu_bar(ctx: &egui::Context, app: &mut PixelPaintApp) {
    egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
        egui::MenuBar::new().ui(ui, |ui| {
            ui.menu_button(app.localization.get("menu_file"), |ui| {
                if ui.button(app.localization.get("menu_open")).clicked() {
                    file_actions::open_document(app);
                    ui.close();
                }
                if ui.button(app.localization.get("menu_save")).clicked() {
                    file_actions::save_document(app);
                    ui.close();
                }
                if ui.button(app.localization.get("menu_save_as")).clicked() {
                    file_actions::save_document_as(app);
                    ui.close();
                }
                if ui.button(app.localization.get("menu_export_png")).clicked() {
                    file_actions::export_current_view_png(app);
                    ui.close();
                }
            });

            ui.menu_button(app.localization.get("menu_settings"), |ui| {
                ui.menu_button(app.localization.get("menu_language"), |ui| {
                    ui.set_min_width(150.0);
                    for (lang, key) in SUPPORTED_LANGUAGES {
                        if ui
                            .selectable_label(
                                app.localization.current_language() == lang,
                                app.localization.get(key),
                            )
                            .clicked()
                        {
                            app.localization.set_language(lang);
                            crate::theme::configure_fonts(ctx, app.localization.current_language());
                            app.mark_session_dirty();
                            ui.close();
                        }
                    }
                });
            });

            ui.menu_button(app.localization.get("menu_help"), |ui| {
                if ui.button(app.localization.get("menu_about")).clicked() {
                    app.show_about_dialog = true;
                    ui.close();
                }
            });
        });
    });
}

fn show_toolbar(ctx: &egui::Context, app: &mut PixelPaintApp) {
    egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
        crate::ui::toolbar::show(ui, app);
    });
}

fn show_tabs(ctx: &egui::Context, app: &mut PixelPaintApp) {
    egui::TopBottomPanel::top("tabs")
        .exact_height(app.settings.tab_strip_height)
        .show_separator_line(false)
        .frame(
            egui::Frame::default()
                .inner_margin(egui::Margin::ZERO)
                .outer_margin(egui::Margin::ZERO),
        )
        .show(ctx, |ui| {
            if crate::ui::tabs::show(ui, &app.localization, &mut app.document, &mut app.editor) {
                app.mark_session_dirty();
            }
        });
}

fn show_status_bar(ctx: &egui::Context, app: &mut PixelPaintApp) {
    let panel_fill = ctx.style().visuals.panel_fill;
    egui::TopBottomPanel::bottom("status_bar")
        .exact_height(app.settings.status_bar_height)
        .show_separator_line(false)
        .frame(
            egui::Frame::default()
                .fill(panel_fill)
                .stroke(egui::Stroke::NONE)
                .inner_margin(egui::Margin::ZERO)
                .outer_margin(egui::Margin::ZERO),
        )
        .show(ctx, |ui| {
            let rect = ui.max_rect();
            let message = app
                .status_message
                .as_deref()
                .unwrap_or(&app.localization.get("status_ready"))
                .to_owned();
            ui.painter().text(
                egui::pos2(rect.left() + 12.0, rect.center().y - 2.0),
                egui::Align2::LEFT_CENTER,
                message,
                egui::TextStyle::Body.resolve(ui.style()),
                ui.visuals().text_color(),
            );
        });
}

fn show_canvas(ctx: &egui::Context, app: &mut PixelPaintApp) {
    egui::CentralPanel::default().show(ctx, |ui| {
        if crate::ui::canvas::interact(ui, &app.localization, &mut app.document, &mut app.editor) {
            app.mark_session_dirty();
        }
    });
}
