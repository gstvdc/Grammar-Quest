use macroquad::prelude::*;

use crate::maze::Room;

pub fn draw_ambient_dust(dust: &[Vec2]) {
    for d in dust {
        draw_circle(d.x, d.y, 1.4, Color::from_rgba(160, 120, 230, 45));
    }
}

/// Draws the arena floor, grid, central sigil, and walls of `room`.
/// `global_timer` drives the sigil pulse animation.
pub fn draw_arena(room: &Room, global_timer: f32) {
    draw_rectangle(
        room.bounds.x,
        room.bounds.y,
        room.bounds.w,
        room.bounds.h,
        Color::from_rgba(15, 10, 28, 255),
    );

    let arena_center = vec2(
        room.bounds.x + room.bounds.w / 2.0,
        room.bounds.y + room.bounds.h / 2.0 + 20.0,
    );

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

    let pulse = (global_timer * 2.2).sin() * 0.15 + 0.85;
    let sigil_r = 72.0 * pulse;

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

    for wall in &room.walls {
        draw_rectangle(
            wall.x,
            wall.y,
            wall.w,
            wall.h,
            Color::from_rgba(20, 14, 36, 255),
        );

        draw_rectangle(
            wall.x + 2.0,
            wall.y + 2.0,
            wall.w - 4.0,
            wall.h - 4.0,
            Color::from_rgba(30, 20, 52, 255),
        );

        draw_rectangle_lines(
            wall.x,
            wall.y,
            wall.w,
            wall.h,
            1.5,
            Color::from_rgba(100, 60, 150, 140),
        );
    }
}
