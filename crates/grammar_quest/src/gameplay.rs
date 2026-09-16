use macroquad::prelude::*;

use crate::audio::Sfx;
use crate::effects::EffectsState;
use crate::maze::{DoorKind, Room};
use crate::player::Player;
use crate::state::{AppState, GameFlow, MazeChoiceOutcome};

/// Advances player movement, footstep dust, and door-collision resolution
/// for one frame of `ScreenMode::Playing`. Mutates `state` only through
/// `AppState::apply_maze_choice`, mirroring what `main.rs` did inline before
/// the split.
pub fn update_playing(
    state: &mut AppState,
    sfx: &Sfx,
    player: &mut Option<Player>,
    current_room: &mut Option<Room>,
    effects: &mut EffectsState,
    dt: f32,
) {
    let (Some(plyr), Some(room)) = (player.as_mut(), current_room.as_ref()) else {
        return;
    };

    plyr.update(dt, &room.walls);

    if plyr.is_moving && rand::gen_range(0.0, 1.0) < 0.25 {
        effects.particles.push(crate::effects::Particle {
            pos: plyr.pos + vec2(rand::gen_range(-8.0, 8.0), 16.0),
            vel: vec2(rand::gen_range(-15.0, 15.0), rand::gen_range(-10.0, 10.0)),
            color: Color::from_rgba(0, 240, 255, 120),
            life: 0.3,
            max_life: 0.3,
            size: 2.0,
        });
    }

    let Some(door) = room.check_door_collision(plyr.bounding_rect()) else {
        return;
    };

    // The exit door only appears once the stack is already empty, at which
    // point `apply_maze_choice` already flipped `state.mode` to `Won` on the
    // choice that emptied it — there is no separate "walk through the exit"
    // step.
    if door.kind == DoorKind::Exit {
        return;
    }

    // Standing on a wrong door's rect fires this every frame; the cooldown
    // prevents a single touch from restarting the secret route repeatedly.
    if state.door_cooldown_active() {
        return;
    }

    let door_center = vec2(
        door.rect.x + door.rect.w / 2.0,
        door.rect.y + door.rect.h / 2.0,
    );

    let choice = door.choice_index;

    effects.spawn_spark_burst(door_center, Color::from_rgba(0, 240, 255, 255));
    effects.push_shockwave(door_center, Color::from_rgba(0, 240, 255, 200), 300.0);

    let Ok(outcome) = state.apply_maze_choice(choice) else {
        return;
    };
    let (Some(g), Some(s)) = (&state.current_grammar, &state.derivation_state) else {
        return;
    };

    if outcome == MazeChoiceOutcome::Restarted {
        sfx.play_door_wrong(state.master_volume);
        effects.spawn_spark_burst(door_center, Color::from_rgba(255, 90, 130, 255));
        let message = if state.secret_last_checkpoint() > 0 {
            format!(
                "PORTA ERRADA — VOLTOU AO CHECKPOINT (PORTA {})",
                state.secret_last_checkpoint()
            )
        } else {
            "PORTA ERRADA — ROTA REINICIADA".to_string()
        };
        effects.push_floating_text(
            door_center + vec2(0.0, -20.0),
            &message,
            Color::from_rgba(255, 90, 130, 255),
        );
    }

    if outcome == MazeChoiceOutcome::Completed {
        sfx.play_victory(state.master_volume);
        effects.push_shockwave(door_center, Color::from_rgba(52, 255, 180, 220), 380.0);
    } else {
        if outcome == MazeChoiceOutcome::Advanced {
            sfx.play_door_correct(state.master_volume);
        }
        let (left_panel_width, right_panel_width) = match (state.show_side_panel, state.active_flow)
        {
            (true, Some(GameFlow::SecretChallenge)) => (180.0, 310.0),
            (true, _) => (340.0, 0.0),
            (false, _) => (0.0, 0.0),
        };
        let new_room = Room::build_with_panels(
            g,
            s,
            screen_width(),
            screen_height(),
            left_panel_width,
            right_panel_width,
        );
        plyr.pos = new_room.spawn_pos;
        *current_room = Some(new_room);
    }
}
