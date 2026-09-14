use crate::state::PanelResult;

pub fn show_result(ui: &mut egui::Ui, result: Option<&PanelResult>) {
    if let Some(result) = result {
        egui::Frame::group(ui.style())
            .fill(egui::Color32::from_rgb(25, 19, 45))
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("DERIVAÇÃO CONCLUÍDA")
                        .color(egui::Color32::from_rgb(116, 255, 191)),
                );
                ui.add_space(18.0);
                ui.label("Sentença gerada");
                ui.label(
                    egui::RichText::new(&result.sentence)
                        .size(52.0)
                        .color(egui::Color32::from_rgb(74, 229, 255)),
                );
                ui.add_space(18.0);
                ui.label("Expressão regular");
                ui.monospace(
                    egui::RichText::new(&result.regex)
                        .size(18.0)
                        .color(egui::Color32::from_rgb(238, 166, 255)),
                );
                ui.add_space(12.0);
                ui.label(format!("{} expansões aplicadas", result.steps.len()));
            });
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
                        egui::Frame::group(ui.style())
                            .fill(egui::Color32::from_rgb(29, 21, 50))
                            .show(ui, |ui| {
                                ui.label(
                                    egui::RichText::new(format!("Passo {:02}", index + 1))
                                        .color(egui::Color32::from_rgb(238, 166, 255)),
                                );
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
