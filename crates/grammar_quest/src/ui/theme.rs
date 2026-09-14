pub fn apply(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();
    visuals.panel_fill = egui::Color32::from_rgb(15, 9, 25);
    visuals.window_fill = egui::Color32::from_rgb(24, 14, 40);
    visuals.selection.bg_fill = egui::Color32::from_rgb(211, 70, 255);
    visuals.widgets.active.bg_fill = egui::Color32::from_rgb(53, 220, 255);
    visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(42, 76, 126);
    visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(27, 19, 43);
    ctx.set_visuals(visuals);
}
