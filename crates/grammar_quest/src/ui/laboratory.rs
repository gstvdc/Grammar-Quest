use grammar_engine::EXAMPLE_SOURCES;

use crate::state::{AppState, Difficulty, LaboratoryStage, PlayMode};
use crate::ui::side_panel;

const PLUM: egui::Color32 = egui::Color32::from_rgb(45, 27, 78);
const GOLD: egui::Color32 = egui::Color32::from_rgb(255, 206, 97);
const RED: egui::Color32 = egui::Color32::from_rgb(148, 33, 51);
const PARCHMENT: egui::Color32 = egui::Color32::from_rgb(255, 230, 158);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LaboratoryAction {
    None,
    Generate,
    PlayFree,
    PlayEnigma,
}

pub fn show_laboratory(ctx: &egui::Context, state: &mut AppState) -> LaboratoryAction {
    let mut action = LaboratoryAction::None;
    egui::CentralPanel::default()
        .frame(
            egui::Frame::central_panel(&ctx.style())
                .fill(egui::Color32::TRANSPARENT)
                .inner_margin(egui::Margin::same(80)),
        )
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                egui::Frame::NONE
                    .fill(PLUM)
                    .stroke(egui::Stroke::new(3.0_f32, GOLD))
                    .shadow(egui::epaint::Shadow {
                        offset: [6, 6],
                        blur: 0,
                        spread: 0,
                        color: RED,
                    })
                    .corner_radius(egui::CornerRadius::same(4))
                    .inner_margin(egui::Margin::symmetric(54, 38))
                    .show(ui, |ui| match state.play_mode {
                        PlayMode::Free => show_free(ui, state, &mut action),
                        PlayMode::Enigma => show_enigma(ui, state, &mut action),
                    });
            });
        });
    action
}

fn title(ui: &mut egui::Ui, text: &str, subtitle: &str) {
    ui.label(egui::RichText::new(text).size(26.0).strong().color(GOLD));
    ui.add_space(8.0);
    ui.label(egui::RichText::new(subtitle).size(13.0).color(PARCHMENT));
    ui.add_space(28.0);
}

fn show_free(ui: &mut egui::Ui, state: &mut AppState, action: &mut LaboratoryAction) {
    match state.laboratory_stage {
        LaboratoryStage::Setup => {
            title(ui, "PREPARAÇÃO LIVRE", "Escolha como montar sua gramática");
            let selected = state
                .selected_example
                .and_then(|i| EXAMPLE_SOURCES.get(i))
                .map_or("Gramática personalizada", |e| e.name);
            ui.vertical_centered(|ui| {
                ui.label(egui::RichText::new(format!("ATIVA: {selected}")).color(PARCHMENT));
            });
            ui.add_space(20.0);
            if action_button(
                ui,
                "ESCOLHER EXEMPLO",
                "Use uma das três gramáticas prontas",
            )
            .clicked()
            {
                state.open_laboratory_stage(LaboratoryStage::ExampleSelection);
            }
            if action_button(ui, "CRIAR GRAMÁTICA", "Escreva as suas próprias produções").clicked()
            {
                state.open_laboratory_stage(LaboratoryStage::GrammarEditor);
            }
            if action_button(
                ui,
                "GERAR SENTENÇA",
                "Veja uma derivação aleatória completa",
            )
            .clicked()
            {
                *action = LaboratoryAction::Generate;
            }
            let can_play = state.grammar_is_regular == Some(true);
            if ui
                .add_enabled_ui(can_play, |ui| {
                    action_button(
                        ui,
                        "▶ JOGAR LABIRINTO",
                        "Aplique as regras caminhando pelas portas",
                    )
                })
                .inner
                .clicked()
            {
                *action = LaboratoryAction::PlayFree;
            }
            if !can_play {
                ui.vertical_centered(|ui| {
                    ui.label(
                        egui::RichText::new(
                            "Disponível após informar uma gramática regular válida.",
                        )
                        .size(11.0)
                        .color(egui::Color32::from_rgb(255, 180, 190)),
                    );
                });
            }
        }
        LaboratoryStage::ExampleSelection => {
            title(ui, "ESCOLHER EXEMPLO", "Três gramáticas regulares prontas");
            for (index, example) in EXAMPLE_SOURCES.iter().enumerate() {
                if button(ui, example.name).clicked() {
                    state.select_example(index);
                    state.return_to_setup();
                }
                ui.label(
                    egui::RichText::new(example.description)
                        .size(11.0)
                        .color(PARCHMENT),
                );
                ui.add_space(8.0);
            }
            if button(ui, "← VOLTAR").clicked() {
                state.return_to_setup();
            }
        }
        LaboratoryStage::GrammarEditor => {
            title(ui, "CRIAR GRAMÁTICA", "Digite as produções regulares");
            let response = ui.add(
                egui::TextEdit::multiline(&mut state.grammar_text)
                    .desired_rows(12)
                    .desired_width(620.0)
                    .font(egui::TextStyle::Monospace),
            );
            if response.changed() {
                state.set_grammar_text(state.grammar_text.clone());
            }
            ui.add_space(12.0);
            show_overview(ui, state);
            if let Some(error) = &state.error_message {
                ui.label(egui::RichText::new(error).color(egui::Color32::from_rgb(255, 170, 170)));
            }
            ui.add_space(16.0);
            if button(ui, "SALVAR E VOLTAR").clicked() {
                state.return_to_setup();
            }
        }
        LaboratoryStage::FormalResult => {
            title(
                ui,
                "SENTENÇA GERADA",
                "Derivação por pilha e expressão regular",
            );
            show_overview(ui, state);
            side_panel::show_result(ui, state.result.as_ref());
            ui.add_space(16.0);
            if button(ui, "↻ NOVA SENTENÇA ALEATÓRIA").clicked() {
                *action = LaboratoryAction::Generate;
            }
            if button(ui, "▶ JOGAR LABIRINTO").clicked() {
                *action = LaboratoryAction::PlayFree;
            }
            if button(ui, "← VOLTAR").clicked() {
                state.return_to_setup();
            }
        }
    }
}

fn show_enigma(ui: &mut egui::Ui, state: &mut AppState, action: &mut LaboratoryAction) {
    title(ui, "PREPARAÇÃO DO ENIGMA", "Escolha a dificuldade da rota");
    for difficulty in Difficulty::ALL {
        let (min, max) = difficulty.step_range();
        let label = format!("{}  ·  {}–{} PORTAS", difficulty.label(), min, max);
        if difficulty_button(ui, &label, state.selected_difficulty == difficulty).clicked() {
            state.selected_difficulty = difficulty;
        }
    }
    ui.add_space(18.0);
    if button(ui, "▶ INICIAR ENIGMA").clicked() {
        *action = LaboratoryAction::PlayEnigma;
    }
}

fn difficulty_button(ui: &mut egui::Ui, label: &str, selected: bool) -> egui::Response {
    let fill = if selected { GOLD } else { PLUM };
    let text = if selected {
        egui::Color32::from_rgb(26, 15, 46)
    } else {
        PARCHMENT
    };
    ui.add_sized(
        egui::vec2(420.0, 42.0),
        egui::Button::new(egui::RichText::new(label).size(13.0).strong().color(text))
            .fill(fill)
            .stroke(egui::Stroke::new(
                2.0_f32,
                if selected { RED } else { GOLD },
            ))
            .corner_radius(egui::CornerRadius::same(2)),
    )
    .on_hover_cursor(egui::CursorIcon::PointingHand)
}

fn button(ui: &mut egui::Ui, label: &str) -> egui::Response {
    let button = egui::Button::new(
        egui::RichText::new(label)
            .size(13.0)
            .strong()
            .color(PARCHMENT),
    )
    .fill(egui::Color32::from_rgb(25, 16, 40))
    .stroke(egui::Stroke::new(2.0_f32, GOLD))
    .corner_radius(egui::CornerRadius::same(2));
    ui.add_sized(egui::vec2(420.0, 42.0), button)
        .on_hover_cursor(egui::CursorIcon::PointingHand)
}

fn action_button(ui: &mut egui::Ui, label: &str, hint: &str) -> egui::Response {
    let response = ui.vertical_centered(|ui| {
        let response = button(ui, label);
        ui.add_space(5.0);
        ui.label(egui::RichText::new(hint).size(11.0).color(PARCHMENT));
        ui.add_space(12.0);
        response
    });
    response.inner
}

fn show_overview(ui: &mut egui::Ui, state: &AppState) {
    if let Some(overview) = &state.grammar_preview {
        ui.monospace(format!(
            "G = {{N, T, P, S}}\nN = {}\nT = {}\nS = {}\nP = {}",
            overview.non_terminals, overview.terminals, overview.start, overview.production_count
        ));
    }
}
