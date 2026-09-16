use crate::state::{AppState, GameFlow, ScreenMode};
use crate::ui::theme::animated_button;
use grammar_engine::Symbol;

pub enum HudAction {
    None,
    BackToLab,
    PlayAgain,
}

pub fn show_hud(ctx: &egui::Context, state: &mut AppState) -> HudAction {
    let mut action = HudAction::None;

    // Retro pixel HUD matching the menu and preparation screens.
    egui::TopBottomPanel::top("maze_hud_bar")
        .frame(
            egui::Frame::side_top_panel(&ctx.style())
                .fill(egui::Color32::from_rgba_premultiplied(45, 27, 78, 245))
                .stroke(egui::Stroke::new(
                    1.0_f32,
                    egui::Color32::from_rgb(255, 206, 97),
                ))
                .inner_margin(egui::Margin::symmetric(16, 10)),
        )
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Game brand
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("GRAMMAR QUEST")
                            .color(egui::Color32::from_rgb(255, 206, 97))
                            .strong()
                            .size(15.0),
                    );
                    ui.label(
                        egui::RichText::new("LABIRINTO 2D")
                            .color(egui::Color32::from_rgb(255, 230, 158))
                            .size(11.0),
                    );
                });

                ui.add_space(16.0);
                ui.separator();
                ui.add_space(12.0);

                match state.active_flow {
                    Some(GameFlow::Free) => {
                        ui.label(
                            egui::RichText::new("LABIRINTO LIVRE · DERIVAÇÃO POR PILHA")
                                .color(egui::Color32::from_rgb(255, 230, 158))
                                .size(11.0)
                                .strong(),
                        );
                    }
                    Some(GameFlow::SecretChallenge) => {
                        let (min_steps, max_steps) = state.selected_difficulty.step_range();
                        ui.label(
                            egui::RichText::new(format!(
                                "{} · PORTAS {}/{} · REINÍCIOS {}",
                                state.selected_difficulty.label(),
                                state.secret_progress(),
                                state.secret_total_steps(),
                                state.restart_count
                            ))
                            .color(egui::Color32::from_rgb(250, 204, 21))
                            .size(11.0)
                            .strong(),
                        );
                        ui.label(
                            egui::RichText::new(format!("META: {min_steps}–{max_steps}"))
                                .color(egui::Color32::from_rgb(140, 110, 190))
                                .size(10.0),
                        );
                    }
                    None => {}
                }

                // Right side controls
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if animated_button(
                        ui,
                        "hud_back_to_lab",
                        egui::vec2(180.0, 28.0),
                        "← Preparação [Esc]",
                        12.0,
                        egui::Color32::from_rgb(26, 15, 46),
                        egui::Color32::from_rgb(255, 206, 97),
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
                    let side_fill = if state.show_side_panel {
                        egui::Color32::from_rgb(50, 25, 80)
                    } else {
                        egui::Color32::from_rgb(26, 17, 44)
                    };
                    if animated_button(
                        ui,
                        "hud_toggle_side_panel",
                        egui::vec2(160.0, 28.0),
                        side_text,
                        12.0,
                        egui::Color32::from_rgb(215, 200, 245),
                        side_fill,
                    )
                    .clicked()
                    {
                        state.show_side_panel = !state.show_side_panel;
                    }
                });
            });
        });

    show_vertical_stack(ctx, state);

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
                ui.set_min_width(360.0);
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
                                        egui::RichText::new(crate::ui::display_regex(&res.regex))
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

                    ui.vertical_centered(|ui| {
                        if animated_button(
                            ui,
                            "victory_play_again",
                            egui::vec2(300.0, 42.0),
                            "▶ JOGAR NOVAMENTE",
                            14.0,
                            egui::Color32::from_rgb(10, 6, 20),
                            egui::Color32::from_rgb(52, 255, 180),
                        )
                        .clicked()
                        {
                            action = HudAction::PlayAgain;
                        }

                        ui.add_space(10.0);

                        if animated_button(
                            ui,
                            "victory_back_to_lab",
                            egui::vec2(300.0, 42.0),
                            "← VOLTAR AO LABORATÓRIO",
                            14.0,
                            egui::Color32::WHITE,
                            egui::Color32::from_rgb(35, 22, 60),
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

/// Live stack readout, docked at the bottom of the formal-trace column.
/// Rendered top-to-bottom in `snapshot_top_first` order, so the visual
/// top of the column really is the top of the stack — the one place in the
/// UI where the metaphor and the picture must not diverge.
fn show_vertical_stack(ctx: &egui::Context, state: &AppState) {
    if state.derivation_state.is_none() {
        return;
    }

    let snapshot = state.visual_stack_snapshot();

    egui::Area::new(egui::Id::new("live_stack_panel"))
        .anchor(egui::Align2::LEFT_BOTTOM, egui::vec2(16.0, -16.0))
        .show(ctx, |ui| {
            egui::Frame::NONE
                .fill(egui::Color32::from_rgba_premultiplied(10, 7, 20, 235))
                .stroke(egui::Stroke::new(
                    1.0_f32,
                    egui::Color32::from_rgb(45, 28, 75),
                ))
                .corner_radius(egui::CornerRadius::same(8))
                .inner_margin(egui::Margin::symmetric(10, 10))
                .show(ui, |ui| {
                    ui.set_min_width(120.0);
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new("PILHA")
                                .color(egui::Color32::from_rgb(160, 140, 200))
                                .strong()
                                .size(12.0),
                        );
                        ui.add_space(8.0);

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
                                    ui.add_sized(
                                        egui::vec2(104.0, 18.0),
                                        egui::Label::new(
                                            egui::RichText::new("✨ VAZIA")
                                                .color(egui::Color32::from_rgb(250, 204, 21))
                                                .size(12.0)
                                                .strong(),
                                        ),
                                    );
                                });
                        } else {
                            for (idx, sym) in snapshot.iter().enumerate() {
                                let (fill, border, text_rgb) = match sym {
                                    Symbol::NonTerminal(_) => (
                                        egui::Color32::from_rgb(45, 20, 70),
                                        egui::Color32::from_rgb(192, 132, 252),
                                        (238, 166, 255),
                                    ),
                                    Symbol::Terminal(_) => (
                                        egui::Color32::from_rgb(15, 35, 45),
                                        egui::Color32::from_rgb(0, 240, 255),
                                        (74, 229, 255),
                                    ),
                                };
                                egui::Frame::NONE
                                    .fill(fill)
                                    .stroke(egui::Stroke::new(1.0_f32, border))
                                    .corner_radius(egui::CornerRadius::same(4))
                                    .inner_margin(egui::Margin::symmetric(8, 3))
                                    .show(ui, |ui| {
                                        ui.with_layout(
                                            egui::Layout::top_down(egui::Align::Center),
                                            |ui| {
                                                ui.add_sized(
                                                    egui::vec2(104.0, 18.0),
                                                    egui::Label::new(
                                                        egui::RichText::new(sym.to_string())
                                                            .color(egui::Color32::from_rgb(
                                                                text_rgb.0, text_rgb.1, text_rgb.2,
                                                            ))
                                                            .strong(),
                                                    ),
                                                );
                                            },
                                        );
                                    });
                                if idx + 1 < snapshot.len() {
                                    ui.label(
                                        egui::RichText::new("│")
                                            .color(egui::Color32::from_rgb(100, 80, 140)),
                                    );
                                }
                            }
                        }
                    });
                });
        });
}
