use macroquad::prelude::*;

use crate::maze::{DoorKind, Room};
use crate::player::Player;

/// Draws every door's threshold, frame, side pylons, light beam, overhead
/// marquee, and (when the player is close enough) the "enter" prompt.
/// `global_timer` drives the scanline and beam animation.
pub fn draw_doors(room: &Room, player: Option<&Player>, global_timer: f32) {
    for door in &room.doors {
        let is_exit = door.kind == DoorKind::Exit;
        let theme_color = if is_exit {
            Color::from_rgba(52, 255, 180, 255)
        } else {
            Color::from_rgba(0, 240, 255, 255)
        };

        let center_x = door.rect.x + door.rect.w / 2.0;

        draw_rectangle(
            door.rect.x + 4.0,
            door.rect.y + 4.0,
            door.rect.w - 8.0,
            door.rect.h - 8.0,
            Color::from_rgba(18, 25, 42, 240),
        );

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

        draw_rectangle_lines(
            door.rect.x + 4.0,
            door.rect.y + 4.0,
            door.rect.w - 8.0,
            door.rect.h - 8.0,
            2.5,
            theme_color,
        );

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

        let sign_w = door.rect.w;
        let base_font_size = if is_exit { 14 } else { 17 };
        // Macroquad's built-in ProggyClean font lacks the Unicode arrow glyph.
        let display_label = door.label.replace('→', "->");
        let measured = measure_text(&display_label, None, base_font_size, 1.0);
        let font_size = (base_font_size as f32
            * ((sign_w - 16.0) / measured.width.max(1.0)).min(1.0))
        .max(9.0) as u16;
        let label_size = measure_text(&display_label, None, font_size, 1.0);
        let sign_h = label_size.height + 10.0;
        let sign_x = center_x - sign_w / 2.0;
        let sign_y = door.rect.y - sign_h - 8.0;

        draw_rectangle(
            sign_x,
            sign_y,
            sign_w,
            sign_h,
            Color::from_rgba(10, 7, 20, 235),
        );
        draw_rectangle_lines(sign_x, sign_y, sign_w, sign_h, 1.5, theme_color);
        draw_text(
            &display_label,
            center_x - label_size.width / 2.0,
            sign_y + sign_h - 5.0,
            font_size as f32,
            theme_color,
        );

        if let Some(plyr) = player {
            let dist = (plyr.pos - vec2(center_x, door.rect.y + door.rect.h)).length();
            if dist < 120.0 {
                let prompt_text = "ENTRAR [W / CIMA]";
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
}
