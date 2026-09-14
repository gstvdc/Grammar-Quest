use crate::state::PanelResult;

pub fn show_result(ui: &mut egui::Ui, result: Option<&PanelResult>) {
    if let Some(result) = result {
        ui.heading("Sentença gerada");
        ui.label(
            egui::RichText::new(&result.sentence)
                .size(36.0)
                .color(egui::Color32::from_rgb(74, 229, 255)),
        );
        ui.add_space(12.0);
        ui.label("Expressão regular");
        ui.monospace(&result.regex);
    } else {
        ui.heading("Pronto para derivar");
        ui.label("Escolha um exemplo ou escreva uma gramática regular e gere uma sentença.");
    }
}

pub fn show_side_panel(ctx: &egui::Context, result: Option<&PanelResult>) {
    egui::SidePanel::right("derivation_trace")
        .resizable(true)
        .default_width(300.0)
        .show(ctx, |ui| {
            ui.heading("Pilha e derivação");
            egui::ScrollArea::vertical().show(ui, |ui| {
                if let Some(result) = result {
                    for (index, step) in result.steps.iter().enumerate() {
                        ui.group(|ui| {
                            ui.label(format!("Passo {}", index + 1));
                            let production: String =
                                step.production.iter().map(ToString::to_string).collect();
                            ui.monospace(format!("{} → {production}", step.non_terminal));
                            ui.label(format!(
                                "Saída: {}",
                                if step.output_so_far.is_empty() {
                                    "ε"
                                } else {
                                    &step.output_so_far
                                }
                            ));
                            let stack: String = step
                                .stack_after
                                .iter()
                                .map(ToString::to_string)
                                .collect::<Vec<_>>()
                                .join(" · ");
                            ui.label(format!(
                                "Topo → base: {}",
                                if stack.is_empty() { "∅" } else { &stack }
                            ));
                        });
                        ui.add_space(6.0);
                    }
                } else {
                    ui.label("A pilha aparecerá aqui após a geração.");
                }
            });
        });
}
