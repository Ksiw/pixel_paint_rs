use super::export_png;
use crate::app::PixelPaintApp;
use std::fs;
use std::path::{Path, PathBuf};

pub fn new_document(app: &mut PixelPaintApp) {
    app.document = crate::domain::PaintDocument::new_with_title(format!(
        "{} 1",
        app.localization.get("sheet_name")
    ));
    app.current_file_path = None;
    app.status_message = Some(app.localization.get("new_document"));
    app.mark_session_dirty();
}

pub fn open_document(app: &mut PixelPaintApp) {
    if let Some(path) = rfd::FileDialog::new()
        .add_filter("Pixel Paint JSON", &["json"])
        .pick_file()
    {
        match fs::read_to_string(&path)
            .ok()
            .and_then(|text| crate::persistence::json::deserialize_document(&text).ok())
        {
            Some(document) => {
                app.document = document;
                sync_active_tab_title_with_path(app, &path);
                app.current_file_path = Some(path.clone());
                app.status_message = Some(format!(
                    "{} {}",
                    app.localization.get("opened"),
                    path.display()
                ));
                app.mark_session_dirty();
            }
            None => {
                app.status_message = Some(format!(
                    "{} {}",
                    app.localization.get("open_failed"),
                    path.display()
                ));
            }
        }
    }
}

pub fn save_document(app: &mut PixelPaintApp) {
    if let Some(path) = app.current_file_path.clone() {
        save_document_to_path(app, path);
    } else {
        save_document_as(app);
    }
}

pub fn save_document_as(app: &mut PixelPaintApp) {
    if let Some(path) = rfd::FileDialog::new()
        .add_filter("Pixel Paint JSON", &["json"])
        .set_file_name("drawing.json")
        .save_file()
    {
        save_document_to_path(app, path);
    }
}

pub fn export_current_view_png(app: &mut PixelPaintApp) {
    let Some(tab) = app.document.active_tab() else {
        return;
    };
    if let Some(path) = rfd::FileDialog::new()
        .add_filter("PNG", &["png"])
        .set_file_name("drawing.png")
        .save_file()
    {
        match export_png::render_current_view_png(tab, &app.editor).save(&path) {
            Ok(()) => {
                app.status_message = Some(format!(
                    "{} {}",
                    app.localization.get("exported_png"),
                    path.display()
                ));
            }
            Err(_) => {
                app.status_message = Some(format!(
                    "{} {}",
                    app.localization.get("export_png_failed"),
                    path.display()
                ));
            }
        }
    }
}

fn save_document_to_path(app: &mut PixelPaintApp, path: PathBuf) {
    match crate::persistence::json::serialize_document(&app.document)
        .ok()
        .and_then(|text| fs::write(&path, text).ok())
    {
        Some(()) => {
            sync_active_tab_title_with_path(app, &path);
            app.current_file_path = Some(path.clone());
            app.status_message = Some(format!(
                "{} {}",
                app.localization.get("saved"),
                path.display()
            ));
            app.mark_session_dirty();
        }
        None => {
            app.status_message = Some(format!(
                "{} {}",
                app.localization.get("save_failed"),
                path.display()
            ));
        }
    }
}

fn sync_active_tab_title_with_path(app: &mut PixelPaintApp, path: &Path) {
    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
        let _ = app
            .document
            .rename_tab(app.document.active_tab_id, stem.to_string());
    }
}
