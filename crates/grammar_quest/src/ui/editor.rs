use grammar_engine::EXAMPLE_SOURCES;

use crate::state::AppState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorAction {
    None,
    Generate,
    PlayMaze,
}

pub fn show_editor(ui: &mut egui::Ui, state: &mut AppState) -> EditorAction {
    ui.heading("Grammar Maze");
    ui.label("Laboratório de gramáticas regulares");
    let selected = state
        .selected_example
        .and_then(|index| EXAMPLE_SOURCES.get(index))
        .map_or("Editor manual", |example| example.name);
    egui::ComboBox::from_label("Gramática de exemplo")
        .selected_text(selected)
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
    ui.add_space(12.0);
    let response = ui.add(
        egui::TextEdit::multiline(&mut state.grammar_text)
            .hint_text("S -> aS | ab")
            .desired_rows(8)
            .code_editor(),
    );
    if response.changed() {
        let text = state.grammar_text.clone();
        state.set_grammar_text(text);
    }
    if let Some(error) = &state.error_message {
        ui.colored_label(egui::Color32::from_rgb(255, 125, 170), format!("⚠ {error}"));
    }
    ui.add_space(10.0);

    let play_clicked = ui
        .add_sized(
            [ui.available_width(), 44.0],
            egui::Button::new(
                egui::RichText::new("⚔ ENTRAR NO LABIRINTO")
                    .size(16.0)
                    .color(egui::Color32::from_rgb(13, 7, 24))
                    .strong(),
            )
            .fill(egui::Color32::from_rgb(74, 229, 255)),
        )
        .clicked();

    ui.add_space(6.0);
    let generate_clicked = ui
        .add_sized(
            [ui.available_width(), 36.0],
            egui::Button::new("Gerar sentença rápida"),
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
