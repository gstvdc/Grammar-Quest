use crate::state::{AppState, ScreenMode};

pub enum HudAction {
    None,
    BackToLab,
    PlayAgain,
}

pub fn show_hud(ctx: &egui::Context, state: &mut AppState) -> HudAction {
    let mut action = HudAction::None;

    egui::TopBottomPanel::top("maze_hud_bar")
        .frame(
            egui::Frame::side_top_panel(&ctx.style())
                .fill(egui::Color32::from_rgba_premultiplied(13, 7, 24, 230))
                .inner_margin(12.0),
        )
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("⚔ GRAMMAR MAZE")
                        .color(egui::Color32::from_rgb(74, 229, 255))
                        .strong(),
                );
                ui.separator();

                let output = state
                    .derivation_state
                    .as_ref()
                    .map_or("", |s| s.output.as_str());
                ui.label("Saída:");
                ui.monospace(
                    egui::RichText::new(if output.is_empty() { "ε" } else { output })
                        .size(18.0)
                        .color(egui::Color32::from_rgb(116, 255, 191))
                        .strong(),
                );

                ui.separator();

                let stack_str = state.derivation_state.as_ref().map_or_else(
                    || "∅".to_string(),
                    |s| {
                        let syms: Vec<String> = s
                            .stack
                            .snapshot_top_first()
                            .iter()
                            .map(ToString::to_string)
                            .collect();
                        if syms.is_empty() {
                            "∅ (pilha vazia)".to_string()
                        } else {
                            syms.join(" · ")
                        }
                    },
                );

                ui.label("Pilha:");
                ui.monospace(
                    egui::RichText::new(stack_str)
                        .size(16.0)
                        .color(egui::Color32::from_rgb(238, 166, 255)),
                );

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("⚙ Laboratório [Esc]").clicked() {
                        action = HudAction::BackToLab;
                    }
                    let side_text = if state.show_side_panel {
                        "📖 Ocultar Trilha"
                    } else {
                        "📖 Ver Trilha"
                    };
                    if ui.button(side_text).clicked() {
                        state.show_side_panel = !state.show_side_panel;
                    }
                });
            });
        });

    if state.mode == ScreenMode::Won {
        egui::Window::new("Vitória!")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .frame(
                egui::Frame::window(&ctx.style())
                    .fill(egui::Color32::from_rgb(22, 14, 42))
                    .stroke(egui::Stroke::new(
                        2.0_f32,
                        egui::Color32::from_rgb(116, 255, 191),
                    )),
            )
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(8.0);
                    ui.label(
                        egui::RichText::new("★ DERIVAÇÃO CONCLUÍDA NO LABIRINTO! ★")
                            .size(20.0)
                            .color(egui::Color32::from_rgb(116, 255, 191))
                            .strong(),
                    );
                    ui.add_space(8.0);
                    ui.label("A pilha de símbolos foi completamente esvaziada!");

                    if let Some(res) = &state.result {
                        ui.add_space(14.0);
                        ui.label("Sentença final gerada:");
                        ui.label(
                            egui::RichText::new(&res.sentence)
                                .size(42.0)
                                .color(egui::Color32::from_rgb(74, 229, 255))
                                .strong(),
                        );
                        ui.add_space(10.0);
                        ui.label("Expressão regular da linguagem:");
                        ui.monospace(
                            egui::RichText::new(&res.regex)
                                .size(18.0)
                                .color(egui::Color32::from_rgb(238, 166, 255)),
                        );
                        ui.add_space(6.0);
                        ui.label(format!("Passos de derivação: {}", res.steps.len()));
                    }

                    ui.add_space(18.0);
                    ui.horizontal(|ui| {
                        if ui
                            .add_sized([160.0, 36.0], egui::Button::new("Jogar novamente"))
                            .clicked()
                        {
                            action = HudAction::PlayAgain;
                        }
                        ui.add_space(12.0);
                        if ui
                            .add_sized([160.0, 36.0], egui::Button::new("Voltar ao Laboratório"))
                            .clicked()
                        {
                            action = HudAction::BackToLab;
                        }
                    });
                    ui.add_space(8.0);
                });
            });
    }

    action
}
