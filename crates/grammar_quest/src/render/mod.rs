pub mod portals;
pub mod world;

use macroquad::prelude::*;

use crate::effects::EffectsState;
use crate::maze::Room;
use crate::player::Player;

/// Draws one frame of the `ScreenMode::Playing` / `ScreenMode::Won` world:
/// ambient dust, arena, doors, effects, and the player. Purely a renderer —
/// it reads `room`/`player`/`effects` but never mutates `AppState`.
pub fn draw_playing_scene(
    ambient_dust: &[Vec2],
    room: &Room,
    player: Option<&Player>,
    player_tex: &Texture2D,
    effects: &EffectsState,
    global_timer: f32,
) {
    world::draw_ambient_dust(ambient_dust);
    world::draw_arena(room, global_timer);
    portals::draw_doors(room, player, global_timer);
    effects.draw_particles_and_shockwaves();

    if let Some(plyr) = player {
        plyr.draw(Some(player_tex));
    }

    effects.draw_floating_texts();
}
