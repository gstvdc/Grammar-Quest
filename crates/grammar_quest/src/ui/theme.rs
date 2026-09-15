pub fn apply(ctx: &egui::Context) {
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

    // Hovered widgets
    visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(60, 32, 95);
    visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.5_f32, egui::Color32::WHITE);
    visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(8);

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
