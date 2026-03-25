use eframe::egui;

pub fn draw_tab_button(
    ui: &mut egui::Ui,
    title: &str,
    active: bool,
    baseline_y: f32,
) -> egui::Response {
    let desired_size = egui::vec2(140.0, 28.0);
    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    let visuals = ui.visuals();
    let fill = visuals.panel_fill;
    let tab_rect = if active {
        egui::Rect::from_min_max(rect.min, egui::pos2(rect.max.x, baseline_y))
    } else {
        egui::Rect::from_min_max(
            rect.min + egui::vec2(0.0, 2.0),
            egui::pos2(rect.max.x, baseline_y),
        )
    };
    let stroke_color = if active {
        visuals.widgets.active.bg_stroke.color
    } else if response.hovered() {
        visuals.widgets.hovered.bg_stroke.color
    } else {
        visuals.widgets.inactive.bg_stroke.color
    };
    let stroke = egui::Stroke::new(1.0, stroke_color);
    let rounding = egui::CornerRadius {
        nw: 6,
        ne: 6,
        sw: 0,
        se: 0,
    };
    ui.painter().rect_filled(tab_rect, rounding, fill);
    paint_tab_outline(ui.painter(), tab_rect, stroke, 6.0);
    if active {
        ui.painter().line_segment(
            [
                egui::pos2(tab_rect.left() + 1.0, baseline_y),
                egui::pos2(tab_rect.right() - 1.0, baseline_y),
            ],
            egui::Stroke::new(3.0, fill),
        );
    }
    let text = ellipsize_tab_title(ui, title, tab_rect.width() - 16.0);
    ui.painter().text(
        tab_rect.center(),
        egui::Align2::CENTER_CENTER,
        text,
        egui::TextStyle::Button.resolve(ui.style()),
        if active {
            visuals.strong_text_color()
        } else {
            visuals.text_color()
        },
    );
    response
}

pub fn draw_tab_add_button(ui: &mut egui::Ui, baseline_y: f32) -> egui::Response {
    let desired_size = egui::vec2(26.0, 26.0);
    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    let visuals = ui.visuals();
    let button_rect = egui::Rect::from_min_max(
        rect.min + egui::vec2(0.0, 2.0),
        egui::pos2(rect.max.x, baseline_y),
    );
    let fill = visuals.panel_fill;
    let stroke = egui::Stroke::new(
        1.0,
        if response.hovered() {
            visuals.widgets.hovered.bg_stroke.color
        } else {
            visuals.widgets.inactive.bg_stroke.color
        },
    );
    let rounding = egui::CornerRadius {
        nw: 6,
        ne: 6,
        sw: 0,
        se: 0,
    };
    ui.painter().rect_filled(button_rect, rounding, fill);
    paint_tab_outline(ui.painter(), button_rect, stroke, 6.0);
    ui.painter().text(
        button_rect.center(),
        egui::Align2::CENTER_CENTER,
        "+",
        egui::TextStyle::Button.resolve(ui.style()),
        visuals.text_color(),
    );
    response
}

pub fn paint_tab_outline(
    painter: &egui::Painter,
    rect: egui::Rect,
    stroke: egui::Stroke,
    radius: f32,
) {
    let mut points = Vec::with_capacity(10);
    let left = rect.left();
    let right = rect.right();
    let top = rect.top();
    let bottom = rect.bottom();
    let r = radius.min(rect.width() * 0.5).min((bottom - top) * 0.5);
    points.push(egui::pos2(left, bottom));
    points.push(egui::pos2(left, top + r));
    for step in 0..=4 {
        let t = step as f32 / 4.0;
        let angle = std::f32::consts::PI - (std::f32::consts::FRAC_PI_2 * t);
        points.push(egui::pos2(
            left + r + r * angle.cos(),
            top + r - r * angle.sin(),
        ));
    }
    for step in 0..=4 {
        let t = step as f32 / 4.0;
        let angle = std::f32::consts::FRAC_PI_2 * (1.0 - t);
        points.push(egui::pos2(
            right - r + r * angle.cos(),
            top + r - r * angle.sin(),
        ));
    }
    points.push(egui::pos2(right, bottom));
    painter.add(egui::Shape::Path(egui::epaint::PathShape::line(
        points, stroke,
    )));
}

fn ellipsize_tab_title(ui: &egui::Ui, title: &str, max_width: f32) -> String {
    let font_id = egui::TextStyle::Button.resolve(ui.style());
    let fits = |text: &str| {
        ui.painter()
            .layout_no_wrap(text.to_owned(), font_id.clone(), ui.visuals().text_color())
            .size()
            .x
            <= max_width
    };
    if fits(title) {
        return title.to_string();
    }
    let chars: Vec<char> = title.chars().collect();
    if chars.is_empty() {
        return String::new();
    }
    for idx in 1..=chars.len() {
        let candidate = format!("{}..", chars[..idx].iter().collect::<String>());
        if !fits(&candidate) {
            return if idx == 1 {
                "..".to_string()
            } else {
                format!("{}..", chars[..idx - 1].iter().collect::<String>())
            };
        }
    }
    format!("{}..", title)
}
