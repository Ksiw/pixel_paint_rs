use crate::app::PixelPaintApp;
use crate::domain::DrawPrimitive;
use eframe::egui;

pub fn handle_shortcuts(ctx: &egui::Context, app: &mut PixelPaintApp) {
    let text_input_active = ctx.wants_keyboard_input();
    let home_pressed = !text_input_active && ctx.input(|i| i.key_pressed(egui::Key::Home));
    let reset_zoom_pressed =
        !text_input_active && ctx.input(|i| i.modifiers.command && i.key_pressed(egui::Key::Num0));
    let undo_pressed =
        !text_input_active && ctx.input(|i| i.modifiers.command && i.key_pressed(egui::Key::Z));
    let redo_pressed = !text_input_active
        && ctx.input(|i| {
            (i.modifiers.command && i.key_pressed(egui::Key::Y))
                || (i.modifiers.command && i.modifiers.shift && i.key_pressed(egui::Key::Z))
        });
    let new_pressed =
        !text_input_active && ctx.input(|i| i.modifiers.command && i.key_pressed(egui::Key::N));
    let open_pressed =
        !text_input_active && ctx.input(|i| i.modifiers.command && i.key_pressed(egui::Key::O));
    let save_pressed =
        !text_input_active && ctx.input(|i| i.modifiers.command && i.key_pressed(egui::Key::S));
    let save_as_pressed = !text_input_active
        && ctx.input(|i| i.modifiers.command && i.modifiers.shift && i.key_pressed(egui::Key::S));
    let line_primitive_pressed =
        !text_input_active && ctx.input(|i| i.key_pressed(egui::Key::Q));
    let point_primitive_pressed =
        !text_input_active && ctx.input(|i| i.key_pressed(egui::Key::W));
    let size_1_pressed =
        !text_input_active && ctx.input(|i| i.key_pressed(egui::Key::Num1));
    let size_2_pressed =
        !text_input_active && ctx.input(|i| i.key_pressed(egui::Key::Num2));
    let size_3_pressed =
        !text_input_active && ctx.input(|i| i.key_pressed(egui::Key::Num3));
    let size_4_pressed =
        !text_input_active && ctx.input(|i| i.key_pressed(egui::Key::Num4));

    if reset_zoom_pressed {
        app.editor.zoom = 1.0;
        app.editor.pan = egui::Vec2::ZERO;
        app.mark_session_dirty();
    }
    if home_pressed {
        app.editor.pan = egui::Vec2::ZERO;
        app.mark_session_dirty();
    }
    if undo_pressed {
        crate::ui::canvas::apply_draw_undo(&mut app.document, &mut app.editor);
        app.mark_session_dirty();
    }
    if redo_pressed {
        crate::ui::canvas::apply_draw_redo(&mut app.document, &mut app.editor);
        app.mark_session_dirty();
    }
    if new_pressed {
        super::file_actions::new_document(app);
    }
    if open_pressed {
        super::file_actions::open_document(app);
    }
    if save_as_pressed {
        super::file_actions::save_document_as(app);
    } else if save_pressed {
        super::file_actions::save_document(app);
    }
    if line_primitive_pressed {
        app.editor.primitive = DrawPrimitive::Line;
        app.mark_session_dirty();
    }
    if point_primitive_pressed {
        app.editor.primitive = DrawPrimitive::Point;
        app.mark_session_dirty();
    }
    if size_1_pressed {
        app.editor.size_index = 0;
        app.mark_session_dirty();
    }
    if size_2_pressed {
        app.editor.size_index = 1;
        app.mark_session_dirty();
    }
    if size_3_pressed {
        app.editor.size_index = 2;
        app.mark_session_dirty();
    }
    if size_4_pressed {
        app.editor.size_index = 3;
        app.mark_session_dirty();
    }
}
