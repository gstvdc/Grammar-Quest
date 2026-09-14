use macroquad::prelude::*;

mod maze;
mod player;
mod state;
mod ui;

use maze::Room;
use player::Player;
use state::{AppState, PanelResult, ScreenMode};

fn window_conf() -> Conf {
    Conf {
        window_title: "Grammar Quest — Grammar Maze".to_owned(),
        window_width: 1280,
        window_height: 800,
        ..Default::default()
    }
}

struct Particle {
    pos: Vec2,
    vel: Vec2,
    color: Color,
    life: f32,
    max_life: f32,
    size: f32,
}

fn spawn_spark_burst(particles: &mut Vec<Particle>, center: Vec2, base_color: Color) {
    for _ in 0..28 {
        let angle = rand::gen_range(0.0, std::f32::consts::TAU);
        let speed = rand::gen_range(60.0, 220.0);
        let vel = vec2(angle.cos() * speed, angle.sin() * speed);
        let life = rand::gen_range(0.3, 0.7);
        particles.push(Particle {
            pos: center,
            vel,
            color: base_color,
            life,
            max_life: life,
            size: rand::gen_range(2.0, 5.0),
        });
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = AppState::new();

    // Embed textures so game runs reliably anywhere
    let player_tex = Texture2D::from_file_with_format(
        include_bytes!("../assets/player/swordsman_walk.png"),
        None,
    );
    player_tex.set_filter(FilterMode::Nearest);

    let floor_tex =
        Texture2D::from_file_with_format(include_bytes!("../assets/tiles/floor_metal.png"), None);
    floor_tex.set_filter(FilterMode::Nearest);

    let wall_tex =
        Texture2D::from_file_with_format(include_bytes!("../assets/tiles/wall_glass.png"), None);
    wall_tex.set_filter(FilterMode::Nearest);

    let mut current_room: Option<Room> = None;
    let mut player: Option<Player> = None;
    let mut particles: Vec<Particle> = Vec::new();

    loop {
        let dt = get_frame_time().min(0.05);

        clear_background(Color::from_rgba(13, 7, 24, 255));

        // Global Esc key behavior
        if is_key_pressed(KeyCode::Escape) {
            match state.mode {
                ScreenMode::Laboratory => break,
                ScreenMode::Playing | ScreenMode::Won => {
                    state.back_to_lab();
                }
            }
        }

        match state.mode {
            ScreenMode::Laboratory => {
                egui_macroquad::ui(|ctx| {
                    ui::theme::apply(ctx);
                    ui::side_panel::show_side_panel(ctx, state.result.as_ref());
                    egui::SidePanel::left("grammar_editor")
                        .default_width(340.0)
                        .show(ctx, |ui| match ui::editor::show_editor(ui, &mut state) {
                            ui::editor::EditorAction::PlayMaze => {
                                if state.start_maze().is_ok()
                                    && let (Some(g), Some(s)) =
                                        (&state.current_grammar, &state.derivation_state)
                                {
                                    let room = Room::build(g, s, 410.0);
                                    player = Some(Player::new(room.spawn_pos));
                                    current_room = Some(room);
                                    particles.clear();
                                }
                            }
                            ui::editor::EditorAction::Generate => {
                                state.generate();
                            }
                            ui::editor::EditorAction::None => {}
                        });
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.horizontal(|ui| {
                            ui.heading("Grammar Maze");
                            ui.separator();
                            ui.label(
                                egui::RichText::new("LABORATÓRIO DE DERIVAÇÃO")
                                    .color(egui::Color32::from_rgb(238, 166, 255)),
                            );
                        });
                        ui.separator();
                        ui.vertical_centered(|ui| {
                            ui.add_space(72.0);
                            ui::side_panel::show_result(ui, state.result.as_ref());
                        });
                    });
                });
            }

            ScreenMode::Playing | ScreenMode::Won => {
                // Game logic
                if state.mode == ScreenMode::Playing
                    && let (Some(plyr), Some(room)) = (&mut player, &current_room)
                {
                    plyr.update(dt, &room.walls);

                    if let Some(door) = room.check_door_collision(plyr.bounding_rect()) {
                        let door_center = vec2(
                            door.rect.x + door.rect.w / 2.0,
                            door.rect.y + door.rect.h / 2.0,
                        );
                        if door.is_exit {
                            state.complete_maze();
                            spawn_spark_burst(
                                &mut particles,
                                door_center,
                                Color::from_rgba(116, 255, 191, 255),
                            );
                        } else {
                            let choice = door.choice_index;
                            spawn_spark_burst(
                                &mut particles,
                                door_center,
                                Color::from_rgba(74, 229, 255, 255),
                            );

                            if let Ok(_is_complete) = state.apply_maze_choice(choice)
                                && let (Some(g), Some(s)) =
                                    (&state.current_grammar, &state.derivation_state)
                            {
                                let new_room = Room::build(g, s, 410.0);
                                plyr.pos = new_room.spawn_pos;
                                current_room = Some(new_room);
                            }
                        }
                    }
                }

                // Update particles
                particles.retain_mut(|p| {
                    p.pos += p.vel * dt;
                    p.life -= dt;
                    p.life > 0.0
                });

                // 2D World Rendering
                if let Some(ref room) = current_room {
                    // Draw room floor tiles
                    let tile_sz = 64.0;
                    let cols = (room.bounds.w / tile_sz).ceil() as usize;
                    let rows = (room.bounds.h / tile_sz).ceil() as usize;

                    for r in 0..rows {
                        for c in 0..cols {
                            let tx = room.bounds.x + c as f32 * tile_sz;
                            let ty = room.bounds.y + r as f32 * tile_sz;
                            draw_texture_ex(
                                &floor_tex,
                                tx,
                                ty,
                                Color::from_rgba(180, 170, 210, 255),
                                DrawTextureParams {
                                    source: Some(Rect::new(0.0, 0.0, tile_sz, tile_sz)),
                                    dest_size: Some(vec2(tile_sz, tile_sz)),
                                    ..Default::default()
                                },
                            );
                        }
                    }

                    // Floor grid accent
                    draw_rectangle_lines(
                        room.bounds.x,
                        room.bounds.y,
                        room.bounds.w,
                        room.bounds.h,
                        3.0,
                        Color::from_rgba(74, 229, 255, 80),
                    );

                    // Floor emblem for current non-terminal
                    if let Some(ref nt) = room.non_terminal {
                        let emblem_text = format!("EXPANDINDO NÃO-TERMINAL: [ {nt} ]");
                        let dim = measure_text(&emblem_text, None, 22, 1.0);
                        let ex = room.bounds.x + (room.bounds.w - dim.width) / 2.0;
                        let ey = room.bounds.y + room.bounds.h - 40.0;
                        draw_text(
                            &emblem_text,
                            ex,
                            ey,
                            22.0,
                            Color::from_rgba(238, 166, 255, 160),
                        );
                    }

                    // Draw walls
                    for wall in &room.walls {
                        // Wall texture tiled
                        let w_cols = (wall.w / tile_sz).ceil() as usize;
                        let w_rows = (wall.h / tile_sz).ceil() as usize;
                        for wr in 0..w_rows {
                            for wc in 0..w_cols {
                                let wx = wall.x + wc as f32 * tile_sz;
                                let wy = wall.y + wr as f32 * tile_sz;
                                let dw = tile_sz.min(wall.x + wall.w - wx);
                                let dh = tile_sz.min(wall.y + wall.h - wy);
                                draw_texture_ex(
                                    &wall_tex,
                                    wx,
                                    wy,
                                    Color::from_rgba(120, 100, 160, 255),
                                    DrawTextureParams {
                                        source: Some(Rect::new(0.0, 0.0, dw, dh)),
                                        dest_size: Some(vec2(dw, dh)),
                                        ..Default::default()
                                    },
                                );
                            }
                        }
                        // Wall outline glow
                        draw_rectangle_lines(
                            wall.x,
                            wall.y,
                            wall.w,
                            wall.h,
                            2.0,
                            Color::from_rgba(211, 70, 255, 120),
                        );
                    }

                    // Draw doors & production signs
                    for door in &room.doors {
                        let portal_color = if door.is_exit {
                            Color::from_rgba(116, 255, 191, 200)
                        } else {
                            Color::from_rgba(74, 229, 255, 200)
                        };

                        // Glowing doorway threshold
                        draw_rectangle(
                            door.rect.x,
                            door.rect.y + 4.0,
                            door.rect.w,
                            door.rect.h - 8.0,
                            Color::from_rgba(25, 19, 45, 230),
                        );
                        draw_rectangle_lines(
                            door.rect.x,
                            door.rect.y + 4.0,
                            door.rect.w,
                            door.rect.h - 8.0,
                            3.0,
                            portal_color,
                        );

                        // Floating Neon Sign above the door
                        let sign_w = door.rect.w + 20.0;
                        let sign_h = 32.0;
                        let sign_x = door.rect.x + (door.rect.w - sign_w) / 2.0;
                        let sign_y = door.rect.y - 36.0;

                        draw_rectangle(
                            sign_x,
                            sign_y,
                            sign_w,
                            sign_h,
                            Color::from_rgba(20, 12, 38, 240),
                        );
                        draw_rectangle_lines(
                            sign_x,
                            sign_y,
                            sign_w,
                            sign_h,
                            2.0,
                            if door.is_exit {
                                Color::from_rgba(116, 255, 191, 255)
                            } else {
                                Color::from_rgba(238, 166, 255, 255)
                            },
                        );

                        // Draw sign text centered
                        let text_dim = measure_text(&door.label, None, 20, 1.0);
                        let tx = sign_x + (sign_w - text_dim.width) / 2.0;
                        let ty = sign_y + (sign_h + text_dim.height) / 2.0 - 2.0;
                        draw_text(
                            &door.label,
                            tx,
                            ty,
                            20.0,
                            if door.is_exit {
                                Color::from_rgba(116, 255, 191, 255)
                            } else {
                                Color::from_rgba(74, 229, 255, 255)
                            },
                        );
                    }

                    // Draw particles
                    for p in &particles {
                        let alpha = p.life / p.max_life;
                        let mut c = p.color;
                        c.a = alpha;
                        draw_circle(p.pos.x, p.pos.y, p.size * alpha, c);
                    }

                    // Draw player
                    if let Some(ref plyr) = player {
                        plyr.draw(Some(&player_tex));
                    }
                }

                // UI Overlay (egui)
                egui_macroquad::ui(|ctx| {
                    ui::theme::apply(ctx);

                    // Side panel showing live derivation
                    if state.show_side_panel {
                        let live_result = if let (Some(s), Some(g)) =
                            (&state.derivation_state, &state.current_grammar)
                        {
                            Some(PanelResult {
                                sentence: s.output.clone(),
                                regex: grammar_engine::to_regex(g).unwrap_or_default(),
                                steps: s.steps.clone(),
                            })
                        } else {
                            state.result.clone()
                        };
                        ui::side_panel::show_side_panel(ctx, live_result.as_ref());
                    }

                    // HUD and victory overlay
                    match ui::game_hud::show_hud(ctx, &mut state) {
                        ui::game_hud::HudAction::BackToLab => {
                            state.back_to_lab();
                        }
                        ui::game_hud::HudAction::PlayAgain => {
                            state.reset_maze();
                            if let (Some(g), Some(s)) =
                                (&state.current_grammar, &state.derivation_state)
                            {
                                let room = Room::build(g, s, 410.0);
                                if let Some(ref mut plyr) = player {
                                    plyr.pos = room.spawn_pos;
                                }
                                current_room = Some(room);
                                particles.clear();
                            }
                        }
                        ui::game_hud::HudAction::None => {}
                    }
                });
            }
        }

        egui_macroquad::draw();
        next_frame().await;
    }
}
