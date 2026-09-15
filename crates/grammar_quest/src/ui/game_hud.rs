use crate::state::{AppState, ScreenMode};
use grammar_engine::Symbol;

pub enum HudAction {
    None,
    BackToLab,
    PlayAgain,
}

pub fn show_hud(ctx: &egui::Context, state: &mut AppState) -> HudAction {
    let mut action = HudAction::None;

    // Sleek floating glassmorphism top bar
    egui::TopBottomPanel::top("maze_hud_bar")
        .frame(
            egui::Frame::side_top_panel(&ctx.style())
                .fill(egui::Color32::from_rgba_premultiplied(10, 7, 20, 240))
                .stroke(egui::Stroke::new(
                    1.0_f32,
                    egui::Color32::from_rgb(45, 28, 75),
                ))
                .inner_margin(egui::Margin::symmetric(16, 10)),
        )
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Game brand
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("⚡ GRAMMAR QUEST")
                            .color(egui::Color32::from_rgb(0, 240, 255))
                            .strong()
                            .size(15.0),
                    );
                    ui.label(
                        egui::RichText::new("LABIRINTO 2D")
                            .color(egui::Color32::from_rgb(140, 110, 190))
                            .size(11.0),
                    );
                });

                ui.add_space(16.0);
                ui.separator();
                ui.add_space(12.0);

                // Word generated so far
                let output = state
                    .derivation_state
                    .as_ref()
                    .map_or("", |s| s.output.as_str());

                ui.label(
                    egui::RichText::new("PALAVRA:")
                        .color(egui::Color32::from_rgb(160, 140, 200))
                        .size(12.0),
                );

                egui::Frame::NONE
                    .fill(egui::Color32::from_rgb(16, 28, 38))
                    .stroke(egui::Stroke::new(
                        1.0_f32,
                        egui::Color32::from_rgb(0, 240, 255),
                    ))
                    .corner_radius(egui::CornerRadius::same(6))
                    .inner_margin(egui::Margin::symmetric(10, 4))
                    .show(ui, |ui| {
                        ui.monospace(
                            egui::RichText::new(if output.is_empty() { "ε" } else { output })
                                .size(16.0)
                                .color(egui::Color32::from_rgb(52, 255, 180))
                                .strong(),
                        );
                    });

                ui.add_space(12.0);
                ui.separator();
                ui.add_space(12.0);

                // Stack symbols visualization (chips)
                ui.label(
                    egui::RichText::new("PILHA:")
                        .color(egui::Color32::from_rgb(160, 140, 200))
                        .size(12.0),
                );

                if state.stack_animation_progress().is_some() {
                    ui.label(
                        egui::RichText::new("↑ PUSH")
                            .color(egui::Color32::from_rgb(250, 204, 21))
                            .strong()
                            .size(10.0),
                    );
                }

                let snapshot = state.visual_stack_snapshot();
                let symbol_alpha = state
                    .stack_animation_progress()
                    .map_or(255, |progress| (progress * 255.0) as u8);
                if state.derivation_state.is_some() {
                    if snapshot.is_empty() {
                        egui::Frame::NONE
                            .fill(egui::Color32::from_rgb(35, 28, 12))
                            .stroke(egui::Stroke::new(
                                1.0_f32,
                                egui::Color32::from_rgb(250, 204, 21),
                            ))
                            .corner_radius(egui::CornerRadius::same(6))
                            .inner_margin(egui::Margin::symmetric(8, 3))
                            .show(ui, |ui| {
                                ui.label(
                                    egui::RichText::new("✨ PILHA VAZIA")
                                        .color(egui::Color32::from_rgb(250, 204, 21))
                                        .size(12.0)
                                        .strong(),
                                );
                            });
                    } else {
                        ui.horizontal(|ui| {
                            for (idx, sym) in snapshot.iter().enumerate() {
                                if idx > 0 {
                                    ui.label(
                                        egui::RichText::new("·")
                                            .color(egui::Color32::from_rgb(100, 80, 140)),
                                    );
                                }
                                match sym {
                                    Symbol::NonTerminal(name) => {
                                        egui::Frame::NONE
                                            .fill(egui::Color32::from_rgb(45, 20, 70))
                                            .stroke(egui::Stroke::new(
                                                1.0_f32,
                                                egui::Color32::from_rgb(192, 132, 252),
                                            ))
                                            .corner_radius(egui::CornerRadius::same(4))
                                            .inner_margin(egui::Margin::symmetric(6, 2))
                                            .show(ui, |ui| {
                                                ui.monospace(
                                                    egui::RichText::new(name)
                                                        .color(
                                                            egui::Color32::from_rgba_unmultiplied(
                                                                238,
                                                                166,
                                                                255,
                                                                symbol_alpha,
                                                            ),
                                                        )
                                                        .strong(),
                                                );
                                            });
                                    }
                                    Symbol::Terminal(ch) => {
                                        egui::Frame::NONE
                                            .fill(egui::Color32::from_rgb(15, 35, 45))
                                            .stroke(egui::Stroke::new(
                                                1.0_f32,
                                                egui::Color32::from_rgb(0, 240, 255),
                                            ))
                                            .corner_radius(egui::CornerRadius::same(4))
                                            .inner_margin(egui::Margin::symmetric(6, 2))
                                            .show(ui, |ui| {
                                                ui.monospace(
                                                    egui::RichText::new(ch.to_string())
                                                        .color(
                                                            egui::Color32::from_rgba_unmultiplied(
                                                                74,
                                                                229,
                                                                255,
                                                                symbol_alpha,
                                                            ),
                                                        )
                                                        .strong(),
                                                );
                                            });
                                    }
                                }
                            }
                        });
                    }
                }

                // Right side controls
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .add(
                            egui::Button::new("⚙ Laboratório [Esc]")
                                .fill(egui::Color32::from_rgb(26, 17, 44)),
                        )
                        .clicked()
                    {
                        action = HudAction::BackToLab;
                    }

                    let side_text = if state.show_side_panel {
                        "📖 Ocultar Trilha"
                    } else {
                        "📖 Ver Trilha"
                    };
                    if ui
                        .add(egui::Button::new(side_text).fill(if state.show_side_panel {
                            egui::Color32::from_rgb(50, 25, 80)
                        } else {
                            egui::Color32::from_rgb(26, 17, 44)
                        }))
                        .clicked()
                    {
                        state.show_side_panel = !state.show_side_panel;
                    }
                });
            });
        });

    // Victory celebration modal
    if state.mode == ScreenMode::Won {
        egui::Window::new("Vitória")
            .collapsible(false)
            .resizable(false)
            .title_bar(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .frame(
                egui::Frame::window(&ctx.style())
                    .fill(egui::Color32::from_rgb(16, 11, 30))
                    .stroke(egui::Stroke::new(
                        2.0_f32,
                        egui::Color32::from_rgb(52, 255, 180),
                    ))
                    .corner_radius(egui::CornerRadius::same(16))
                    .inner_margin(egui::Margin::same(24)),
            )
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(
                        egui::RichText::new("🏆 VITÓRIA NO LABIRINTO!")
                            .size(24.0)
                            .color(egui::Color32::from_rgb(52, 255, 180))
                            .strong(),
                    );
                    ui.add_space(4.0);
                    ui.label(
                        egui::RichText::new("A derivação formal foi concluída e a pilha esvaziou.")
                            .color(egui::Color32::from_rgb(180, 160, 220))
                            .size(13.0),
                    );
                    if let Some(score) = state.score {
                        ui.add_space(8.0);
                        ui.label(
                            egui::RichText::new(format!("PONTUAÇÃO: {score}"))
                                .color(egui::Color32::from_rgb(250, 204, 21))
                                .strong()
                                .size(18.0),
                        );
                    }

                    ui.add_space(20.0);

                    if let Some(res) = &state.result {
                        egui::Frame::NONE
                            .fill(egui::Color32::from_rgb(10, 6, 20))
                            .stroke(egui::Stroke::new(
                                1.0_f32,
                                egui::Color32::from_rgb(55, 35, 95),
                            ))
                            .corner_radius(egui::CornerRadius::same(12))
                            .inner_margin(egui::Margin::symmetric(24, 16))
                            .show(ui, |ui| {
                                ui.label(
                                    egui::RichText::new("Sentença Gerada:")
                                        .color(egui::Color32::from_rgb(160, 140, 200))
                                        .size(13.0),
                                );
                                ui.label(
                                    egui::RichText::new(&res.sentence)
                                        .size(44.0)
                                        .color(egui::Color32::from_rgb(0, 240, 255))
                                        .strong(),
                                );
                                ui.add_space(10.0);
                                ui.horizontal(|ui| {
                                    ui.label(
                                        egui::RichText::new("Expressão Regular:")
                                            .color(egui::Color32::from_rgb(160, 140, 200)),
                                    );
                                    ui.monospace(
                                        egui::RichText::new(&res.regex)
                                            .size(16.0)
                                            .color(egui::Color32::from_rgb(238, 166, 255))
                                            .strong(),
                                    );
                                });
                                ui.add_space(4.0);
                                ui.label(
                                    egui::RichText::new(format!(
                                        "{} passos de expansão aplicados pelo jogador",
                                        res.steps.len()
                                    ))
                                    .color(egui::Color32::from_rgb(130, 110, 170))
                                    .size(12.0),
                                );
                            });
                    }

                    ui.add_space(22.0);

                    ui.horizontal(|ui| {
                        if ui
                            .add_sized(
                                [170.0, 42.0],
                                egui::Button::new(
                                    egui::RichText::new("🔄 Jogar Novamente")
                                        .color(egui::Color32::from_rgb(10, 6, 20))
                                        .strong()
                                        .size(14.0),
                                )
                                .fill(egui::Color32::from_rgb(52, 255, 180))
                                .corner_radius(egui::CornerRadius::same(8)),
                            )
                            .clicked()
                        {
                            action = HudAction::PlayAgain;
                        }

                        ui.add_space(12.0);

                        if ui
                            .add_sized(
                                [170.0, 42.0],
                                egui::Button::new(
                                    egui::RichText::new("⚙ Voltar ao Laboratório")
                                        .color(egui::Color32::WHITE)
                                        .size(14.0),
                                )
                                .fill(egui::Color32::from_rgb(35, 22, 60))
                                .corner_radius(egui::CornerRadius::same(8)),
                            )
                            .clicked()
                        {
                            action = HudAction::BackToLab;
                        }
                    });
                });
            });
    }

    action
}
