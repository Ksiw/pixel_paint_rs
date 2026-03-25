pub fn app_icon() -> eframe::egui::IconData {
    let width = 64_u32;
    let height = 64_u32;
    let mut rgba = vec![0_u8; (width * height * 4) as usize];

    fill_rect(&mut rgba, width, 0, 0, 64, 64, [33, 37, 43, 255]);
    fill_round_rect(&mut rgba, width, 5, 5, 54, 54, 10, [22, 26, 33, 255]);

    for x in [14_u32, 23, 32, 41, 50] {
        draw_line(&mut rgba, width, x, 11, x, 53, [58, 64, 76, 140]);
    }
    for y in [14_u32, 23, 32, 41, 50] {
        draw_line(&mut rgba, width, 11, y, 53, y, [58, 64, 76, 140]);
    }

    let blue = [97, 175, 239, 255];
    let green = [126, 171, 132, 255];
    fill_round_rect(&mut rgba, width, 18, 31, 9, 9, 2, blue);
    fill_round_rect(&mut rgba, width, 27, 31, 9, 9, 2, blue);
    fill_round_rect(&mut rgba, width, 36, 31, 9, 9, 2, blue);
    fill_round_rect(&mut rgba, width, 36, 22, 9, 9, 2, blue);
    fill_round_rect(&mut rgba, width, 27, 40, 9, 9, 2, green);

    fill_rect(&mut rgba, width, 37, 14, 4, 16, [243, 152, 78, 255]);
    fill_rect(&mut rgba, width, 41, 10, 7, 8, [232, 104, 96, 255]);
    fill_rect(&mut rgba, width, 33, 18, 6, 6, [241, 243, 245, 255]);
    draw_line(&mut rgba, width, 36, 17, 46, 27, [243, 152, 78, 255]);
    draw_line(&mut rgba, width, 34, 19, 44, 29, [241, 243, 245, 255]);

    eframe::egui::IconData {
        rgba,
        width,
        height,
    }
}

fn fill_rect(rgba: &mut [u8], width: u32, x: u32, y: u32, w: u32, h: u32, color: [u8; 4]) {
    for py in y..(y + h) {
        for px in x..(x + w) {
            put_pixel(rgba, width, px, py, color);
        }
    }
}

fn fill_round_rect(
    rgba: &mut [u8],
    width: u32,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
    radius: u32,
    color: [u8; 4],
) {
    for py in y..(y + h) {
        for px in x..(x + w) {
            let dx = if px < x + radius {
                x + radius - px
            } else if px >= x + w - radius {
                px - (x + w - radius - 1)
            } else {
                0
            };
            let dy = if py < y + radius {
                y + radius - py
            } else if py >= y + h - radius {
                py - (y + h - radius - 1)
            } else {
                0
            };
            if dx == 0
                || dy == 0
                || dx.saturating_mul(dx) + dy.saturating_mul(dy) <= radius.saturating_mul(radius)
            {
                put_pixel(rgba, width, px, py, color);
            }
        }
    }
}

fn draw_line(rgba: &mut [u8], width: u32, x0: u32, y0: u32, x1: u32, y1: u32, color: [u8; 4]) {
    let mut x0 = x0 as i32;
    let mut y0 = y0 as i32;
    let x1 = x1 as i32;
    let y1 = y1 as i32;
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        if x0 >= 0 && y0 >= 0 {
            put_pixel(rgba, width, x0 as u32, y0 as u32, color);
        }
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = err * 2;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}

fn put_pixel(rgba: &mut [u8], width: u32, x: u32, y: u32, color: [u8; 4]) {
    let index = ((y * width + x) * 4) as usize;
    if index + 3 < rgba.len() {
        rgba[index..index + 4].copy_from_slice(&color);
    }
}
