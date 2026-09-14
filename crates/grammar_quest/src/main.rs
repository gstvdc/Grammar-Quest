use macroquad::prelude::*;

mod state;
mod ui;

fn window_conf() -> Conf {
    Conf {
        window_title: "Grammar Quest".to_owned(),
        window_width: 1280,
        window_height: 800,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = state::AppState::new();
    loop {
        clear_background(Color::from_rgba(13, 7, 24, 255));

        egui_macroquad::ui(|ctx| {
            ui::theme::apply(ctx);
            ui::side_panel::show_side_panel(ctx, state.result.as_ref());
            egui::SidePanel::left("grammar_editor")
                .default_width(330.0)
                .show(ctx, |ui| {
                    if ui::editor::show_editor(ui, &mut state) {
                        state.generate();
                    }
                });
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(100.0);
                    ui::side_panel::show_result(ui, state.result.as_ref());
                });
            });
        });

        egui_macroquad::draw();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}
