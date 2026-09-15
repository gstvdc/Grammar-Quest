use macroquad::miniquad::conf::Icon;
use macroquad::prelude::*;

mod maze;
mod player;
mod state;
mod ui;

use maze::Room;
use player::Player;
use state::{AppState, PanelResult, ScreenMode};

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

struct Particle {
    pos: Vec2,
    vel: Vec2,
    color: Color,
    life: f32,
    max_life: f32,
    size: f32,
}

struct Shockwave {
    pos: Vec2,
    radius: f32,
    max_radius: f32,
    color: Color,
}

struct FloatingText {
    pos: Vec2,
    text: String,
    color: Color,
    life: f32,
    max_life: f32,
}

fn spawn_spark_burst(particles: &mut Vec<Particle>, center: Vec2, base_color: Color) {
    for _ in 0..32 {
        let angle = rand::gen_range(0.0, std::f32::consts::TAU);
        let speed = rand::gen_range(60.0, 260.0);
        let vel = vec2(angle.cos() * speed, angle.sin() * speed);
        let life = rand::gen_range(0.35, 0.75);
        particles.push(Particle {
            pos: center,
            vel,
            color: base_color,
            life,
            max_life: life,
            size: rand::gen_range(2.5, 6.0),
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

    let mut current_room: Option<Room> = None;
    let mut player: Option<Player> = None;
    let mut particles: Vec<Particle> = Vec::new();
    let mut shockwaves: Vec<Shockwave> = Vec::new();
    let mut floating_texts: Vec<FloatingText> = Vec::new();

    // Ambient background cyber-dust
    let mut ambient_dust: Vec<Vec2> = (0..40)
        .map(|_| vec2(rand::gen_range(0.0, 1280.0), rand::gen_range(0.0, 800.0)))
        .collect();

    let mut global_timer: f32 = 0.0;

    loop {
        let dt = get_frame_time().min(0.05);
        global_timer += dt;
        state.advance_animations(dt);

        clear_background(Color::from_rgba(10, 7, 18, 255));

        // Global Esc key behavior
        if is_key_pressed(KeyCode::Escape) {
            match state.mode {
                ScreenMode::Laboratory => break,
                ScreenMode::Playing | ScreenMode::Won => {
                    state.back_to_lab();
                }
            }
        }

        // Tab hotkey to toggle side panel during gameplay
        if is_key_pressed(KeyCode::Tab) && state.mode != ScreenMode::Laboratory {
            state.show_side_panel = !state.show_side_panel;
        }

        // Slowly drift ambient dust
        for dust in &mut ambient_dust {
            dust.y -= 12.0 * dt;
            if dust.y < 0.0 {
                dust.y = 800.0;
                dust.x = rand::gen_range(0.0, 1280.0);
            }
        }

        match state.mode {
            ScreenMode::Laboratory => {
                // Draw subtle ambient background stars in lab
                for dust in &ambient_dust {
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
                        .show(ctx, |ui| match ui::editor::show_editor(ui, &mut state) {
                            ui::editor::EditorAction::PlayMaze => {
                                if state.start_maze().is_ok()
                                    && let (Some(g), Some(s)) =
                                        (&state.current_grammar, &state.derivation_state)
                                {
                                    let room =
                                        Room::build(g, s, 1280.0, 800.0, state.puzzle.as_ref());
                                    player = Some(Player::new(room.spawn_pos));
                                    current_room = Some(room);
                                    particles.clear();
                                    shockwaves.clear();
                                    floating_texts.clear();
                                }
                            }
                            ui::editor::EditorAction::Generate => {
                                state.generate();
                            }
                            ui::editor::EditorAction::None => {}
                        });

                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.add_space(8.0);
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new("⚡ GRAMMAR QUEST")
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
                        ui.add_space(24.0);

                        ui.vertical_centered(|ui| {
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

                    // Footstep dust when moving
                    if plyr.is_moving && rand::gen_range(0.0, 1.0) < 0.25 {
                        particles.push(Particle {
                            pos: plyr.pos + vec2(rand::gen_range(-8.0, 8.0), 16.0),
                            vel: vec2(rand::gen_range(-15.0, 15.0), rand::gen_range(-10.0, 10.0)),
                            color: Color::from_rgba(0, 240, 255, 120),
                            life: 0.3,
                            max_life: 0.3,
                            size: 2.0,
                        });
                    }

                    // Check door collision
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
                                Color::from_rgba(52, 255, 180, 255),
                            );
                            shockwaves.push(Shockwave {
                                pos: door_center,
                                radius: 10.0,
                                max_radius: 380.0,
                                color: Color::from_rgba(52, 255, 180, 220),
                            });
                        } else if door.is_puzzle {
                            let correct = state.answer_puzzle(door.choice_index).unwrap_or(false);
                            let color = if correct {
                                Color::from_rgba(52, 255, 180, 255)
                            } else {
                                Color::from_rgba(255, 90, 130, 255)
                            };
                            spawn_spark_burst(&mut particles, door_center, color);
                            if correct {
                                state.complete_maze();
                                shockwaves.push(Shockwave {
                                    pos: door_center,
                                    radius: 10.0,
                                    max_radius: 380.0,
                                    color,
                                });
                            } else {
                                plyr.pos = room.spawn_pos;
                                floating_texts.push(FloatingText {
                                    pos: door_center + vec2(0.0, -20.0),
                                    text: "Essa sentença não pertence à linguagem".to_string(),
                                    color,
                                    life: 1.4,
                                    max_life: 1.4,
                                });
                            }
                        } else {
                            let choice = door.choice_index;

                            // Feedback effects
                            spawn_spark_burst(
                                &mut particles,
                                door_center,
                                Color::from_rgba(0, 240, 255, 255),
                            );
                            shockwaves.push(Shockwave {
                                pos: door_center,
                                radius: 10.0,
                                max_radius: 300.0,
                                color: Color::from_rgba(0, 240, 255, 200),
                            });

                            let prev_output_len = state
                                .derivation_state
                                .as_ref()
                                .map_or(0, |s| s.output.len());

                            if let Ok(_is_complete) = state.apply_maze_choice(choice)
                                && let (Some(g), Some(s)) =
                                    (&state.current_grammar, &state.derivation_state)
                            {
                                // Floating text showing emitted terminal
                                if s.output.len() > prev_output_len {
                                    let added: String = s.output[prev_output_len..].to_string();
                                    floating_texts.push(FloatingText {
                                        pos: door_center + vec2(0.0, -20.0),
                                        text: format!("+ \"{added}\""),
                                        color: Color::from_rgba(52, 255, 180, 255),
                                        life: 1.2,
                                        max_life: 1.2,
                                    });
                                }

                                let new_room =
                                    Room::build(g, s, 1280.0, 800.0, state.puzzle.as_ref());
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

                // Update shockwaves
                shockwaves.retain_mut(|s| {
                    s.radius += 380.0 * dt;
                    s.radius < s.max_radius
                });

                // Update floating texts
                floating_texts.retain_mut(|ft| {
                    ft.pos.y -= 32.0 * dt;
                    ft.life -= dt;
                    ft.life > 0.0
                });

                // 2D World Rendering
                // 1. Cosmic ambient background dust
                for dust in &ambient_dust {
                    draw_circle(dust.x, dust.y, 1.4, Color::from_rgba(160, 120, 230, 45));
                }

                if let Some(ref room) = current_room {
                    // Floor: Sleek dark cyber slate platform
                    draw_rectangle(
                        room.bounds.x,
                        room.bounds.y,
                        room.bounds.w,
                        room.bounds.h,
                        Color::from_rgba(15, 10, 28, 255),
                    );

                    // Ambient soft radial illumination in the center of arena
                    let arena_center = vec2(
                        room.bounds.x + room.bounds.w / 2.0,
                        room.bounds.y + room.bounds.h / 2.0 + 20.0,
                    );

                    if room.doors.iter().any(|door| door.is_puzzle) {
                        let prompt = "DESAFIO FINAL — QUAL SENTENÇA PERTENCE À LINGUAGEM?";
                        let prompt_size = measure_text(prompt, None, 16, 1.0);
                        draw_text(
                            prompt,
                            arena_center.x - prompt_size.width / 2.0,
                            room.bounds.y + 78.0,
                            16.0,
                            Color::from_rgba(238, 166, 255, 255),
                        );
                    }

                    draw_circle(
                        arena_center.x,
                        arena_center.y,
                        240.0,
                        Color::from_rgba(32, 20, 56, 120),
                    );
                    draw_circle(
                        arena_center.x,
                        arena_center.y,
                        140.0,
                        Color::from_rgba(45, 25, 75, 90),
                    );

                    // Sleek cyber grid lines across the floor (48px spacing)
                    let grid_sz = 48.0;
                    let x_start = room.bounds.x + 32.0;
                    let x_end = room.bounds.x + room.bounds.w - 32.0;
                    let y_start = room.bounds.y + 32.0;
                    let y_end = room.bounds.y + room.bounds.h - 32.0;

                    let mut gx = x_start;
                    while gx <= x_end {
                        draw_line(
                            gx,
                            y_start,
                            gx,
                            y_end,
                            1.0,
                            Color::from_rgba(60, 40, 100, 35),
                        );
                        gx += grid_sz;
                    }

                    let mut gy = y_start;
                    while gy <= y_end {
                        draw_line(
                            x_start,
                            gy,
                            x_end,
                            gy,
                            1.0,
                            Color::from_rgba(60, 40, 100, 35),
                        );
                        gy += grid_sz;
                    }

                    // Center Holographic Summoning Sigil / Node
                    let pulse = (global_timer * 2.2).sin() * 0.15 + 0.85;
                    let sigil_r = 72.0 * pulse;

                    // Outer pulsing ring with dashes
                    draw_circle_lines(
                        arena_center.x,
                        arena_center.y,
                        sigil_r,
                        2.0,
                        Color::from_rgba(192, 132, 252, 90),
                    );
                    draw_circle_lines(
                        arena_center.x,
                        arena_center.y,
                        sigil_r * 0.75,
                        1.0,
                        Color::from_rgba(0, 240, 255, 60),
                    );

                    // Central glowing Non-Terminal symbol
                    if let Some(ref nt) = room.non_terminal {
                        let text_dim = measure_text(nt, None, 44, 1.0);
                        draw_text(
                            nt,
                            arena_center.x - text_dim.width / 2.0,
                            arena_center.y + text_dim.height / 2.0 - 2.0,
                            44.0,
                            Color::from_rgba(238, 166, 255, 230),
                        );

                        let label_text = "NÃO-TERMINAL ATIVO";
                        let sub_dim = measure_text(label_text, None, 13, 1.0);
                        draw_text(
                            label_text,
                            arena_center.x - sub_dim.width / 2.0,
                            arena_center.y + sigil_r + 20.0,
                            13.0,
                            Color::from_rgba(160, 130, 210, 150),
                        );
                    }

                    // Draw solid cyber walls
                    for wall in &room.walls {
                        // Base solid dark slate fill
                        draw_rectangle(
                            wall.x,
                            wall.y,
                            wall.w,
                            wall.h,
                            Color::from_rgba(20, 14, 36, 255),
                        );

                        // Inner subtle bevel
                        draw_rectangle(
                            wall.x + 2.0,
                            wall.y + 2.0,
                            wall.w - 4.0,
                            wall.h - 4.0,
                            Color::from_rgba(30, 20, 52, 255),
                        );

                        // Wall border outline glow
                        draw_rectangle_lines(
                            wall.x,
                            wall.y,
                            wall.w,
                            wall.h,
                            1.5,
                            Color::from_rgba(100, 60, 150, 140),
                        );
                    }

                    // Draw Gateways & Overhead Hologram Signs
                    for door in &room.doors {
                        let is_exit = door.is_exit;
                        let theme_color = if is_exit {
                            Color::from_rgba(52, 255, 180, 255)
                        } else if door.is_puzzle {
                            Color::from_rgba(192, 132, 252, 255)
                        } else {
                            Color::from_rgba(0, 240, 255, 255)
                        };

                        let center_x = door.rect.x + door.rect.w / 2.0;

                        // Gateway threshold energy pad on the floor
                        draw_rectangle(
                            door.rect.x + 4.0,
                            door.rect.y + 4.0,
                            door.rect.w - 8.0,
                            door.rect.h - 8.0,
                            Color::from_rgba(18, 25, 42, 240),
                        );

                        // Animated shimmering forcefield lines inside threshold
                        let scanline_offset = (global_timer * 40.0) % (door.rect.h - 12.0);
                        draw_line(
                            door.rect.x + 8.0,
                            door.rect.y + 6.0 + scanline_offset,
                            door.rect.x + door.rect.w - 8.0,
                            door.rect.y + 6.0 + scanline_offset,
                            2.0,
                            Color::from_rgba(
                                theme_color.r as u8,
                                theme_color.g as u8,
                                theme_color.b as u8,
                                160,
                            ),
                        );

                        // Gateway frame border
                        draw_rectangle_lines(
                            door.rect.x + 4.0,
                            door.rect.y + 4.0,
                            door.rect.w - 8.0,
                            door.rect.h - 8.0,
                            2.5,
                            theme_color,
                        );

                        // Side energy pylons
                        let pylon_w = 8.0;
                        draw_rectangle(
                            door.rect.x - pylon_w,
                            door.rect.y,
                            pylon_w,
                            door.rect.h,
                            Color::from_rgba(40, 25, 70, 255),
                        );
                        draw_rectangle(
                            door.rect.x + door.rect.w,
                            door.rect.y,
                            pylon_w,
                            door.rect.h,
                            Color::from_rgba(40, 25, 70, 255),
                        );
                        draw_circle(
                            door.rect.x - pylon_w / 2.0,
                            door.rect.y + door.rect.h / 2.0,
                            3.0,
                            theme_color,
                        );
                        draw_circle(
                            door.rect.x + door.rect.w + pylon_w / 2.0,
                            door.rect.y + door.rect.h / 2.0,
                            3.0,
                            theme_color,
                        );

                        // Soft light beam shining down into the arena
                        draw_triangle(
                            vec2(door.rect.x + 8.0, door.rect.y + door.rect.h),
                            vec2(door.rect.x + door.rect.w - 8.0, door.rect.y + door.rect.h),
                            vec2(center_x, door.rect.y + door.rect.h + 50.0),
                            Color::from_rgba(
                                (theme_color.r * 255.0) as u8,
                                (theme_color.g * 255.0) as u8,
                                (theme_color.b * 255.0) as u8,
                                22,
                            ),
                        );

                        // Overhead Floating Holographic Marquee (Sign)
                        let sign_w = (door.rect.w + 24.0).max(140.0);
                        let font_size = if is_exit {
                            14
                        } else if door.is_puzzle {
                            13
                        } else {
                            17
                        };
                        let mut label_lines = vec![String::new()];
                        for character in door.label.chars() {
                            let line = label_lines.last_mut().expect("one label line exists");
                            let mut candidate = line.clone();
                            candidate.push(character);
                            if measure_text(&candidate, None, font_size, 1.0).width > sign_w - 12.0
                                && !line.is_empty()
                            {
                                label_lines.push(character.to_string());
                            } else {
                                line.push(character);
                            }
                        }
                        let sign_h =
                            (label_lines.len() as f32 * (font_size as f32 + 2.0) + 8.0).max(34.0);
                        let sign_x = center_x - sign_w / 2.0;
                        let sign_y = door.rect.y - sign_h - 10.0;

                        // Glassmorphism card backdrop
                        draw_rectangle(
                            sign_x,
                            sign_y,
                            sign_w,
                            sign_h,
                            Color::from_rgba(12, 8, 22, 240),
                        );
                        draw_rectangle_lines(
                            sign_x,
                            sign_y,
                            sign_w,
                            sign_h,
                            1.5,
                            if is_exit {
                                Color::from_rgba(52, 255, 180, 220)
                            } else {
                                Color::from_rgba(192, 132, 252, 220)
                            },
                        );

                        // Sign text
                        let text_color = if is_exit {
                            Color::from_rgba(52, 255, 180, 255)
                        } else {
                            Color::from_rgba(240, 230, 255, 255)
                        };
                        for (index, line) in label_lines.iter().enumerate() {
                            let text_dim = measure_text(line, None, font_size, 1.0);
                            draw_text(
                                line,
                                sign_x + (sign_w - text_dim.width) / 2.0,
                                sign_y + 6.0 + (index as f32 + 1.0) * (font_size as f32 + 1.0),
                                font_size as f32,
                                text_color,
                            );
                        }

                        // Proximity prompt if player is near
                        if let Some(ref plyr) = player {
                            let dist =
                                (plyr.pos - vec2(center_x, door.rect.y + door.rect.h)).length();
                            if dist < 120.0 {
                                let prompt_text = "ENTRAR [W / ↑]";
                                let pdim = measure_text(prompt_text, None, 12, 1.0);
                                let px = center_x - pdim.width / 2.0;
                                let py = door.rect.y + door.rect.h + 24.0;
                                draw_rectangle(
                                    px - 6.0,
                                    py - pdim.height - 2.0,
                                    pdim.width + 12.0,
                                    pdim.height + 6.0,
                                    Color::from_rgba(10, 7, 20, 220),
                                );
                                draw_text(
                                    prompt_text,
                                    px,
                                    py,
                                    12.0,
                                    Color::from_rgba(0, 240, 255, 240),
                                );
                            }
                        }
                    }

                    // Draw shockwaves
                    for s in &shockwaves {
                        let alpha = 1.0 - (s.radius / s.max_radius);
                        let mut c = s.color;
                        c.a = alpha;
                        draw_circle_lines(s.pos.x, s.pos.y, s.radius, 2.5, c);
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

                    // Draw floating text feedback
                    for ft in &floating_texts {
                        let alpha = ft.life / ft.max_life;
                        let mut c = ft.color;
                        c.a = alpha;
                        let dim = measure_text(&ft.text, None, 22, 1.0);
                        draw_text(&ft.text, ft.pos.x - dim.width / 2.0, ft.pos.y, 22.0, c);
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
                                let room = Room::build(g, s, 1280.0, 800.0, state.puzzle.as_ref());
                                if let Some(ref mut plyr) = player {
                                    plyr.pos = room.spawn_pos;
                                }
                                current_room = Some(room);
                                particles.clear();
                                shockwaves.clear();
                                floating_texts.clear();
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
