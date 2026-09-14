use grammar_engine::{DerivationState, Grammar};
use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct Door {
    pub choice_index: usize,
    pub label: String,
    pub rect: Rect,
    pub is_exit: bool,
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
    pub fn build(grammar: &Grammar, state: &DerivationState, room_center_y: f32) -> Self {
        let room_width = 896.0;
        let room_height = 640.0;
        let room_x = (1280.0 - room_width) / 2.0;
        let room_y = room_center_y - room_height / 2.0;
        let bounds = Rect::new(room_x, room_y, room_width, room_height);

        let wall_thickness = 48.0;
        let mut walls = Vec::new();

        // Left wall
        walls.push(Rect::new(room_x, room_y, wall_thickness, room_height));
        // Right wall
        walls.push(Rect::new(
            room_x + room_width - wall_thickness,
            room_y,
            wall_thickness,
            room_height,
        ));
        // Bottom wall
        walls.push(Rect::new(
            room_x,
            room_y + room_height - wall_thickness,
            room_width,
            wall_thickness,
        ));

        let spawn_pos = Vec2::new(room_x + room_width / 2.0, room_y + room_height - 110.0);
        let mut doors = Vec::new();

        if let Some(nt) = state.current_non_terminal() {
            let alternatives = grammar.alternatives(&nt).cloned().unwrap_or_default();
            let count = alternatives.len().max(1);
            let door_width = 160.0;
            let door_height = 48.0;

            let usable_width = room_width - 2.0 * wall_thickness - 60.0;
            let spacing = usable_width / (count as f32);

            let mut prev_x = room_x + wall_thickness;

            for (i, alt) in alternatives.iter().enumerate() {
                let center_x = room_x + wall_thickness + 30.0 + (i as f32 + 0.5) * spacing;
                let door_x = center_x - door_width / 2.0;
                let door_y = room_y;

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
                    is_exit: false,
                });
            }

            let end_x = room_x + room_width - wall_thickness;
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
            let door_width = 200.0;
            let door_height = 48.0;
            let door_x = room_x + (room_width - door_width) / 2.0;
            let door_y = room_y;

            walls.push(Rect::new(
                room_x + wall_thickness,
                room_y,
                door_x - (room_x + wall_thickness),
                wall_thickness,
            ));
            walls.push(Rect::new(
                door_x + door_width,
                room_y,
                (room_x + room_width - wall_thickness) - (door_x + door_width),
                wall_thickness,
            ));

            doors.push(Door {
                choice_index: 0,
                label: "SAÍDA / VITÓRIA".to_string(),
                rect: Rect::new(door_x, door_y, door_width, door_height),
                is_exit: true,
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
        let room = Room::build(&grammar, &state, 400.0);

        assert_eq!(room.doors.len(), 2);
        assert_eq!(room.doors[0].choice_index, 0);
        assert_eq!(room.doors[0].label, "S → aS");
        assert_eq!(room.doors[1].choice_index, 1);
        assert_eq!(room.doors[1].label, "S → ab");
        assert!(!room.doors[0].is_exit);
        assert!(!room.doors[1].is_exit);
    }

    #[test]
    fn room_generates_exit_door_when_stack_is_empty() {
        let grammar = parse_grammar("S -> a").unwrap();
        let mut state = DerivationState::new(&grammar);
        state.apply_choice(&grammar, 0).unwrap();
        assert!(state.is_complete());

        let room = Room::build(&grammar, &state, 400.0);
        assert_eq!(room.doors.len(), 1);
        assert!(room.doors[0].is_exit);
        assert_eq!(room.doors[0].label, "SAÍDA / VITÓRIA");
    }

    #[test]
    fn collision_detects_player_at_door() {
        let grammar = parse_grammar("S -> a").unwrap();
        let state = DerivationState::new(&grammar);
        let room = Room::build(&grammar, &state, 400.0);

        let door_rect = room.doors[0].rect;
        let player_at_door = Rect::new(door_rect.x + 10.0, door_rect.y + 10.0, 32.0, 32.0);
        let player_far_away = Rect::new(room.spawn_pos.x, room.spawn_pos.y, 32.0, 32.0);

        assert!(room.check_door_collision(player_at_door).is_some());
        assert!(room.check_door_collision(player_far_away).is_none());
    }
}
