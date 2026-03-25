use crate::app::EditorState;
use crate::domain::PaintTab;
use crate::ui::paint_palette::RGBA_PALETTE;
use crate::ui::render_core::{
    RenderTarget, ScreenPoint, ViewTransform, paint_grid as core_paint_grid,
    paint_strokes as core_paint_strokes,
};
use eframe::egui;
use image::{Rgba, RgbaImage};

const GRID_STEP: i32 = 16;
const LINE_SIZES: [i32; 4] = [2, 3, 6, 9];
const POINT_SIZES: [i32; 4] = [2, 3, 5, 8];

pub fn render_current_view_png(tab: &PaintTab, editor: &EditorState) -> RgbaImage {
    let size = editor
        .canvas_size
        .unwrap_or([tab.canvas_width.max(1.0), tab.canvas_height.max(1.0)]);
    let clip_min = editor.canvas_clip_min.unwrap_or([0.0, 0.0]);
    render_current_view_png_with_params(
        tab,
        size,
        editor.pan - egui::vec2(clip_min[0], clip_min[1]),
        editor.zoom,
    )
}

fn render_current_view_png_with_params(
    tab: &PaintTab,
    canvas_size: [f32; 2],
    pan: egui::Vec2,
    zoom: f32,
) -> RgbaImage {
    let width = canvas_size[0].max(1.0).floor() as u32;
    let height = canvas_size[1].max(1.0).floor() as u32;
    let mut img = RgbaImage::from_pixel(width, height, Rgba([22, 26, 33, 255]));

    let mut target = PngRenderTarget { img: &mut img };
    core_paint_grid(
        &mut target,
        width as f32,
        height as f32,
        ViewTransform {
            pan: [pan.x, pan.y],
            zoom: zoom.max(0.001),
            grid_step: GRID_STEP as f32,
        },
    );
    core_paint_strokes(
        &mut target,
        &tab.draw_strokes,
        ViewTransform {
            pan: [pan.x, pan.y],
            zoom,
            grid_step: GRID_STEP as f32,
        },
        LINE_SIZES.map(|x| x as f32),
        POINT_SIZES.map(|x| x as f32),
    );

    img
}

struct PngRenderTarget<'a> {
    img: &'a mut RgbaImage,
}

impl RenderTarget for PngRenderTarget<'_> {
    fn draw_grid_line(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, major: bool) {
        let color = if major { [0, 0, 0, 40] } else { [0, 0, 0, 22] };
        if (x0 - x1).abs() < f32::EPSILON {
            let x = x0.round() as i32;
            for y in y0.round() as i32..y1.round() as i32 {
                blend_pixel(self.img, x, y, color);
            }
        } else {
            let y = y0.round() as i32;
            for x in x0.round() as i32..x1.round() as i32 {
                blend_pixel(self.img, x, y, color);
            }
        }
    }

    fn draw_line_point(&mut self, center: ScreenPoint, radius: f32, color_index: u8) {
        let color = RGBA_PALETTE[(color_index as usize).min(RGBA_PALETTE.len() - 1)];
        fill_circle(
            self.img,
            center.x.round() as i32,
            center.y.round() as i32,
            radius.round() as i32,
            color,
        );
    }

    fn draw_line_segment(
        &mut self,
        from: ScreenPoint,
        to: ScreenPoint,
        thickness: f32,
        color_index: u8,
    ) {
        let color = RGBA_PALETTE[(color_index as usize).min(RGBA_PALETTE.len() - 1)];
        draw_thick_line(
            self.img,
            from.x.round() as i32,
            from.y.round() as i32,
            to.x.round() as i32,
            to.y.round() as i32,
            ((thickness * 0.5).max(1.0)).round() as i32,
            color,
        );
    }

    fn draw_point_rect(&mut self, center: ScreenPoint, side: f32, color_index: u8) {
        let color = RGBA_PALETTE[(color_index as usize).min(RGBA_PALETTE.len() - 1)];
        let side = side.round() as i32;
        fill_rect(
            self.img,
            center.x.round() as i32 - side / 2,
            center.y.round() as i32 - side / 2,
            side,
            side,
            color,
        );
    }
}

fn fill_rect(img: &mut RgbaImage, x: i32, y: i32, w: i32, h: i32, color: [u8; 4]) {
    for py in y.max(0)..(y + h).min(img.height() as i32) {
        for px in x.max(0)..(x + w).min(img.width() as i32) {
            img.put_pixel(px as u32, py as u32, Rgba(color));
        }
    }
}

fn fill_circle(img: &mut RgbaImage, cx: i32, cy: i32, radius: i32, color: [u8; 4]) {
    for y in (cy - radius).max(0)..=(cy + radius).min(img.height() as i32 - 1) {
        for x in (cx - radius).max(0)..=(cx + radius).min(img.width() as i32 - 1) {
            let dx = x - cx;
            let dy = y - cy;
            if dx * dx + dy * dy <= radius * radius {
                img.put_pixel(x as u32, y as u32, Rgba(color));
            }
        }
    }
}

fn draw_thick_line(
    img: &mut RgbaImage,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    radius: i32,
    color: [u8; 4],
) {
    let dx = x1 - x0;
    let dy = y1 - y0;
    let steps = dx.abs().max(dy.abs()).max(1);
    for step in 0..=steps {
        let t = step as f32 / steps as f32;
        let x = x0 as f32 + dx as f32 * t;
        let y = y0 as f32 + dy as f32 * t;
        fill_circle(img, x.round() as i32, y.round() as i32, radius, color);
    }
}

fn blend_pixel(img: &mut RgbaImage, x: i32, y: i32, color: [u8; 4]) {
    if x < 0 || y < 0 || x >= img.width() as i32 || y >= img.height() as i32 {
        return;
    }
    let dst = img.get_pixel_mut(x as u32, y as u32);
    let alpha = color[3] as f32 / 255.0;
    for i in 0..3 {
        dst[i] = ((dst[i] as f32 * (1.0 - alpha)) + (color[i] as f32 * alpha)).round() as u8;
    }
}
