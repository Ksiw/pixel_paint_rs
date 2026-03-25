use crate::domain::{DrawPrimitive, DrawStroke};
use std::collections::HashSet;

// Единый render core для экранного canvas и PNG-экспорта.
// Вся геометрия рисования должна сходиться здесь, чтобы поведение не расходилось.
#[derive(Debug, Clone, Copy)]
pub struct ViewTransform {
    pub pan: [f32; 2],
    pub zoom: f32,
    pub grid_step: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct StrokeStyle {
    pub primitive: DrawPrimitive,
    pub size_index: u8,
    pub color_index: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct ScreenPoint {
    pub x: f32,
    pub y: f32,
}

// Абстрактная цель рендера: egui-холст, PNG-буфер или любой другой backend.
pub trait RenderTarget {
    fn draw_grid_line(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, major: bool);
    fn draw_line_point(&mut self, center: ScreenPoint, radius: f32, color_index: u8);
    fn draw_line_segment(
        &mut self,
        from: ScreenPoint,
        to: ScreenPoint,
        thickness: f32,
        color_index: u8,
    );
    fn draw_point_rect(&mut self, center: ScreenPoint, side: f32, color_index: u8);
}

pub fn paint_grid<T: RenderTarget>(
    target: &mut T,
    width: f32,
    height: f32,
    transform: ViewTransform,
) {
    let major_every = 4_i32;
    let world_min_x = (-transform.pan[0]) / transform.zoom;
    let world_max_x = (width - transform.pan[0]) / transform.zoom;
    let world_min_y = (-transform.pan[1]) / transform.zoom;
    let world_max_y = (height - transform.pan[1]) / transform.zoom;

    let min_ix = ((world_min_x / transform.grid_step).floor() as i32).saturating_sub(1);
    let max_ix = ((world_max_x / transform.grid_step).ceil() as i32).saturating_add(1);
    let min_iy = ((world_min_y / transform.grid_step).floor() as i32).saturating_sub(1);
    let max_iy = ((world_max_y / transform.grid_step).ceil() as i32).saturating_add(1);

    for ix in min_ix..=max_ix {
        let x = ix as f32 * transform.grid_step * transform.zoom + transform.pan[0];
        target.draw_grid_line(x, 0.0, x, height, ix.rem_euclid(major_every) == 0);
    }
    for iy in min_iy..=max_iy {
        let y = iy as f32 * transform.grid_step * transform.zoom + transform.pan[1];
        target.draw_grid_line(0.0, y, width, y, iy.rem_euclid(major_every) == 0);
    }
}

pub fn paint_strokes<T: RenderTarget>(
    target: &mut T,
    strokes: &[DrawStroke],
    transform: ViewTransform,
    line_sizes: [f32; 4],
    point_sizes: [f32; 4],
) {
    // Штрихи группируются по стилю, чтобы line/point-логика была одинаковой
    // для canvas и PNG и не зависела от того, как именно документ был набран по событиям.
    struct GroupCells {
        style: StrokeStyle,
        cells: HashSet<(i32, i32)>,
    }

    let mut groups: Vec<GroupCells> = Vec::new();
    for stroke in strokes {
        let idx = groups.iter().position(|group| {
            group.style.primitive == stroke.primitive
                && group.style.size_index == stroke.size_index
                && group.style.color_index == stroke.color_index
        });
        let group = if let Some(index) = idx {
            &mut groups[index]
        } else {
            groups.push(GroupCells {
                style: StrokeStyle {
                    primitive: stroke.primitive,
                    size_index: stroke.size_index.min(3),
                    color_index: stroke.color_index,
                },
                cells: HashSet::new(),
            });
            groups.last_mut().expect("group exists")
        };
        for cell in &stroke.cells {
            group.cells.insert((cell[0], cell[1]));
        }
    }

    groups.sort_by_key(|group| std::cmp::Reverse(group.style.size_index));
    for group in groups {
        let idx = (group.style.size_index as usize).min(3);
        match group.style.primitive {
            DrawPrimitive::Line => {
                let thickness = line_sizes[idx] * transform.zoom;
                for (x, y) in &group.cells {
                    let center = cell_center_on_screen(*x, *y, transform);
                    target.draw_line_point(
                        center,
                        (thickness * 0.5).max(1.0),
                        group.style.color_index,
                    );
                    for (nx, ny) in [(*x + 1, *y), (*x, *y + 1)] {
                        if !group.cells.contains(&(nx, ny)) {
                            continue;
                        }
                        let next_center = cell_center_on_screen(nx, ny, transform);
                        target.draw_line_segment(
                            center,
                            next_center,
                            thickness,
                            group.style.color_index,
                        );
                    }
                }
            }
            DrawPrimitive::Point => {
                let side = (point_sizes[idx] * transform.zoom * 2.0)
                    .clamp(2.0, transform.grid_step * transform.zoom);
                for (x, y) in &group.cells {
                    let center = cell_center_on_screen(*x, *y, transform);
                    target.draw_point_rect(center, side, group.style.color_index);
                }
            }
        }
    }
}

pub fn cell_center_on_screen(x: i32, y: i32, transform: ViewTransform) -> ScreenPoint {
    ScreenPoint {
        x: x as f32 * transform.grid_step * transform.zoom
            + transform.pan[0]
            + transform.grid_step * 0.5 * transform.zoom,
        y: y as f32 * transform.grid_step * transform.zoom
            + transform.pan[1]
            + transform.grid_step * 0.5 * transform.zoom,
    }
}
