use macroquad::prelude::*;

use crate::state::{AppState, MainMenuStage, PlayMode};

const INK: Color = Color::new(0.075, 0.035, 0.13, 1.0);
const PLUM: Color = Color::new(0.176, 0.106, 0.306, 1.0);
const GOLD: Color = Color::new(1.0, 0.808, 0.38, 1.0);
const RED: Color = Color::new(0.58, 0.13, 0.2, 1.0);
const PARCHMENT: Color = Color::new(1.0, 0.9, 0.62, 1.0);
const SKY_TOP: Color = Color::new(0.102, 0.059, 0.18, 1.0);
const SKY_MIDDLE: Color = Color::new(0.176, 0.106, 0.306, 1.0);
const SKY_BOTTOM: Color = Color::new(0.29, 0.176, 0.431, 1.0);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MainMenuAction {
    None,
    OpenModeChoice,
    SelectMode(PlayMode),
    OpenOptions,
    Quit,
    Back,
}

pub fn show_main_menu(state: &AppState, font: &Font, sparks: &[Vec2], time: f32) -> MainMenuAction {
    miniquad::window::set_mouse_cursor(miniquad::CursorIcon::Default);
    draw_retro_background(sparks, time);
    draw_title(font, time);

    match state.menu_stage {
        MainMenuStage::Root => show_root_actions(font),
        MainMenuStage::ChooseMode => show_mode_actions(font),
    }
}

pub fn draw_retro_background(sparks: &[Vec2], time: f32) {
    let width = screen_width();
    let height = screen_height();
    let scale_x = width / 1280.0;
    let scale_y = height / 800.0;
    for y in 0..height.ceil() as i32 {
        let progress = y as f32 / (height - 1.0).max(1.0);
        let color = if progress < 0.4 {
            blend(SKY_TOP, SKY_MIDDLE, progress / 0.4)
        } else {
            blend(SKY_MIDDLE, SKY_BOTTOM, (progress - 0.4) / 0.6)
        };
        draw_line(0.0, y as f32, width, y as f32, 1.0, color);
    }
    for y in (0..height.ceil() as i32).step_by(4) {
        draw_rectangle(0.0, y as f32, width, 2.0, Color::new(0.0, 0.0, 0.0, 0.15));
    }
    for (index, spark) in sparks.iter().enumerate() {
        let shimmer = ((time * 2.0) + index as f32 * 0.8).sin().mul_add(0.25, 0.7);
        draw_circle(
            spark.x * scale_x,
            spark.y * scale_y,
            1.5 + shimmer,
            Color::new(1.0, 0.76, 0.28, shimmer),
        );
    }
    draw_rectangle_lines(
        12.0,
        12.0,
        width - 24.0,
        height - 24.0,
        3.0,
        Color::new(0.05, 0.02, 0.1, 0.55),
    );
}

fn draw_title(font: &Font, time: f32) {
    let center_x = screen_width() / 2.0;
    let pulse = (time * 2.4).sin();
    let title_size = (44.0 * title_scale_at(time)).round() as u16;
    let title_lift = pulse * 2.0;
    centered_text(
        "GRAMMAR",
        center_x + 6.0,
        241.0 - title_lift,
        title_size,
        RED,
        font,
    );
    centered_text(
        "GRAMMAR",
        center_x,
        235.0 - title_lift,
        title_size,
        GOLD,
        font,
    );
    centered_text(
        "QUEST",
        center_x + 6.0,
        299.0 - title_lift,
        title_size,
        RED,
        font,
    );
    centered_text(
        "QUEST",
        center_x,
        293.0 - title_lift,
        title_size,
        GOLD,
        font,
    );
    centered_text(
        "A aventura das gramáticas regulares",
        center_x,
        329.0,
        13,
        PARCHMENT,
        font,
    );
}

fn title_scale_at(time: f32) -> f32 {
    1.0 + (time * 2.4).sin() * 0.035
}

fn show_root_actions(font: &Font) -> MainMenuAction {
    let actions = [
        (
            "▶ JOGAR",
            "Começar uma jornada",
            MainMenuAction::OpenModeChoice,
        ),
        ("OPÇÕES", "Volume e créditos", MainMenuAction::OpenOptions),
        ("SAIR", "Encerrar o jogo", MainMenuAction::Quit),
    ];
    let button_x = (screen_width() - 310.0) / 2.0;
    for (index, (label, hint, action)) in actions.iter().enumerate() {
        let y = 375.0 + index as f32 * 94.0;
        if pixel_button(button_x, y, 310.0, 58.0, label, hint, font) {
            return *action;
        }
    }
    MainMenuAction::None
}

fn show_mode_actions(font: &Font) -> MainMenuAction {
    let center_x = screen_width() / 2.0;
    centered_text("ESCOLHA SUA JORNADA", center_x, 369.0, 15, PARCHMENT, font);
    let choices = [
        (
            "MODO LIVRE",
            "Edite uma gramática e explore",
            MainMenuAction::SelectMode(PlayMode::Free),
        ),
        (
            "MODO ENIGMA",
            "Descubra a rota correta",
            MainMenuAction::SelectMode(PlayMode::Enigma),
        ),
        ("← VOLTAR", "Retornar ao início", MainMenuAction::Back),
    ];
    let button_x = (screen_width() - 310.0) / 2.0;
    for (index, (label, hint, action)) in choices.iter().enumerate() {
        let y = 405.0 + index as f32 * 94.0;
        if pixel_button(button_x, y, 310.0, 58.0, label, hint, font) {
            return *action;
        }
    }
    if is_key_pressed(KeyCode::Escape) {
        MainMenuAction::Back
    } else {
        MainMenuAction::None
    }
}

fn pixel_button(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    label: &str,
    hint: &str,
    font: &Font,
) -> bool {
    let (mouse_x, mouse_y) = mouse_position();
    let hovered = (x..=x + width).contains(&mouse_x) && (y..=y + height).contains(&mouse_y);
    if hovered {
        miniquad::window::set_mouse_cursor(miniquad::CursorIcon::Pointer);
    }
    let fill = if hovered { GOLD } else { PLUM };
    let text_color = if hovered { INK } else { PARCHMENT };
    draw_rectangle(x + 5.0, y + 5.0, width, height, RED);
    draw_rectangle(x, y, width, height, fill);
    draw_rectangle_lines(x, y, width, height, 3.0, GOLD);
    centered_text(label, x + width / 2.0, y + 29.0, 16, text_color, font);
    centered_text(hint, x + width / 2.0, y + 78.0, 10, PARCHMENT, font);
    hovered && is_mouse_button_pressed(MouseButton::Left)
}

fn centered_text(text: &str, center_x: f32, baseline_y: f32, size: u16, color: Color, font: &Font) {
    let dimensions = measure_text(text, Some(font), size, 1.0);
    draw_text_ex(
        text,
        center_x - dimensions.width / 2.0,
        baseline_y,
        TextParams {
            font: Some(font),
            font_size: size,
            color,
            ..Default::default()
        },
    );
}

fn blend(start: Color, end: Color, progress: f32) -> Color {
    let t = progress.clamp(0.0, 1.0);
    Color::new(
        start.r + (end.r - start.r) * t,
        start.g + (end.g - start.g) * t,
        start.b + (end.b - start.b) * t,
        1.0,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn title_pulse_stays_subtle_and_reaches_its_peak() {
        let resting = title_scale_at(0.0);
        let peak = title_scale_at(std::f32::consts::FRAC_PI_2 / 2.4);

        assert_eq!(resting, 1.0);
        assert!(peak > resting);
        assert!(peak <= 1.04);
    }
}
