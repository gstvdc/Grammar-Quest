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

pub async fn run() {
    let mut state = AppState::new();
    let sfx = Sfx::load().await;

    // Embed textures so game runs reliably anywhere
    let player_tex = Texture2D::from_file_with_format(
        include_bytes!("../assets/player/swordsman_walk.png"),
        None,
    );
    player_tex.set_filter(FilterMode::Nearest);

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

        if is_key_pressed(KeyCode::Escape) {
            match state.mode {
                ScreenMode::Laboratory => break,
                ScreenMode::Playing | ScreenMode::Won => {
                    state.back_to_lab();
                }
            }
        }

        if is_key_pressed(KeyCode::Tab) && state.mode != ScreenMode::Laboratory {
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
    for dust in ambient_dust {
        draw_circle(dust.x, dust.y, 1.2, Color::from_rgba(140, 100, 210, 40));
    }

    egui_macroquad::ui(|ctx| {
        ui::theme::apply(ctx);
        ui::side_panel::show_side_panel(ctx, state.result.as_ref());

        egui::SidePanel::left("grammar_editor")
            .default_width(360.0)
            .frame(
                egui::Frame::side_top_panel(&ctx.style())
                    .fill(egui::Color32::from_rgb(14, 10, 25))
                    .stroke(egui::Stroke::new(
                        1.0_f32,
                        egui::Color32::from_rgb(45, 28, 75),
                    ))
                    .inner_margin(egui::Margin::symmetric(20, 16)),
            )
            .show(ctx, |ui| match ui::editor::show_editor(ui, state) {
                ui::editor::EditorAction::PlayFreeMaze => {
                    sfx.play_click();
                    if state.start_free_maze().is_ok()
                        && let (Some(g), Some(s)) =
                            (&state.current_grammar, &state.derivation_state)
                    {
                        let room = Room::build(g, s, VIEWPORT_W, VIEWPORT_H);
                        *player = Some(Player::new(room.spawn_pos));
                        *current_room = Some(room);
                        effects.clear();
                    }
                }
                ui::editor::EditorAction::PlayDifficultyMaze => {
                    sfx.play_click();
                    if state.start_difficulty_maze().is_ok()
                        && let (Some(g), Some(s)) =
                            (&state.current_grammar, &state.derivation_state)
                    {
                        let room = Room::build(g, s, VIEWPORT_W, VIEWPORT_H);
                        *player = Some(Player::new(room.spawn_pos));
                        *current_room = Some(room);
                        effects.clear();
                    }
                }
                ui::editor::EditorAction::Generate => {
                    sfx.play_click();
                    state.generate();
                }
                ui::editor::EditorAction::None => {}
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("GRAMMAR QUEST")
                        .color(egui::Color32::from_rgb(0, 240, 255))
                        .size(16.0)
                        .strong(),
                );
                ui.separator();
                ui.label(
                    egui::RichText::new("PAINEL DE ANÁLISE FORMAL")
                        .color(egui::Color32::from_rgb(192, 132, 252))
                        .size(12.0),
                );
            });
            ui.separator();
            ui.add_space(6.0);

            // A permanent telemetry strip instead of dead canvas:
            // register-style readout of the grammar the engine
            // actually parsed, always current, never decorative.
            ui.horizontal(|ui| {
                if let Some(overview) = &state.grammar_preview {
                    let count = |set: &str| {
                        let inner = set.trim_matches(['{', '}']);
                        if inner.is_empty() {
                            0
                        } else {
                            inner.split(',').count()
                        }
                    };
                    ui.monospace(
                        egui::RichText::new(format!(
                            "N:{}  T:{}  P:{}  S:{}",
                            count(&overview.non_terminals),
                            count(&overview.terminals),
                            overview.production_count,
                            overview.start
                        ))
                        .color(egui::Color32::from_rgb(160, 140, 200))
                        .size(11.0),
                    );
                }
                let (regular_text, regular_color) = match state.grammar_is_regular {
                    Some(true) => ("REGULAR: SIM", egui::Color32::from_rgb(52, 255, 180)),
                    Some(false) => ("REGULAR: NÃO", egui::Color32::from_rgb(255, 140, 165)),
                    None => ("REGULAR: —", egui::Color32::from_rgb(140, 120, 180)),
                };
                ui.monospace(
                    egui::RichText::new(regular_text)
                        .color(regular_color)
                        .size(11.0)
                        .strong(),
                );
            });
            ui.add_space(4.0);
            ui.separator();

            // The result card is short relative to the window, so
            // center it in the remaining space instead of letting
            // it strand near the top with a dead void below (the
            // "operate" surface should read as a filled console,
            // not an empty one). Height is measured a frame late
            // via egui's temp-memory pattern; the lag is
            // imperceptible since this content is static per
            // state change, not animating every frame.
            let height_id = egui::Id::new("lab_result_block_height");
            let remembered_height = ui.data(|d| d.get_temp::<f32>(height_id)).unwrap_or(0.0);
            let top_pad = ((ui.available_height() - remembered_height) / 2.0).max(24.0);
            ui.add_space(top_pad);

            let response = ui.vertical_centered(|ui| {
                ui::side_panel::show_result(ui, state.result.as_ref());
            });
            ui.data_mut(|d| d.insert_temp(height_id, response.response.rect.height()));
        });
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
                sfx.play_click();
                state.back_to_lab();
            }
            ui::game_hud::HudAction::PlayAgain => {
                sfx.play_click();
                state.reset_maze();
                if let (Some(g), Some(s)) = (&state.current_grammar, &state.derivation_state) {
                    let room = Room::build(g, s, VIEWPORT_W, VIEWPORT_H);
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
