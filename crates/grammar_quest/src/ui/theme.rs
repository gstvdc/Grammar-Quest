pub fn apply(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();
    visuals.panel_fill = egui::Color32::from_rgb(15, 9, 25);
    visuals.window_fill = egui::Color32::from_rgb(28, 15, 48);
    visuals.selection.bg_fill = egui::Color32::from_rgb(211, 70, 255);
    visuals.widgets.active.bg_fill = egui::Color32::from_rgb(39, 208, 255);
    visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(83, 41, 125);
    visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(27, 19, 43);
    ctx.set_visuals(visuals);
}
