use super::geometry::screen_to_grid_cell;
use super::render::{paint_grid, paint_strokes};
use crate::app::{EditorState, PaintTool};
use crate::domain::{DrawPrimitive, DrawStroke, PaintDocument};
use crate::localization::LocalizationManager;
use eframe::egui;
use std::collections::{HashSet, VecDeque};

const UNDO_HISTORY_LIMIT: usize = 100;

pub fn apply_draw_undo(document: &mut PaintDocument, editor: &mut EditorState) {
    let Some(previous) = editor.draw_undo_stack.pop() else {
        return;
    };
    if let Some(current) = document
        .active_tab_strokes()
        .map(|strokes| strokes.to_vec())
    {
        editor.draw_redo_stack.push(current);
        if let Some(strokes) = document.active_tab_strokes_mut() {
            *strokes = previous;
        }
    }
}

pub fn apply_draw_redo(document: &mut PaintDocument, editor: &mut EditorState) {
    let Some(next) = editor.draw_redo_stack.pop() else {
        return;
    };
    if let Some(current) = document
        .active_tab_strokes()
        .map(|strokes| strokes.to_vec())
    {
        editor.draw_undo_stack.push(current);
        if let Some(strokes) = document.active_tab_strokes_mut() {
            *strokes = next;
        }
    }
}

pub fn interact(
    ui: &mut egui::Ui,
    _localization: &LocalizationManager,
    document: &mut PaintDocument,
    editor: &mut EditorState,
) -> bool {
    let mut changed = false;
    let available = ui.available_size_before_wrap();
    let (rect, response) = ui.allocate_exact_size(available, egui::Sense::drag());
    let visible_rect = rect.intersect(ui.painter().clip_rect());
    let draw_rect = visible_rect.shrink(1.0);
    editor.canvas_size = Some([draw_rect.width(), draw_rect.height()]);
    editor.canvas_clip_min = Some([draw_rect.left() - rect.left(), draw_rect.top() - rect.top()]);

    if response.dragged_by(egui::PointerButton::Middle) {
        editor.pan += response.drag_delta();
        changed = true;
    }

    if response.hovered() {
        let zoom_delta = ui.input(|i| i.zoom_delta());
        if (zoom_delta - 1.0).abs() > f32::EPSILON {
            editor.zoom = (editor.zoom * zoom_delta).clamp(0.25, 4.0);
            changed = true;
        }
    }

    let painter = ui.painter().with_clip_rect(draw_rect);
    painter.rect_filled(draw_rect, 0.0, ui.visuals().extreme_bg_color);
    paint_grid(&painter, draw_rect, editor.pan, editor.zoom);
    paint_strokes(&painter, document, draw_rect, editor.pan, editor.zoom);
    ui.painter().rect_stroke(
        draw_rect,
        0.0,
        egui::Stroke::new(1.0, egui::Color32::BLACK),
        egui::StrokeKind::Inside,
    );

    if handle_draw_mode(ui, document, editor, draw_rect) {
        changed = true;
    }

    changed
}

fn handle_draw_mode(
    ui: &mut egui::Ui,
    document: &mut PaintDocument,
    editor: &mut EditorState,
    canvas_rect: egui::Rect,
) -> bool {
    let mut changed = false;
    let pointer_pos = ui
        .ctx()
        .input(|i| i.pointer.hover_pos().or(i.pointer.latest_pos()));
    let primary_down = ui
        .ctx()
        .input(|i| i.pointer.button_down(egui::PointerButton::Primary));

    // Snapshot берётся в начале жеста рисования, чтобы весь drag складывался в один шаг undo.
    if primary_down && editor.draw_action_snapshot.is_none() {
        editor.draw_action_snapshot = document.active_tab_strokes().map(|s| s.to_vec());
        editor.draw_action_changed = false;
    }

    if let Some(pointer_pos) = pointer_pos {
        if canvas_rect.contains(pointer_pos) && primary_down {
            let cell = screen_to_grid_cell(pointer_pos, canvas_rect, editor.pan, editor.zoom);
            if editor.last_paint_cell != Some(cell) {
                editor.last_paint_cell = Some(cell);
                match editor.tool {
                    PaintTool::Erase => {
                        let ctrl = ui.input(|i| i.modifiers.ctrl);
                        let removed = if ctrl {
                            erase_connected_color_at_cell(document, cell[0], cell[1])
                        } else {
                            erase_at_cell(document, cell[0], cell[1])
                        };
                        if removed > 0 {
                            editor.draw_action_changed = true;
                            changed = true;
                        }
                    }
                    PaintTool::Draw => {
                        let stroke_added = add_draw_stroke(
                            document,
                            editor.primitive,
                            editor.size_index.min(3),
                            editor.color_index,
                            vec![cell],
                        )
                        .is_some();
                        if stroke_added {
                            editor.draw_action_changed = true;
                            changed = true;
                        }
                    }
                }
            }
        } else if !primary_down {
            editor.last_paint_cell = None;
        }
    } else if !primary_down {
        editor.last_paint_cell = None;
    }

    if !primary_down {
        // Undo пополняется только если жест реально что-то изменил.
        if editor.draw_action_changed {
            if let Some(snapshot) = editor.draw_action_snapshot.take() {
                editor.draw_undo_stack.push(snapshot);
                if editor.draw_undo_stack.len() > UNDO_HISTORY_LIMIT {
                    let _ = editor.draw_undo_stack.remove(0);
                }
                editor.draw_redo_stack.clear();
            }
        } else {
            editor.draw_action_snapshot = None;
        }
        editor.draw_action_changed = false;
    }
    changed
}

fn add_draw_stroke(
    document: &mut PaintDocument,
    primitive: DrawPrimitive,
    size_index: u8,
    color_index: u8,
    cells: Vec<[i32; 2]>,
) -> Option<u64> {
    if cells.is_empty() {
        return None;
    }
    let id = document.next_stroke_id.max(1);
    document.next_stroke_id = id.saturating_add(1);
    let strokes = document.active_tab_strokes_mut()?;
    if let Some(last) = strokes.last_mut() {
        if last.primitive == primitive
            && last.size_index == size_index
            && last.color_index == color_index
            && cells.len() == 1
        {
            let cell = cells[0];
            if !last.cells.contains(&cell) {
                last.cells.push(cell);
            }
            return Some(last.id);
        }
    }

    strokes.push(DrawStroke {
        id,
        primitive,
        size_index,
        color_index,
        cells,
    });
    Some(id)
}

fn erase_at_cell(document: &mut PaintDocument, cx: i32, cy: i32) -> usize {
    let Some(strokes) = document.active_tab_strokes_mut() else {
        return 0;
    };
    let mut removed = 0_usize;
    for stroke in strokes.iter_mut() {
        let before = stroke.cells.len();
        stroke
            .cells
            .retain(|cell| !(cell[0] == cx && cell[1] == cy));
        removed = removed.saturating_add(before.saturating_sub(stroke.cells.len()));
    }
    strokes.retain(|stroke| !stroke.cells.is_empty());
    removed
}

fn erase_connected_color_at_cell(document: &mut PaintDocument, cx: i32, cy: i32) -> usize {
    let Some(strokes) = document.active_tab_strokes_mut() else {
        return 0;
    };

    let Some(seed) = strokes
        .iter()
        .rev()
        .find(|stroke| {
            stroke
                .cells
                .iter()
                .any(|cell| cell[0] == cx && cell[1] == cy)
        })
        .map(|stroke| (stroke.primitive, stroke.size_index, stroke.color_index))
    else {
        return 0;
    };

    let all_cells: HashSet<(i32, i32)> = strokes
        .iter()
        .filter(|stroke| {
            stroke.primitive == seed.0
                && stroke.size_index == seed.1
                && stroke.color_index == seed.2
        })
        .flat_map(|stroke| stroke.cells.iter().map(|cell| (cell[0], cell[1])))
        .collect();
    if !all_cells.contains(&(cx, cy)) {
        return 0;
    }

    let mut queue = VecDeque::from([(cx, cy)]);
    let mut region: HashSet<(i32, i32)> = HashSet::from([(cx, cy)]);
    while let Some((x, y)) = queue.pop_front() {
        for (nx, ny) in [
            (x - 1, y),
            (x + 1, y),
            (x, y - 1),
            (x, y + 1),
            (x - 1, y - 1),
            (x - 1, y + 1),
            (x + 1, y - 1),
            (x + 1, y + 1),
        ] {
            if all_cells.contains(&(nx, ny)) && region.insert((nx, ny)) {
                queue.push_back((nx, ny));
            }
        }
    }

    let mut removed = 0_usize;
    for stroke in strokes.iter_mut().filter(|stroke| {
        stroke.primitive == seed.0 && stroke.size_index == seed.1 && stroke.color_index == seed.2
    }) {
        let before = stroke.cells.len();
        stroke
            .cells
            .retain(|cell| !region.contains(&(cell[0], cell[1])));
        removed = removed.saturating_add(before.saturating_sub(stroke.cells.len()));
    }
    strokes.retain(|stroke| !stroke.cells.is_empty());
    removed
}
