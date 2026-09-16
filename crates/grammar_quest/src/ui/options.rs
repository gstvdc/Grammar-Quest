use crate::state::AppState;
use crate::ui::theme::{
    RETRO_GOLD as GOLD, RETRO_PARCHMENT as PARCHMENT, RETRO_PLUM as PLUM, RETRO_RED as RED,
    animated_button, retro_panel,
};

const ATTRIBUTION: &str = include_str!("../../assets/ATTRIBUTION.md");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptionsAction {
    None,
    Back,
}

pub fn show_options(ctx: &egui::Context, state: &mut AppState) -> OptionsAction {
    let mut action = OptionsAction::None;
    egui::CentralPanel::default()
        .frame(
            egui::Frame::central_panel(&ctx.style())
                .fill(egui::Color32::TRANSPARENT)
                .inner_margin(egui::Margin::same(80)),
        )
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                retro_panel(ui, egui::Margin::symmetric(46, 34), |ui| {
                    ui.set_width(520.0);
                    ui.vertical_centered(|ui| {
                        ui.label(
                            egui::RichText::new("OPÇÕES")
                                .size(26.0)
                                .strong()
                                .color(GOLD),
                        );
                        ui.add_space(8.0);
                        ui.label(
                            egui::RichText::new("Ajuste sua jornada")
                                .size(13.0)
                                .color(PARCHMENT),
                        );
                    });
                    ui.add_space(24.0);

                    section_card(ui, |ui| show_volume(ui, state));
                    ui.add_space(14.0);
                    section_card(ui, |ui| show_display(ui, state));
                    ui.add_space(14.0);
                    section_card(ui, show_credits);

                    ui.add_space(22.0);
                    ui.vertical_centered(|ui| {
                        if animated_button(
                            ui,
                            "options_back",
                            egui::vec2(300.0, 42.0),
                            "← VOLTAR",
                            14.0,
                            PARCHMENT,
                            egui::Color32::from_rgb(35, 22, 60),
                        )
                        .clicked()
                        {
                            action = OptionsAction::Back;
                        }
                    });
                });
            });
        });
    action
}

fn section_card(ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::NONE
        .fill(egui::Color32::from_rgb(20, 13, 36))
        .stroke(egui::Stroke::new(
            1.0_f32,
            egui::Color32::from_rgb(120, 78, 170),
        ))
        .corner_radius(egui::CornerRadius::same(2))
        .inner_margin(egui::Margin::same(18))
        .show(ui, add_contents);
}

fn show_volume(ui: &mut egui::Ui, state: &mut AppState) {
    ui.label(egui::RichText::new("VOLUME").strong().color(PARCHMENT));
    ui.add_space(6.0);
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(format!("{}%", (state.master_volume * 100.0).round()))
                .size(24.0)
                .strong()
                .color(GOLD),
        );
        ui.label(
            egui::RichText::new("volume mestre").color(egui::Color32::from_rgb(160, 140, 200)),
        );
    });
    ui.add_space(10.0);
    ui.scope(|ui| {
        ui.spacing_mut().slider_width = ui.available_width();
        ui.style_mut().visuals.selection.bg_fill = GOLD;
        ui.style_mut().visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(53, 34, 84);
        ui.style_mut().visuals.widgets.inactive.fg_stroke = egui::Stroke::new(2.0_f32, GOLD);
        ui.style_mut().visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(77, 51, 114);
        ui.style_mut().visuals.widgets.hovered.fg_stroke = egui::Stroke::new(2.5_f32, GOLD);
        ui.style_mut().visuals.widgets.active.bg_fill = GOLD;
        ui.style_mut().visuals.widgets.active.fg_stroke =
            egui::Stroke::new(2.5_f32, egui::Color32::from_rgb(35, 22, 60));

        let mut volume = state.master_volume;
        if ui
            .add(
                egui::Slider::new(&mut volume, 0.0..=1.0)
                    .step_by(0.05)
                    .show_value(false)
                    .trailing_fill(true),
            )
            .on_hover_cursor(egui::CursorIcon::PointingHand)
            .changed()
        {
            state.set_master_volume(volume);
        }
    });
    ui.add_space(10.0);
    ui.horizontal(|ui| {
        for level in [0.0, 0.25, 0.5, 0.75, 1.0] {
            let selected = (state.master_volume - level).abs() < 0.025;
            let response = ui.add_sized(
                egui::vec2(82.0, 30.0),
                egui::Button::new(
                    egui::RichText::new(format!("{:.0}%", level * 100.0))
                        .strong()
                        .color(if selected {
                            egui::Color32::from_rgb(26, 15, 46)
                        } else {
                            PARCHMENT
                        }),
                )
                .fill(if selected { GOLD } else { PLUM })
                .stroke(egui::Stroke::new(
                    1.0_f32,
                    if selected { RED } else { GOLD },
                ))
                .corner_radius(egui::CornerRadius::same(2)),
            );
            if response
                .on_hover_cursor(egui::CursorIcon::PointingHand)
                .clicked()
            {
                state.set_master_volume(level);
            }
        }
    });
}

fn show_display(ui: &mut egui::Ui, state: &mut AppState) {
    ui.label(egui::RichText::new("EXIBIÇÃO").strong().color(PARCHMENT));
    ui.add_space(10.0);
    let mut fullscreen = state.fullscreen;
    if fullscreen_checkbox(ui, &mut fullscreen).changed() {
        state.set_fullscreen(fullscreen);
    }
    ui.label(
        egui::RichText::new(if state.fullscreen {
            "O jogo ocupa toda a tela."
        } else {
            "O jogo será exibido em uma janela."
        })
        .color(egui::Color32::from_rgb(160, 140, 200)),
    );
}

fn fullscreen_checkbox(ui: &mut egui::Ui, fullscreen: &mut bool) -> egui::Response {
    let desired_size = egui::vec2(ui.available_width(), 38.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    response = response.on_hover_cursor(egui::CursorIcon::PointingHand);

    if response.clicked() {
        *fullscreen = !*fullscreen;
        response.mark_changed();
    }

    let border = if response.hovered() { PARCHMENT } else { GOLD };
    let fill = if response.hovered() {
        egui::Color32::from_rgb(61, 40, 92)
    } else {
        egui::Color32::from_rgb(35, 22, 60)
    };
    ui.painter()
        .rect_filled(rect, egui::CornerRadius::same(2), fill);
    ui.painter().rect_stroke(
        rect,
        egui::CornerRadius::same(2),
        egui::Stroke::new(1.5_f32, border),
        egui::StrokeKind::Inside,
    );

    let box_rect = egui::Rect::from_min_size(
        rect.left_top() + egui::vec2(12.0, 8.0),
        egui::vec2(22.0, 22.0),
    );
    ui.painter().rect_filled(
        box_rect,
        egui::CornerRadius::same(1),
        if *fullscreen { GOLD } else { PLUM },
    );
    ui.painter().rect_stroke(
        box_rect,
        egui::CornerRadius::same(1),
        egui::Stroke::new(2.0_f32, GOLD),
        egui::StrokeKind::Inside,
    );
    if *fullscreen {
        ui.painter().text(
            box_rect.center(),
            egui::Align2::CENTER_CENTER,
            "✓",
            egui::FontId::proportional(17.0),
            egui::Color32::from_rgb(35, 22, 60),
        );
    }
    ui.painter().text(
        box_rect.right_center() + egui::vec2(12.0, 0.0),
        egui::Align2::LEFT_CENTER,
        "Tela cheia",
        egui::FontId::proportional(15.0),
        PARCHMENT,
    );

    response
}

fn show_credits(ui: &mut egui::Ui) {
    ui.label(egui::RichText::new("CRÉDITOS").strong().color(PARCHMENT));
    ui.add_space(6.0);
    ui.label(egui::RichText::new("Desenvolvido por").color(egui::Color32::from_rgb(160, 140, 200)));
    ui.label(
        egui::RichText::new("Gustavo da Cunha Constante")
            .size(17.0)
            .strong()
            .color(GOLD),
    );
    ui.add_space(8.0);
    egui::CollapsingHeader::new(
        egui::RichText::new("Licenças dos recursos").color(egui::Color32::from_rgb(180, 160, 220)),
    )
    .show(ui, |ui| {
        egui::ScrollArea::vertical()
            .max_height(110.0)
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new(ATTRIBUTION)
                        .monospace()
                        .color(egui::Color32::from_rgb(180, 160, 220)),
                );
            });
    });
}
