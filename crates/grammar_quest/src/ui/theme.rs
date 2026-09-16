pub const RETRO_PLUM: egui::Color32 = egui::Color32::from_rgb(45, 27, 78);
pub const RETRO_GOLD: egui::Color32 = egui::Color32::from_rgb(255, 206, 97);
pub const RETRO_RED: egui::Color32 = egui::Color32::from_rgb(148, 33, 51);
pub const RETRO_PARCHMENT: egui::Color32 = egui::Color32::from_rgb(255, 230, 158);

/// The "Derivation Terminal" identity treats every piece of UI text as
/// instrument readout, not prose — so there is deliberately one voice
/// (a technical monospace) for both display and body text, not a
/// proportional/monospace split. Reads from the OS-installed SF Mono
/// (present on every shipping macOS version this app targets, per
/// `scripts/build-macos-app.sh`) rather than bundling a font file, since
/// nothing here is redistributed — only referenced at runtime. Falls back
/// to egui's stock fonts on any platform where the path is absent.
fn install_terminal_font(ctx: &egui::Context) {
    static INSTALLED: std::sync::Once = std::sync::Once::new();
    INSTALLED.call_once(|| {
        let Ok(bytes) = std::fs::read("/System/Library/Fonts/SFNSMono.ttf") else {
            return;
        };

        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "terminal_mono".to_owned(),
            egui::FontData::from_owned(bytes).into(),
        );

        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "terminal_mono".to_owned());
        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .insert(0, "terminal_mono".to_owned());

        ctx.set_fonts(fonts);
    });
}

pub fn apply(ctx: &egui::Context) {
    install_terminal_font(ctx);

    let mut visuals = egui::Visuals::dark();

    // Deep modern cyberpunk / synthwave palette
    visuals.panel_fill = egui::Color32::from_rgb(11, 8, 20);
    visuals.window_fill = egui::Color32::from_rgb(18, 12, 32);
    visuals.window_stroke = egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(74, 45, 110));
    visuals.window_corner_radius = egui::CornerRadius::same(12);

    visuals.selection.bg_fill = egui::Color32::from_rgb(192, 132, 252);
    visuals.selection.stroke = egui::Stroke::new(1.0_f32, egui::Color32::WHITE);

    // Active widgets (buttons clicked / focused)
    visuals.widgets.active.bg_fill = egui::Color32::from_rgb(0, 240, 255);
    visuals.widgets.active.fg_stroke =
        egui::Stroke::new(1.5_f32, egui::Color32::from_rgb(11, 8, 20));
    visuals.widgets.active.corner_radius = egui::CornerRadius::same(8);

    // Hovered widgets — slightly expanded + brighter stroke so built-in
    // widgets (combobox items, checkboxes) still read as "alive" even
    // though they don't go through `animated_button`.
    visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(70, 38, 108);
    visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.5_f32, egui::Color32::WHITE);
    visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(8);
    visuals.widgets.hovered.expansion = 1.5_f32;

    // Inactive / default widgets
    visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(26, 17, 44);
    visuals.widgets.inactive.fg_stroke =
        egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(215, 200, 245));
    visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(8);

    // Non-interactive surfaces
    visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(18, 12, 32);
    visuals.widgets.noninteractive.fg_stroke =
        egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(160, 140, 200));
    visuals.widgets.noninteractive.corner_radius = egui::CornerRadius::same(8);

    // Extreme backgrounds (text boxes, code areas)
    visuals.extreme_bg_color = egui::Color32::from_rgb(8, 5, 15);

    ctx.set_visuals(visuals);
}

fn lerp_color(a: egui::Color32, b: egui::Color32, t: f32) -> egui::Color32 {
    let t = t.clamp(0.0, 1.0);
    let lerp_channel = |x: u8, y: u8| (x as f32 + (y as f32 - x as f32) * t).round() as u8;
    egui::Color32::from_rgba_unmultiplied(
        lerp_channel(a.r(), b.r()),
        lerp_channel(a.g(), b.g()),
        lerp_channel(a.b(), b.b()),
        lerp_channel(a.a(), b.a()),
    )
}

/// A button painted by hand (instead of `egui::Button`) so hover/press can
/// be smoothly animated — grow + glow on hover, a small "sink" on press —
/// which the stock immediate-mode button only renders as an instant color
/// swap. `salt` must be unique per call site (it seeds the persistent
/// animation id); size is fixed rather than auto-computed from `label`,
/// matching how every call site already used `add_sized`.
pub fn animated_button(
    ui: &mut egui::Ui,
    salt: &str,
    size: egui::Vec2,
    label: &str,
    font_size: f32,
    text_color: egui::Color32,
    fill: egui::Color32,
) -> egui::Response {
    let id = ui.make_persistent_id(salt);
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());
    let response = response.on_hover_cursor(egui::CursorIcon::PointingHand);

    let hover_t = ui.ctx().animate_value_with_time(
        id.with("hover"),
        if response.hovered() { 1.0 } else { 0.0 },
        0.12,
    );
    let press_t = ui.ctx().animate_value_with_time(
        id.with("press"),
        if response.is_pointer_button_down_on() {
            1.0
        } else {
            0.0
        },
        0.05,
    );

    if ui.is_rect_visible(rect) {
        let lift = hover_t * 3.0 - press_t * 2.0;
        let grow = hover_t * 3.0 - press_t * 1.5;
        let painted_rect = rect.expand(grow).translate(egui::vec2(0.0, -lift));
        let corner_radius = egui::CornerRadius::same(8);

        let glow_alpha = (hover_t * 70.0) as u8;
        if glow_alpha > 0 {
            ui.painter().rect_filled(
                painted_rect.expand(5.0),
                egui::CornerRadius::same(12),
                egui::Color32::from_rgba_unmultiplied(fill.r(), fill.g(), fill.b(), glow_alpha),
            );
        }

        let lightened = lerp_color(fill, egui::Color32::WHITE, hover_t * 0.18);
        let painted_fill = lerp_color(lightened, egui::Color32::BLACK, press_t * 0.25);
        ui.painter()
            .rect_filled(painted_rect, corner_radius, painted_fill);
        ui.painter().rect_stroke(
            painted_rect,
            corner_radius,
            egui::Stroke::new(
                1.0_f32,
                lerp_color(
                    egui::Color32::TRANSPARENT,
                    egui::Color32::WHITE,
                    hover_t * 0.6,
                ),
            ),
            egui::StrokeKind::Outside,
        );

        ui.painter().text(
            painted_rect.center(),
            egui::Align2::CENTER_CENTER,
            label,
            egui::FontId::proportional(font_size),
            text_color,
        );
    }

    response
}

pub fn retro_panel(
    ui: &mut egui::Ui,
    inner_margin: egui::Margin,
    add_contents: impl FnOnce(&mut egui::Ui),
) {
    egui::Frame::NONE
        .fill(RETRO_PLUM)
        .stroke(egui::Stroke::new(3.0_f32, RETRO_GOLD))
        .shadow(egui::epaint::Shadow {
            offset: [6, 6],
            blur: 0,
            spread: 0,
            color: RETRO_RED,
        })
        .corner_radius(egui::CornerRadius::same(4))
        .inner_margin(inner_margin)
        .show(ui, add_contents);
}
