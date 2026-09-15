use crate::state::{AppState, PanelResult};

pub fn show_result(ui: &mut egui::Ui, result: Option<&PanelResult>) {
    if let Some(result) = result {
        egui::Frame::NONE
            .fill(egui::Color32::from_rgb(18, 12, 34))
            .stroke(egui::Stroke::new(
                1.5_f32,
                egui::Color32::from_rgb(52, 255, 180),
            ))
            .corner_radius(egui::CornerRadius::same(16))
            .inner_margin(egui::Margin::same(28))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("✓ DERIVAÇÃO CONCLUÍDA")
                            .color(egui::Color32::from_rgb(52, 255, 180))
                            .strong()
                            .size(13.0),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            egui::RichText::new(format!("{} passos aplicados", result.steps.len()))
                                .color(egui::Color32::from_rgb(150, 130, 190))
                                .size(12.0),
                        );
                    });
                });

                ui.add_space(18.0);
                ui.label(
                    egui::RichText::new("Sentença Gerada pela Gramática:")
                        .color(egui::Color32::from_rgb(160, 140, 200))
                        .size(13.0),
                );
                ui.label(
                    egui::RichText::new(&result.sentence)
                        .size(48.0)
                        .color(egui::Color32::from_rgb(0, 240, 255))
                        .strong(),
                );

                ui.add_space(16.0);
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("Expressão Regular:")
                            .color(egui::Color32::from_rgb(160, 140, 200))
                            .size(13.0),
                    );
                    egui::Frame::NONE
                        .fill(egui::Color32::from_rgb(28, 16, 50))
                        .stroke(egui::Stroke::new(
                            1.0_f32,
                            egui::Color32::from_rgb(192, 132, 252),
                        ))
                        .corner_radius(egui::CornerRadius::same(6))
                        .inner_margin(egui::Margin::symmetric(10, 4))
                        .show(ui, |ui| {
                            ui.monospace(
                                egui::RichText::new(&result.regex)
                                    .size(17.0)
                                    .color(egui::Color32::from_rgb(238, 166, 255))
                                    .strong(),
                            );
                        });
                });

                ui.add_space(14.0);
                show_regex_trace(ui, &result.regex_trace);

                ui.add_space(14.0);
                ui.label(
                    egui::RichText::new(
                        "Clique em '▶ JOGAR NO LABIRINTO 2D' para derivar passo a passo controlando o personagem.",
                    )
                    .color(egui::Color32::from_rgb(130, 110, 170))
                    .size(12.0),
                );
            });
    } else {
        let logo_id = egui::Id::new("grammar_quest_brand_logo");
        let logo = ui
            .ctx()
            .data(|data| data.get_temp::<egui::TextureHandle>(logo_id))
            .or_else(|| {
                let image = macroquad::texture::Image::from_file_with_format(
                    include_bytes!("../../assets/brand/grammar-quest-logo.png"),
                    None,
                )
                .ok()?;
                let texture = ui.ctx().load_texture(
                    "grammar_quest_brand_logo",
                    egui::ColorImage::from_rgba_unmultiplied(
                        [image.width as usize, image.height as usize],
                        &image.bytes,
                    ),
                    egui::TextureOptions::NEAREST,
                );
                ui.ctx()
                    .data_mut(|data| data.insert_temp(logo_id, texture.clone()));
                Some(texture)
            });

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing = egui::vec2(24.0, 0.0);
            if let Some(logo) = logo {
                ui.add(
                    egui::Image::new((logo.id(), egui::vec2(96.0, 96.0)))
                        .alt_text("Logo do Grammar Quest"),
                );
            }
            ui.vertical(|ui| {
                ui.label(
                    egui::RichText::new("BEM-VINDO AO GRAMMAR QUEST")
                        .color(egui::Color32::from_rgb(0, 240, 255))
                        .strong()
                        .size(22.0),
                );
                ui.add_space(10.0);
                ui.label(
                    egui::RichText::new(
                        "Escolha um exemplo à esquerda ou digite produções de uma gramática regular.\nVocê pode testar a derivação no laboratório ou entrar no labirinto interativo!",
                    )
                    .color(egui::Color32::from_rgb(170, 150, 210))
                    .size(14.0),
                );
            });
        });
    }
}

/// Renders the professor's equation-elimination walkthrough (see
/// docs/audits/2026-09-15-project-audit.md, T3) behind a collapsible
/// header so the final regex above stays the headline result.
fn show_regex_trace(ui: &mut egui::Ui, trace: &grammar_engine::RegexTrace) {
    if trace.initial_equations.is_empty() {
        return;
    }
    egui::CollapsingHeader::new(
        egui::RichText::new("Equações e eliminação de variáveis")
            .color(egui::Color32::from_rgb(160, 140, 200))
            .size(13.0),
    )
    .default_open(false)
    .show(ui, |ui| {
        ui.label(
            egui::RichText::new("Equações iniciais:")
                .color(egui::Color32::from_rgb(140, 120, 180))
                .size(11.0),
        );
        for (_, equation) in &trace.initial_equations {
            ui.monospace(
                egui::RichText::new(equation)
                    .color(egui::Color32::from_rgb(238, 166, 255))
                    .size(12.0),
            );
        }

        ui.add_space(8.0);
        ui.label(
            egui::RichText::new("Eliminação de variáveis (regra de Arden):")
                .color(egui::Color32::from_rgb(140, 120, 180))
                .size(11.0),
        );
        for step in &trace.eliminations {
            ui.monospace(
                egui::RichText::new(format!(
                    "elimina {} → {}",
                    step.eliminated, step.resolved_equation
                ))
                .color(egui::Color32::from_rgb(52, 255, 180))
                .size(12.0),
            );
        }
    });
}

fn typewriter_prefix(text: &str, progress: f32) -> String {
    let visible = (text.chars().count() as f32 * progress.clamp(0.0, 1.0)).ceil() as usize;
    text.chars().take(visible).collect()
}

pub fn show_side_panel(ctx: &egui::Context, result: Option<&PanelResult>) {
    egui::SidePanel::right("derivation_trace")
        .resizable(false)
        .default_width(310.0)
        .frame(
            egui::Frame::side_top_panel(&ctx.style())
                .fill(egui::Color32::from_rgb(12, 8, 22))
                .stroke(egui::Stroke::new(
                    1.0_f32,
                    egui::Color32::from_rgb(45, 28, 75),
                ))
                .inner_margin(egui::Margin::symmetric(16, 14)),
        )
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("📖 TRILHA FORMAL")
                        .color(egui::Color32::from_rgb(192, 132, 252))
                        .strong()
                        .size(14.0),
                );
                if let Some(result) = result {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            egui::RichText::new(format!("{} passos", result.steps.len()))
                                .color(egui::Color32::from_rgb(140, 120, 180))
                                .size(12.0),
                        );
                    });
                }
            });
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(8.0);

            egui::ScrollArea::vertical().show(ui, |ui| {
                if let Some(result) = result {
                    let total_steps = result.steps.len();
                    let duration = (total_steps as f32 * 0.12).clamp(0.20, 1.2);
                    let animated_steps = ctx.animate_value_with_time(
                        egui::Id::new("derivation_trace_steps"),
                        total_steps as f32,
                        duration,
                    );
                    let visible_steps = (animated_steps.ceil() as usize).min(total_steps);
                    if result.steps.is_empty() {
                        ui.label(
                            egui::RichText::new("Nenhum passo aplicado ainda.")
                                .color(egui::Color32::from_rgb(140, 120, 180))
                                .size(12.0),
                        );
                    } else {
                        for (index, step) in result.steps.iter().take(visible_steps).enumerate() {
                            egui::Frame::NONE
                                .fill(egui::Color32::from_rgb(20, 14, 38))
                                .stroke(egui::Stroke::new(
                                    1.0_f32,
                                    egui::Color32::from_rgb(45, 28, 75),
                                ))
                                .corner_radius(egui::CornerRadius::same(8))
                                .inner_margin(egui::Margin::symmetric(12, 8))
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(
                                            egui::RichText::new(format!("Passo {:02}", index + 1))
                                                .color(egui::Color32::from_rgb(0, 240, 255))
                                                .size(11.0)
                                                .strong(),
                                        );
                                        let production: String = step
                                            .production
                                            .iter()
                                            .map(ToString::to_string)
                                            .collect();
                                        let full_rule = format!("{} → {production}", step.non_terminal);
                                        let fractional_step = animated_steps.fract();
                                        let rule_text = if index + 1 == visible_steps
                                            && animated_steps < total_steps as f32
                                        {
                                            typewriter_prefix(&full_rule, fractional_step)
                                        } else {
                                            full_rule
                                        };
                                        ui.with_layout(
                                            egui::Layout::right_to_left(egui::Align::Center),
                                            |ui| {
                                                ui.monospace(
                                                    egui::RichText::new(rule_text)
                                                    .color(egui::Color32::from_rgb(
                                                        238, 166, 255,
                                                    ))
                                                    .strong(),
                                                );
                                            },
                                        );
                                    });

                                    ui.add_space(4.0);
                                    ui.label(
                                        egui::RichText::new(format!(
                                            "Saída parcial: \"{}\"",
                                            if step.output_so_far.is_empty() {
                                                "ε"
                                            } else {
                                                &step.output_so_far
                                            }
                                        ))
                                        .color(egui::Color32::from_rgb(52, 255, 180))
                                        .size(11.0),
                                    );

                                    let stack: String = step
                                        .stack_after
                                        .iter()
                                        .map(ToString::to_string)
                                        .collect::<Vec<_>>()
                                        .join(" · ");
                                    ui.label(
                                        egui::RichText::new(format!(
                                            "Pilha pós-regra: {}",
                                            if stack.is_empty() { "∅" } else { &stack }
                                        ))
                                        .color(egui::Color32::from_rgb(160, 140, 200))
                                        .size(11.0),
                                    );
                                });
                            ui.add_space(6.0);
                        }
                    }
                } else {
                    ui.label(
                        egui::RichText::new("A trilha acadêmica será registrada aqui conforme você deriver ou jogar no labirinto.")
                            .color(egui::Color32::from_rgb(140, 120, 180))
                            .size(12.0),
                    );
                }
            });
        });
}

pub fn show_secret_status(ctx: &egui::Context, state: &AppState) {
    egui::SidePanel::right("secret_route_panel")
        .default_width(310.0)
        .resizable(false)
        .frame(
            egui::Frame::side_top_panel(&ctx.style())
                .fill(egui::Color32::from_rgb(14, 10, 25))
                .stroke(egui::Stroke::new(
                    1.0_f32,
                    egui::Color32::from_rgb(45, 28, 75),
                ))
                .inner_margin(egui::Margin::symmetric(18, 16)),
        )
        .show(ctx, |ui| {
            ui.label(
                egui::RichText::new("MODO ENIGMA")
                    .color(egui::Color32::from_rgb(0, 240, 255))
                    .strong()
                    .size(16.0),
            );
            ui.add_space(8.0);
            ui.label(
                egui::RichText::new(
                    "A sentença só será revelada ao concluir todas as portas corretas.",
                )
                .color(egui::Color32::from_rgb(170, 150, 210))
                .size(12.0),
            );
            ui.add_space(18.0);
            if let Some(grammar) = &state.current_grammar {
                let production_count: usize = grammar.productions.values().map(Vec::len).sum();
                ui.label(
                    egui::RichText::new("GRAMÁTICA GERADA")
                        .color(egui::Color32::from_rgb(140, 120, 180))
                        .strong()
                        .size(11.0),
                );
                ui.monospace(
                    egui::RichText::new(format!(
                        "N: {}   T: {}   P: {}",
                        grammar.non_terminals.len(),
                        grammar.terminals.len(),
                        production_count
                    ))
                    .color(egui::Color32::from_rgb(238, 166, 255))
                    .size(12.0),
                );
                ui.add_space(14.0);
            }
            ui.label(
                egui::RichText::new(format!(
                    "PROGRESSO  {}/{}",
                    state.secret_progress(),
                    state.secret_total_steps()
                ))
                .color(egui::Color32::from_rgb(250, 204, 21))
                .strong()
                .size(13.0),
            );
            ui.add_space(6.0);
            ui.label(
                egui::RichText::new(format!(
                    "🚩 Checkpoint: porta {} — um erro nunca volta antes disso",
                    state.secret_last_checkpoint()
                ))
                .color(egui::Color32::from_rgb(52, 255, 180))
                .size(12.0),
            );
            ui.add_space(6.0);
            ui.label(
                egui::RichText::new(format!("Reinícios: {}", state.restart_count))
                    .color(egui::Color32::from_rgb(255, 140, 165))
                    .size(12.0),
            );

            if let Some(derivation_state) = &state.derivation_state
                && !derivation_state.steps.is_empty()
            {
                ui.add_space(18.0);
                ui.separator();
                ui.add_space(10.0);
                ui.label(
                    egui::RichText::new("PORTAS PERCORRIDAS")
                        .color(egui::Color32::from_rgb(140, 120, 180))
                        .strong()
                        .size(11.0),
                );
                ui.add_space(8.0);
                egui::ScrollArea::vertical()
                    .id_salt("secret_route_history")
                    .show(ui, |ui| {
                        for (index, step) in derivation_state.steps.iter().enumerate() {
                            let production: String =
                                step.production.iter().map(ToString::to_string).collect();
                            egui::Frame::NONE
                                .fill(egui::Color32::from_rgb(20, 14, 38))
                                .stroke(egui::Stroke::new(
                                    1.0_f32,
                                    egui::Color32::from_rgb(45, 28, 75),
                                ))
                                .corner_radius(egui::CornerRadius::same(8))
                                .inner_margin(egui::Margin::symmetric(10, 6))
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(
                                            egui::RichText::new(format!("Porta {:02}", index + 1))
                                                .color(egui::Color32::from_rgb(0, 240, 255))
                                                .size(11.0)
                                                .strong(),
                                        );
                                        ui.with_layout(
                                            egui::Layout::right_to_left(egui::Align::Center),
                                            |ui| {
                                                ui.monospace(
                                                    egui::RichText::new(format!(
                                                        "{} → {production}",
                                                        step.non_terminal
                                                    ))
                                                    .color(egui::Color32::from_rgb(238, 166, 255))
                                                    .size(12.0)
                                                    .strong(),
                                                );
                                            },
                                        );
                                    });
                                });
                            ui.add_space(6.0);
                        }
                    });
            }
        });
}

#[cfg(test)]
mod tests {
    use super::typewriter_prefix;

    #[test]
    fn typewriter_reveals_a_prefix_without_splitting_utf8_characters() {
        assert_eq!(typewriter_prefix("S → ação", 0.75), "S → aç");
        assert_eq!(typewriter_prefix("S → ação", 1.0), "S → ação");
    }
}
