use grammar_engine::{DerivationState, Grammar};
use macroquad::prelude::*;

/// Every door a player can walk into is exactly one of these.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DoorKind {
    /// Applies `choice_index` as a production choice via `apply_maze_choice`.
    Production,
    /// The stack is already empty; walking here already won (see
    /// `gameplay::update_playing`, which returns early for this kind).
    Exit,
}

#[derive(Debug, Clone)]
pub struct Door {
    pub choice_index: usize,
    pub label: String,
    pub rect: Rect,
    pub kind: DoorKind,
}

#[derive(Debug, Clone)]
pub struct Room {
    pub bounds: Rect,
    pub walls: Vec<Rect>,
    pub doors: Vec<Door>,
    pub non_terminal: Option<String>,
    pub spawn_pos: Vec2,
}

impl Room {
    #[cfg(test)]
    pub fn build(
        grammar: &Grammar,
        state: &DerivationState,
        viewport_width: f32,
        viewport_height: f32,
    ) -> Self {
        Self::build_with_panels(grammar, state, viewport_width, viewport_height, 0.0, 0.0)
    }

    pub fn build_with_panels(
        grammar: &Grammar,
        state: &DerivationState,
        viewport_width: f32,
        viewport_height: f32,
        left_panel_width: f32,
        right_panel_width: f32,
    ) -> Self {
        // The game canvas is anchored to the usable corridor rather than
        // centred in the window, so ultrawide displays do not get a dead area
        // beside the map or let the arena run beneath a HUD panel.
        let left_margin = if left_panel_width > 0.0 { 40.0 } else { 20.0 };
        let right_margin = if right_panel_width > 0.0 { 40.0 } else { 20.0 };
        let room_width =
            (viewport_width - left_panel_width - right_panel_width - left_margin - right_margin)
                .max(560.0);
        let room_height = (viewport_height - 140.0).clamp(420.0, 600.0);

        let room_x = left_panel_width + left_margin;
        let room_y = 90.0; // Starts below the 64px HUD with clean breathing room
        let bounds = Rect::new(room_x, room_y, room_width, room_height);

        let wall_thickness = 32.0;

        // Solid perimeter walls for left, right, and bottom
        let mut walls = vec![
            // Left wall
            Rect::new(room_x, room_y, wall_thickness, room_height),
            // Right wall
            Rect::new(
                room_x + room_width - wall_thickness,
                room_y,
                wall_thickness,
                room_height,
            ),
            // Bottom wall
            Rect::new(
                room_x,
                room_y + room_height - wall_thickness,
                room_width,
                wall_thickness,
            ),
        ];

        let spawn_pos = Vec2::new(room_x + room_width / 2.0, room_y + room_height - 90.0);
        let mut doors = Vec::new();

        if let Some(nt) = state.current_non_terminal() {
            let alternatives = grammar.alternatives(&nt).cloned().unwrap_or_default();
            let count = alternatives.len().max(1);

            let door_gap = 24.0;
            let usable_width = room_width - 2.0 * wall_thickness - 60.0;
            let door_width = ((usable_width - door_gap * (count as f32 - 1.0)) / (count as f32))
                .clamp(80.0, 220.0);
            let door_height = 48.0;
            let total_doors_width = door_width * count as f32 + door_gap * (count as f32 - 1.0);
            let first_door_x = room_x + (room_width - total_doors_width) / 2.0;

            let mut prev_x = room_x;

            for (i, alt) in alternatives.iter().enumerate() {
                let door_x = first_door_x + i as f32 * (door_width + door_gap);
                let door_y = room_y;

                // Top wall segment between previous point and this door
                if door_x > prev_x {
                    walls.push(Rect::new(prev_x, room_y, door_x - prev_x, wall_thickness));
                }
                prev_x = door_x + door_width;

                let prod_symbols: String = alt.iter().map(ToString::to_string).collect();
                let label = if prod_symbols.is_empty() {
                    format!("{nt} → ε")
                } else {
                    format!("{nt} → {prod_symbols}")
                };

                doors.push(Door {
                    choice_index: i,
                    label,
                    rect: Rect::new(door_x, door_y, door_width, door_height),
                    kind: DoorKind::Production,
                });
            }

            // Top wall segment after the last door
            let end_x = room_x + room_width;
            if end_x > prev_x {
                walls.push(Rect::new(prev_x, room_y, end_x - prev_x, wall_thickness));
            }

            Room {
                bounds,
                walls,
                doors,
                non_terminal: Some(nt),
                spawn_pos,
            }
        } else {
            // Victory / Exit portal
            let door_width = 240.0;
            let door_height = 48.0;
            let door_x = room_x + (room_width - door_width) / 2.0;
            let door_y = room_y;

            // Top wall segments on either side of the exit
            walls.push(Rect::new(room_x, room_y, door_x - room_x, wall_thickness));
            walls.push(Rect::new(
                door_x + door_width,
                room_y,
                (room_x + room_width) - (door_x + door_width),
                wall_thickness,
            ));

            doors.push(Door {
                choice_index: 0,
                label: "★ SAÍDA DO LABIRINTO (VITÓRIA) ★".to_string(),
                rect: Rect::new(door_x, door_y, door_width, door_height),
                kind: DoorKind::Exit,
            });

            Room {
                bounds,
                walls,
                doors,
                non_terminal: None,
                spawn_pos,
            }
        }
    }

    pub fn check_door_collision(&self, player_rect: Rect) -> Option<&Door> {
        self.doors
            .iter()
            .find(|door| door.rect.overlaps(&player_rect))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use grammar_engine::parse_grammar;

    #[test]
    fn room_generates_one_door_per_alternative() {
        let grammar = parse_grammar("S -> aS | ab").unwrap();
        let state = DerivationState::new(&grammar);
        let room = Room::build(&grammar, &state, 1280.0, 800.0);

        assert_eq!(room.doors.len(), 2);
        assert_eq!(room.doors[0].choice_index, 0);
        assert_eq!(room.doors[0].label, "S → aS");
        assert_eq!(room.doors[1].choice_index, 1);
        assert_eq!(room.doors[1].label, "S → ab");
        assert_eq!(room.doors[0].kind, DoorKind::Production);
        assert_eq!(room.doors[1].kind, DoorKind::Production);
    }

    #[test]
    fn room_generates_exit_door_when_stack_is_empty() {
        let grammar = parse_grammar("S -> a").unwrap();
        let mut state = DerivationState::new(&grammar);
        state.apply_choice(&grammar, 0).unwrap();
        assert!(state.is_complete());

        let room = Room::build(&grammar, &state, 1280.0, 800.0);
        assert_eq!(room.doors.len(), 1);
        assert_eq!(room.doors[0].kind, DoorKind::Exit);
        assert_eq!(room.doors[0].label, "★ SAÍDA DO LABIRINTO (VITÓRIA) ★");
    }

    #[test]
    fn collision_detects_player_at_door() {
        let grammar = parse_grammar("S -> a").unwrap();
        let state = DerivationState::new(&grammar);
        let room = Room::build(&grammar, &state, 1280.0, 800.0);

        let door_rect = room.doors[0].rect;
        let player_at_door = Rect::new(door_rect.x + 10.0, door_rect.y + 10.0, 32.0, 32.0);
        let player_far_away = Rect::new(room.spawn_pos.x, room.spawn_pos.y, 32.0, 32.0);

        assert!(room.check_door_collision(player_at_door).is_some());
        assert!(room.check_door_collision(player_far_away).is_none());
    }

    #[test]
    fn five_choices_get_distinct_non_overlapping_portals() {
        let grammar =
            parse_grammar("S -> aA | bB | cC | dD | eE\nA -> a\nB -> b\nC -> c\nD -> d\nE -> e")
                .unwrap();
        let state = DerivationState::new(&grammar);
        let room = Room::build(&grammar, &state, 1280.0, 800.0);

        assert_eq!(room.doors.len(), 5);
        for pair in room.doors.windows(2) {
            assert!(
                pair[0].rect.x + pair[0].rect.w <= pair[1].rect.x,
                "portals must not overlap"
            );
        }
    }

    #[test]
    fn room_fits_a_narrow_game_viewport() {
        let grammar = parse_grammar("S -> aS | ab").unwrap();
        let state = DerivationState::new(&grammar);
        let room = Room::build(&grammar, &state, 800.0, 600.0);

        assert!(room.bounds.x >= 20.0);
        assert!(room.bounds.x + room.bounds.w <= 780.0);
        assert!(room.bounds.y >= 60.0);
    }

    #[test]
    fn room_stays_between_the_fixed_game_panels() {
        let grammar = parse_grammar("S -> aS | ab").unwrap();
        let state = DerivationState::new(&grammar);
        let room = Room::build_with_panels(&grammar, &state, 1440.0, 900.0, 180.0, 310.0);

        assert_eq!(room.bounds.x, 220.0);
        assert!(room.bounds.x + room.bounds.w <= 1_090.0);
    }
}
