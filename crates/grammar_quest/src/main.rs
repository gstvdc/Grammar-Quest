use macroquad::prelude::*;

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
    loop {
        clear_background(Color::from_rgba(10, 12, 16, 255));

        egui_macroquad::ui(|ctx| {
            egui::Window::new("Grammar Quest").show(ctx, |ui| {
                ui.label("Scaffold ok — motor e labirinto ainda por implementar.");
            });
        });

        egui_macroquad::draw();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}
