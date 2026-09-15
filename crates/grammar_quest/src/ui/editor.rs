use grammar_engine::EXAMPLE_SOURCES;

use crate::state::AppState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorAction {
    None,
    Generate,
    PlayMaze,
}

pub fn show_editor(ui: &mut egui::Ui, state: &mut AppState) -> EditorAction {
    ui.add_space(8.0);

    // Header badge
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new("⚡ GRAMMAR STUDIO")
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

    ui.add_space(16.0);

    // Action buttons
    let play_clicked = ui
        .add_sized(
            [ui.available_width(), 46.0],
            egui::Button::new(
                egui::RichText::new("▶ JOGAR NO LABIRINTO 2D")
                    .size(14.0)
                    .color(egui::Color32::from_rgb(10, 6, 20))
                    .strong(),
            )
            .fill(egui::Color32::from_rgb(0, 240, 255))
            .corner_radius(egui::CornerRadius::same(8)),
        )
        .clicked();

    ui.add_space(8.0);

    let generate_clicked = ui
        .add_sized(
            [ui.available_width(), 38.0],
            egui::Button::new(
                egui::RichText::new("⚡ Gerar no Laboratório")
                    .size(13.0)
                    .color(egui::Color32::from_rgb(215, 200, 245)),
            )
            .fill(egui::Color32::from_rgb(32, 20, 52))
            .corner_radius(egui::CornerRadius::same(8)),
        )
        .clicked();

    if play_clicked {
        EditorAction::PlayMaze
    } else if generate_clicked {
        EditorAction::Generate
    } else {
        EditorAction::None
    }
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
