use macroquad::prelude::*;

use crate::audio::Sfx;
use crate::effects::EffectsState;
use crate::gameplay;
use crate::maze::Room;
use crate::player::Player;
use crate::render;
use crate::state::{AppState, GameFlow, ScreenMode};
use crate::ui;

const VIEWPORT_W: f32 = 1280.0;
const VIEWPORT_H: f32 = 800.0;

fn build_game_room(
    state: &AppState,
    grammar: &grammar_engine::Grammar,
    derivation: &grammar_engine::DerivationState,
) -> Room {
    let (left_panel_width, right_panel_width) = match (state.show_side_panel, state.active_flow) {
        (true, Some(GameFlow::SecretChallenge)) => (180.0, 310.0),
        (true, _) => (340.0, 0.0),
        (false, _) => (0.0, 0.0),
    };

    Room::build_with_panels(
        grammar,
        derivation,
        screen_width(),
        screen_height(),
        left_panel_width,
        right_panel_width,
    )
}

pub async fn run() {
    let mut state = AppState::new();
    let sfx = Sfx::load().await;

    // Embed textures so game runs reliably anywhere
    let player_tex = Texture2D::from_file_with_format(
        include_bytes!("../assets/player/swordsman_walk.png"),
        None,
    );
    player_tex.set_filter(FilterMode::Nearest);
    let menu_font =
        load_ttf_font_from_bytes(include_bytes!("../assets/fonts/PressStart2P-Regular.ttf"))
            .expect("embedded Press Start 2P font is valid");

    let mut current_room: Option<Room> = None;
    let mut player: Option<Player> = None;
    let mut effects = EffectsState::default();

    // Ambient background cyber-dust
    let mut ambient_dust: Vec<Vec2> = (0..40)
        .map(|_| {
            vec2(
                rand::gen_range(0.0, VIEWPORT_W),
                rand::gen_range(0.0, VIEWPORT_H),
            )
        })
        .collect();

    let mut global_timer: f32 = 0.0;

    loop {
        let dt = get_frame_time().min(0.05);
        global_timer += dt;
        state.advance_animations(dt);

        clear_background(Color::from_rgba(10, 7, 18, 255));

        let mut should_quit = false;
        if is_key_pressed(KeyCode::Escape) {
            state.handle_escape();
        }

        if is_key_pressed(KeyCode::Tab)
            && matches!(state.mode, ScreenMode::Playing | ScreenMode::Won)
        {
            state.show_side_panel = !state.show_side_panel;
        }

        for dust in &mut ambient_dust {
            dust.y -= 12.0 * dt;
            if dust.y < 0.0 {
                dust.y = VIEWPORT_H;
                dust.x = rand::gen_range(0.0, VIEWPORT_W);
            }
        }

        match state.mode {
            ScreenMode::MainMenu => {
                match ui::main_menu::show_main_menu(&state, &menu_font, &ambient_dust, global_timer)
                {
                    ui::main_menu::MainMenuAction::OpenModeChoice => {
                        sfx.play_click(state.master_volume);
                        state.open_mode_selection();
                    }
                    ui::main_menu::MainMenuAction::SelectMode(play_mode) => {
                        sfx.play_click(state.master_volume);
                        state.select_play_mode(play_mode);
                    }
                    ui::main_menu::MainMenuAction::OpenOptions => {
                        sfx.play_click(state.master_volume);
                        state.open_options();
                    }
                    ui::main_menu::MainMenuAction::Quit => {
                        sfx.play_click(state.master_volume);
                        should_quit = true;
                    }
                    ui::main_menu::MainMenuAction::Back => state.back_to_main_menu(),
                    ui::main_menu::MainMenuAction::None => {}
                }
            }
            ScreenMode::Options => {
                ui::main_menu::draw_retro_background(&ambient_dust, global_timer);
                let fullscreen_before = state.fullscreen;
                egui_macroquad::ui(|ctx| {
                    ui::theme::apply(ctx);
                    if ui::options::show_options(ctx, &mut state)
                        == ui::options::OptionsAction::Back
                    {
                        state.back_to_main_menu();
                    }
                });
                if state.fullscreen != fullscreen_before {
                    set_fullscreen(state.fullscreen);
                }
            }
            ScreenMode::Laboratory => {
                run_laboratory_frame(
                    &mut state,
                    &sfx,
                    &ambient_dust,
                    &mut current_room,
                    &mut player,
                    &mut effects,
                );
            }

            ScreenMode::Playing | ScreenMode::Won => {
                if state.mode == ScreenMode::Playing {
                    gameplay::update_playing(
                        &mut state,
                        &sfx,
                        &mut player,
                        &mut current_room,
                        &mut effects,
                        dt,
                    );
                }

                effects.update(dt);

                if let Some(ref room) = current_room {
                    render::draw_playing_scene(
                        &ambient_dust,
                        room,
                        player.as_ref(),
                        &player_tex,
                        &effects,
                        global_timer,
                    );
                }

                run_playing_hud(
                    &mut state,
                    &sfx,
                    &mut player,
                    &mut current_room,
                    &mut effects,
                );
            }
        }

        egui_macroquad::draw();
        if should_quit {
            break;
        }
        next_frame().await;
    }
}

fn run_laboratory_frame(
    state: &mut AppState,
    sfx: &Sfx,
    ambient_dust: &[Vec2],
    current_room: &mut Option<Room>,
    player: &mut Option<Player>,
    effects: &mut EffectsState,
) {
    ui::main_menu::draw_retro_background(ambient_dust, get_time() as f32);
    egui_macroquad::ui(|ctx| {
        ui::theme::apply(ctx);
        match ui::laboratory::show_laboratory(ctx, state) {
            ui::laboratory::LaboratoryAction::PlayFree => {
                sfx.play_click(state.master_volume);
                if state.start_free_maze().is_ok()
                    && let (Some(g), Some(s)) = (&state.current_grammar, &state.derivation_state)
                {
                    let room = build_game_room(state, g, s);
                    *player = Some(Player::new(room.spawn_pos));
                    *current_room = Some(room);
                    effects.clear();
                }
            }
            ui::laboratory::LaboratoryAction::PlayEnigma => {
                sfx.play_click(state.master_volume);
                if state.start_difficulty_maze().is_ok()
                    && let (Some(g), Some(s)) = (&state.current_grammar, &state.derivation_state)
                {
                    let room = build_game_room(state, g, s);
                    *player = Some(Player::new(room.spawn_pos));
                    *current_room = Some(room);
                    effects.clear();
                }
            }
            ui::laboratory::LaboratoryAction::Generate => {
                sfx.play_click(state.master_volume);
                state.generate();
                if state.result.is_some() {
                    state.open_laboratory_stage(crate::state::LaboratoryStage::FormalResult);
                }
            }
            ui::laboratory::LaboratoryAction::None => {}
        }
    });
}

fn run_playing_hud(
    state: &mut AppState,
    sfx: &Sfx,
    player: &mut Option<Player>,
    current_room: &mut Option<Room>,
    effects: &mut EffectsState,
) {
    egui_macroquad::ui(|ctx| {
        ui::theme::apply(ctx);

        if state.show_side_panel {
            if state.active_flow == Some(GameFlow::SecretChallenge) && state.mode != ScreenMode::Won
            {
                ui::side_panel::show_secret_status(ctx, state);
            } else {
                let live_result = if state.mode == ScreenMode::Won {
                    state.result.clone()
                } else {
                    state.live_panel_result()
                };
                ui::side_panel::show_side_panel(ctx, live_result.as_ref());
            }
        }

        match ui::game_hud::show_hud(ctx, state) {
            ui::game_hud::HudAction::BackToLab => {
                sfx.play_click(state.master_volume);
                state.back_to_lab();
            }
            ui::game_hud::HudAction::PlayAgain => {
                sfx.play_click(state.master_volume);
                state.reset_maze();
                if let (Some(g), Some(s)) = (&state.current_grammar, &state.derivation_state) {
                    let room = build_game_room(state, g, s);
                    if let Some(plyr) = player {
                        plyr.pos = room.spawn_pos;
                    }
                    *current_room = Some(room);
                    effects.clear();
                }
            }
            ui::game_hud::HudAction::None => {}
        }
    });
}
