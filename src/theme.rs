use eframe::egui::{Color32, Stroke, Visuals};
use eframe::egui::{FontData, FontDefinitions, FontFamily};
use std::fs;

pub fn dark_visuals() -> Visuals {
    let mut visuals = Visuals::dark();
    let bg_0 = Color32::from_rgb(33, 37, 43);
    let bg_1 = Color32::from_rgb(40, 44, 52);
    let accent = Color32::from_rgb(97, 175, 239);
    let accent_soft = Color32::from_rgb(126, 171, 132);
    let border = Color32::from_rgb(92, 101, 121);

    visuals.override_text_color = Some(Color32::from_rgb(171, 178, 191));
    visuals.hyperlink_color = accent;
    visuals.faint_bg_color = Color32::from_rgb(46, 52, 63);
    visuals.extreme_bg_color = Color32::from_rgb(22, 26, 33);
    visuals.code_bg_color = Color32::from_rgb(46, 52, 64);
    visuals.warn_fg_color = Color32::from_rgb(229, 192, 123);
    visuals.error_fg_color = Color32::from_rgb(224, 108, 117);
    visuals.window_fill = bg_1;
    visuals.panel_fill = bg_0;
    visuals.window_stroke = Stroke::new(1.0, border);
    visuals.button_frame = true;

    visuals.widgets.noninteractive.bg_fill = bg_1;
    visuals.widgets.noninteractive.weak_bg_fill = Color32::from_rgb(50, 56, 68);
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, border);
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, Color32::from_rgb(171, 178, 191));

    visuals.widgets.inactive.bg_fill = Color32::from_rgb(72, 81, 99);
    visuals.widgets.inactive.weak_bg_fill = Color32::from_rgb(62, 70, 86);
    visuals.widgets.inactive.bg_stroke = Stroke::new(0.8, Color32::from_rgb(95, 106, 127));
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, Color32::from_rgb(198, 205, 218));
    visuals.widgets.inactive.expansion = 0.0;

    visuals.widgets.hovered.bg_fill = Color32::from_rgb(80, 91, 111);
    visuals.widgets.hovered.weak_bg_fill = Color32::from_rgb(80, 91, 111);
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, accent_soft);
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, Color32::from_rgb(214, 221, 233));
    visuals.widgets.hovered.expansion = 0.2;

    visuals.widgets.active.bg_fill = Color32::from_rgb(89, 104, 130);
    visuals.widgets.active.weak_bg_fill = Color32::from_rgb(89, 104, 130);
    visuals.widgets.active.bg_stroke = Stroke::new(1.0, accent);
    visuals.widgets.active.fg_stroke = Stroke::new(1.0, Color32::from_rgb(220, 227, 239));
    visuals.widgets.active.expansion = 0.1;

    visuals.widgets.open.bg_fill = Color32::from_rgb(63, 85, 77);
    visuals.widgets.open.weak_bg_fill = Color32::from_rgb(63, 85, 77);
    visuals.widgets.open.bg_stroke = Stroke::new(1.5, Color32::from_rgb(152, 195, 121));
    visuals
}

pub fn configure_fonts(ctx: &eframe::egui::Context, _language: &str) {
    let mut fonts = FontDefinitions::default();

    if let Some(cjk_bytes) = load_cjk_font_bytes() {
        fonts.font_data.insert(
            "noto_sans_cjk".to_string(),
            std::sync::Arc::new(FontData::from_owned(cjk_bytes)),
        );
        if let Some(family) = fonts.families.get_mut(&FontFamily::Proportional) {
            family.insert(0, "noto_sans_cjk".to_string());
        }
        if let Some(family) = fonts.families.get_mut(&FontFamily::Monospace) {
            family.insert(0, "noto_sans_cjk".to_string());
        }
    }

    ctx.set_fonts(fonts);
}

fn load_cjk_font_bytes() -> Option<Vec<u8>> {
    const CANDIDATES: &[&str] = &[
        "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
        "/usr/share/fonts/opentype/noto/NotoSansCJK-Bold.ttc",
    ];
    for path in CANDIDATES {
        if let Ok(bytes) = fs::read(path) {
            return Some(bytes);
        }
    }
    None
}
