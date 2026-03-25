mod export_png;
mod file_actions;
mod layout;
mod shortcut_actions;

use crate::app::PixelPaintApp;
use eframe::egui;

pub fn show(ctx: &egui::Context, app: &mut PixelPaintApp) {
    shortcut_actions::handle_shortcuts(ctx, app);
    layout::show(ctx, app);
    show_about_dialog(ctx, app);
}

fn show_about_dialog(ctx: &egui::Context, app: &mut PixelPaintApp) {
    if app.show_about_dialog {
        let mut open = app.show_about_dialog;
        egui::Window::new(app.localization.get("menu_about"))
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Pixel Paint RS");
                    ui.label(format!("v{}", env!("CARGO_PKG_VERSION")));
                });
                ui.separator();
                ui.label(app.localization.get("about_text"));
                ui.separator();
                ui.label(format!(
                    "{} Ksiw",
                    app.localization.get("about_author_label")
                ));
                ui.label(app.localization.get("about_contact_label"));
                ui.hyperlink_to("stewhiki@gmail.com", "mailto:stewhiki@gmail.com");
                ui.hyperlink_to("t.me/mr_Ksiw", "https://t.me/mr_Ksiw");
                ui.separator();
                ui.label(app.localization.get("about_license_label"));
                ui.label(app.localization.get("about_license_text"));
            });
        app.show_about_dialog = open;
    }
}
