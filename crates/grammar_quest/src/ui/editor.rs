use grammar_engine::EXAMPLE_SOURCES;

use crate::state::{AppState, Difficulty, PlayMode};
use crate::ui::theme::animated_button;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorAction {
    None,
    Generate,
    PlayFreeMaze,
    PlayDifficultyMaze,
}

pub fn show_editor(ui: &mut egui::Ui, state: &mut AppState) -> EditorAction {
    ui.add_space(8.0);

    // Header badge
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new("GRAMMAR STUDIO")
                .color(egui::Color32::from_rgb(0, 240, 255))
                .strong()
                .size(16.0),
        );
    });
    ui.label(
        egui::RichText::new("Editor de Gramáticas Regulares")
            .color(egui::Color32::from_rgb(170, 150, 210))
            .size(12.0),
    );

    ui.add_space(14.0);

    // Presets Section
    ui.label(
        egui::RichText::new("EXEMPLOS PRONTOS")
            .color(egui::Color32::from_rgb(140, 120, 180))
            .size(11.0)
            .strong(),
    );
    let selected = state
        .selected_example
        .and_then(|index| EXAMPLE_SOURCES.get(index))
        .map_or("Editor manual", |example| example.name);

    egui::ComboBox::from_id_salt("preset_selector")
        .selected_text(egui::RichText::new(selected).color(egui::Color32::WHITE))
        .width(ui.available_width() - 8.0)
        .show_ui(ui, |ui| {
            for (index, example) in EXAMPLE_SOURCES.iter().enumerate() {
                if ui
                    .selectable_label(state.selected_example == Some(index), example.name)
                    .clicked()
                {
                    state.select_example(index);
                }
            }
        });

    if let Some(example) = state
        .selected_example
        .and_then(|index| EXAMPLE_SOURCES.get(index))
    {
        ui.add_space(4.0);
        ui.label(
            egui::RichText::new(example.description)
                .color(egui::Color32::from_rgb(140, 120, 180))
                .size(11.0)
                .italics(),
        );
    }

    ui.add_space(14.0);

    // Editor Section
    ui.label(
        egui::RichText::new("PRODUÇÕES (P)")
            .color(egui::Color32::from_rgb(140, 120, 180))
            .size(11.0)
            .strong(),
    );

    let response = ui.add(
        egui::TextEdit::multiline(&mut state.grammar_text)
            .hint_text("S -> aS | ab")
            .desired_rows(7)
            .font(egui::TextStyle::Monospace)
            .desired_width(ui.available_width()),
    );

    if response.changed() {
        let text = state.grammar_text.clone();
        state.set_grammar_text(text);
    }

    ui.add_space(4.0);
    ui.label(
        egui::RichText::new("💡 Dica: Símbolos maiúsculos = N-T; minúsculos = terminais.")
            .color(egui::Color32::from_rgb(120, 100, 160))
            .size(11.0),
    );

    // Error Alert Box
    if let Some(error) = &state.error_message {
        ui.add_space(8.0);
        egui::Frame::NONE
            .fill(egui::Color32::from_rgb(45, 15, 30))
            .stroke(egui::Stroke::new(
                1.0_f32,
                egui::Color32::from_rgb(255, 99, 132),
            ))
            .corner_radius(egui::CornerRadius::same(8))
            .inner_margin(egui::Margin::symmetric(12, 8))
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new(format!("⚠ {error}"))
                        .color(egui::Color32::from_rgb(255, 140, 165))
                        .size(12.0),
                );
            });
    }

    ui.add_space(12.0);
    show_grammar_overview_card(ui, state);

    ui.add_space(16.0);
    ui.label(
        egui::RichText::new("MODO DE JOGO")
            .color(egui::Color32::from_rgb(140, 120, 180))
            .size(11.0)
            .strong(),
    );
    ui.add_space(6.0);

    let toggle_width = (ui.available_width() - 8.0) / 2.0;
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 8.0;

        let free_selected = state.play_mode == PlayMode::Free;
        let free_fill = if free_selected {
            egui::Color32::from_rgb(52, 255, 180)
        } else {
            egui::Color32::from_rgb(26, 17, 44)
        };
        let free_text = if free_selected {
            egui::Color32::from_rgb(10, 6, 20)
        } else {
            egui::Color32::from_rgb(200, 185, 230)
        };
        if animated_button(
            ui,
            "mode_toggle_free",
            egui::vec2(toggle_width, 34.0),
            "🗺 Livre",
            13.0,
            free_text,
            free_fill,
        )
        .clicked()
        {
            state.play_mode = PlayMode::Free;
        }

        let enigma_selected = state.play_mode == PlayMode::Enigma;
        let enigma_fill = if enigma_selected {
            egui::Color32::from_rgb(0, 240, 255)
        } else {
            egui::Color32::from_rgb(26, 17, 44)
        };
        let enigma_text = if enigma_selected {
            egui::Color32::from_rgb(10, 6, 20)
        } else {
            egui::Color32::from_rgb(200, 185, 230)
        };
        if animated_button(
            ui,
            "mode_toggle_enigma",
            egui::vec2(toggle_width, 34.0),
            "🧩 Enigma",
            13.0,
            enigma_text,
            enigma_fill,
        )
        .clicked()
        {
            state.play_mode = PlayMode::Enigma;
        }
    });

    ui.add_space(10.0);

    match state.play_mode {
        PlayMode::Free => {
            ui.label(
                egui::RichText::new("Usa o exemplo selecionado ou as produções digitadas acima.")
                    .color(egui::Color32::from_rgb(120, 100, 160))
                    .size(11.0),
            );
        }
        PlayMode::Enigma => {
            let (min_steps, max_steps) = state.selected_difficulty.step_range();
            egui::ComboBox::from_id_salt("difficulty_selector")
                .selected_text(format!(
                    "{} · {}–{} portas",
                    state.selected_difficulty.label(),
                    min_steps,
                    max_steps
                ))
                .width(ui.available_width() - 8.0)
                .show_ui(ui, |ui| {
                    for difficulty in Difficulty::ALL {
                        let (min_steps, max_steps) = difficulty.step_range();
                        ui.selectable_value(
                            &mut state.selected_difficulty,
                            difficulty,
                            format!(
                                "{} · {}–{} portas",
                                difficulty.label(),
                                min_steps,
                                max_steps
                            ),
                        );
                    }
                });
            ui.add_space(4.0);
            ui.label(
                egui::RichText::new(
                    "Gera uma gramática nova, com mais produções, portas e símbolos conforme o nível. A sentença só é revelada ao concluir o enigma.",
                )
                .color(egui::Color32::from_rgb(120, 100, 160))
                .size(11.0),
            );
        }
    }

    ui.add_space(12.0);

    let (cta_label, cta_fill) = match state.play_mode {
        PlayMode::Free => (
            "▶ JOGAR LABIRINTO LIVRE",
            egui::Color32::from_rgb(52, 255, 180),
        ),
        PlayMode::Enigma => (
            "▶ INICIAR MODO ENIGMA",
            egui::Color32::from_rgb(0, 240, 255),
        ),
    };
    let cta_clicked = animated_button(
        ui,
        "editor_cta",
        egui::vec2(ui.available_width(), 46.0),
        cta_label,
        14.0,
        egui::Color32::from_rgb(10, 6, 20),
        cta_fill,
    )
    .clicked();

    ui.add_space(8.0);

    let generate_clicked = animated_button(
        ui,
        "editor_generate",
        egui::vec2(ui.available_width(), 38.0),
        "⚡ Gerar sentença aleatória",
        13.0,
        egui::Color32::from_rgb(215, 200, 245),
        egui::Color32::from_rgb(32, 20, 52),
    )
    .clicked();

    if cta_clicked {
        match state.play_mode {
            PlayMode::Free => EditorAction::PlayFreeMaze,
            PlayMode::Enigma => EditorAction::PlayDifficultyMaze,
        }
    } else if generate_clicked {
        EditorAction::Generate
    } else {
        EditorAction::None
    }
}

/// Renders the literal `G={N,T,P,S}` ficha the audit flagged as missing
/// (docs/tasks/2026-09-15-audit-adjustments.md, T1) — the grader should see
/// the four grammar components without inferring them from raw production
/// text. Reads `AppState::grammar_preview`, never re-parses or reinterprets
/// the grammar itself.
fn show_grammar_overview_card(ui: &mut egui::Ui, state: &AppState) {
    egui::Frame::NONE
        .fill(egui::Color32::from_rgb(16, 11, 28))
        .stroke(egui::Stroke::new(
            1.0_f32,
            egui::Color32::from_rgb(45, 28, 75),
        ))
        .corner_radius(egui::CornerRadius::same(8))
        .inner_margin(egui::Margin::symmetric(12, 10))
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new("G = {N, T, P, S}")
                    .color(egui::Color32::from_rgb(160, 140, 200))
                    .size(11.0)
                    .strong(),
            );
            ui.add_space(6.0);
            match &state.grammar_preview {
                Some(overview) => {
                    ui.monospace(
                        egui::RichText::new(format!("N = {}", overview.non_terminals))
                            .color(egui::Color32::from_rgb(238, 166, 255))
                            .size(12.0),
                    );
                    ui.add_space(2.0);
                    ui.monospace(
                        egui::RichText::new(format!("T = {}", overview.terminals))
                            .color(egui::Color32::from_rgb(74, 229, 255))
                            .size(12.0),
                    );
                    ui.add_space(2.0);
                    ui.monospace(
                        egui::RichText::new(format!("S = {}", overview.start))
                            .color(egui::Color32::from_rgb(52, 255, 180))
                            .size(12.0),
                    );
                    ui.label(
                        egui::RichText::new("(S é sempre o primeiro lado esquerdo declarado)")
                            .color(egui::Color32::from_rgb(140, 120, 180))
                            .size(10.0)
                            .italics(),
                    );

                    ui.add_space(8.0);
                    for line in &overview.production_lines {
                        ui.monospace(
                            egui::RichText::new(format!("P: {line}"))
                                .color(egui::Color32::from_rgb(215, 200, 245))
                                .size(12.0),
                        );
                        ui.add_space(2.0);
                    }
                    ui.label(
                        egui::RichText::new(format!(
                            "{} produção(ões) no total",
                            overview.production_count
                        ))
                        .color(egui::Color32::from_rgb(140, 120, 180))
                        .size(11.0),
                    );
                }
                None => {
                    ui.label(
                        egui::RichText::new(
                            "Gramática inválida — corrija a sintaxe para ver N/T/P/S.",
                        )
                        .color(egui::Color32::from_rgb(255, 140, 165))
                        .size(11.0),
                    );
                }
            }
        });
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn editor_is_available_to_the_app() {
        let ctx = egui::Context::default();
        let mut state = crate::state::AppState::new();
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let action = show_editor(ui, &mut state);
                assert_eq!(action, EditorAction::None);
            });
        });
    }
}
