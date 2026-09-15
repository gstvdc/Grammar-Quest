use macroquad::miniquad::conf::Icon;
use macroquad::prelude::*;

mod app;
mod audio;
mod effects;
mod gameplay;
mod maze;
mod player;
mod render;
mod state;
mod ui;

include!(concat!(env!("OUT_DIR"), "/window_icon.rs"));

fn window_conf() -> Conf {
    Conf {
        window_title: "Grammar Quest — Grammar Maze 2D".to_owned(),
        window_width: 1280,
        window_height: 800,
        icon: Some(Icon {
            small: WINDOW_ICON_SMALL,
            medium: WINDOW_ICON_MEDIUM,
            big: WINDOW_ICON_BIG,
        }),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    app::run().await;
}
