use super::{AppSettings, EditorState};
use crate::domain::PaintDocument;
use crate::localization::LocalizationManager;
use std::path::PathBuf;
use std::time::{Duration, Instant};

pub struct PixelPaintApp {
    pub document: PaintDocument,
    pub editor: EditorState,
    pub settings: AppSettings,
    pub current_file_path: Option<PathBuf>,
    pub status_message: Option<String>,
    pub localization: LocalizationManager,
    pub session_dirty: bool,
    pub last_session_save_at: Instant,
    pub window_size: Option<[f32; 2]>,
    pub pending_restore_window_size: Option<[f32; 2]>,
    pub show_about_dialog: bool,
}

impl PixelPaintApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(crate::theme::dark_visuals());
        if let Some(session) = crate::persistence::session::load_session() {
            crate::theme::configure_fonts(&cc.egui_ctx, &session.language);
            let localization = LocalizationManager::new(&session.language);
            return Self {
                document: session.document,
                editor: session.editor.into_editor_state(),
                settings: AppSettings::default(),
                current_file_path: session.current_file_path,
                status_message: None,
                localization,
                session_dirty: false,
                last_session_save_at: Instant::now(),
                window_size: session.window_size,
                pending_restore_window_size: session.window_size,
                show_about_dialog: false,
            };
        }

        let localization = LocalizationManager::new("ru");
        crate::theme::configure_fonts(&cc.egui_ctx, localization.current_language());
        Self {
            document: PaintDocument::new_with_title(format!(
                "{} 1",
                localization.get("sheet_name")
            )),
            editor: EditorState::default(),
            settings: AppSettings::default(),
            current_file_path: None,
            status_message: None,
            localization,
            session_dirty: false,
            last_session_save_at: Instant::now(),
            window_size: None,
            pending_restore_window_size: None,
            show_about_dialog: false,
        }
    }

    pub fn mark_session_dirty(&mut self) {
        self.session_dirty = true;
    }

    pub fn autosave_session_if_needed(&mut self) {
        if !self.session_dirty {
            return;
        }
        if self.last_session_save_at.elapsed()
            < Duration::from_secs(self.settings.autosave_interval_seconds)
        {
            return;
        }
        self.save_session_now();
    }

    pub fn save_session_now(&mut self) {
        let session = crate::persistence::session::AppSession {
            document: self.document.clone(),
            current_file_path: self.current_file_path.clone(),
            language: self.localization.current_language().to_string(),
            editor: crate::persistence::session::EditorSession::from(&self.editor),
            window_size: self.window_size,
        };
        if crate::persistence::session::save_session(&session).is_ok() {
            self.session_dirty = false;
            self.last_session_save_at = Instant::now();
        }
    }
}

impl eframe::App for PixelPaintApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        if let Some([width, height]) = self.pending_restore_window_size.take() {
            ctx.send_viewport_cmd(eframe::egui::ViewportCommand::InnerSize(
                eframe::egui::vec2(width, height),
            ));
        }
        if let Some(inner_rect) = ctx.input(|i| i.viewport().inner_rect) {
            let next_size = [inner_rect.width(), inner_rect.height()];
            if self.window_size != Some(next_size) {
                self.window_size = Some(next_size);
                self.mark_session_dirty();
            }
        }
        crate::ui::main_window::show(ctx, self);
        self.autosave_session_if_needed();
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.save_session_now();
    }
}
