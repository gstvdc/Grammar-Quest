use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Down,
    Up,
    Left,
    Right,
}

pub struct Player {
    pub pos: Vec2,
    pub speed: f32,
    pub facing: Direction,
    pub frame: usize,
    pub anim_timer: f32,
    pub is_moving: bool,
    pub collision_size: Vec2,
}

impl Player {
    pub fn new(spawn: Vec2) -> Self {
        Self {
            pos: spawn,
            speed: 240.0,
            facing: Direction::Up,
            frame: 0,
            anim_timer: 0.0,
            is_moving: false,
            collision_size: Vec2::new(32.0, 28.0),
        }
    }

    pub fn bounding_rect(&self) -> Rect {
        Rect::new(
            self.pos.x - self.collision_size.x / 2.0,
            self.pos.y - self.collision_size.y / 2.0,
            self.collision_size.x,
            self.collision_size.y,
        )
    }

    pub fn update(&mut self, dt: f32, walls: &[Rect]) {
        let mut dir = Vec2::ZERO;

        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            dir.y -= 1.0;
            self.facing = Direction::Up;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            dir.y += 1.0;
            self.facing = Direction::Down;
        }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            dir.x -= 1.0;
            self.facing = Direction::Left;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            dir.x += 1.0;
            self.facing = Direction::Right;
        }

        self.is_moving = dir.length_squared() > 0.0;

        if self.is_moving {
            dir = dir.normalize();
            let move_delta = dir * self.speed * dt;

            // X-axis movement with wall collision
            let mut test_pos = self.pos;
            test_pos.x += move_delta.x;
            let test_rect_x = Rect::new(
                test_pos.x - self.collision_size.x / 2.0,
                self.pos.y - self.collision_size.y / 2.0,
                self.collision_size.x,
                self.collision_size.y,
            );
            if !walls.iter().any(|w| w.overlaps(&test_rect_x)) {
                self.pos.x = test_pos.x;
            }

            // Y-axis movement with wall collision
            let mut test_pos_y = self.pos;
            test_pos_y.y += move_delta.y;
            let test_rect_y = Rect::new(
                self.pos.x - self.collision_size.x / 2.0,
                test_pos_y.y - self.collision_size.y / 2.0,
                self.collision_size.x,
                self.collision_size.y,
            );
            if !walls.iter().any(|w| w.overlaps(&test_rect_y)) {
                self.pos.y = test_pos_y.y;
            }

            self.anim_timer += dt;
            if self.anim_timer >= 0.12 {
                self.anim_timer = 0.0;
                self.frame = (self.frame + 1) % 6;
            }
        } else {
            self.frame = 0;
            self.anim_timer = 0.0;
        }
    }

    pub fn draw(&self, texture: Option<&Texture2D>) {
        // Subtle drop shadow
        draw_ellipse(
            self.pos.x,
            self.pos.y + 18.0,
            18.0,
            8.0,
            0.0,
            Color::from_rgba(0, 0, 0, 100),
        );

        if let Some(tex) = texture {
            let row = match self.facing {
                Direction::Down => 0,
                Direction::Right | Direction::Left => 1,
                Direction::Up => 3,
            };

            let frame_w = 64.0;
            let frame_h = 64.0;
            let sx = self.frame as f32 * frame_w;
            let sy = row as f32 * frame_h;

            let flip_x = self.facing == Direction::Left;

            draw_texture_ex(
                tex,
                self.pos.x - frame_w / 2.0,
                self.pos.y - frame_h / 2.0,
                WHITE,
                DrawTextureParams {
                    source: Some(Rect::new(sx, sy, frame_w, frame_h)),
                    flip_x,
                    ..Default::default()
                },
            );
        } else {
            // Neon fallback avatar
            draw_circle(
                self.pos.x,
                self.pos.y,
                20.0,
                Color::from_rgba(74, 229, 255, 255),
            );
            draw_circle(
                self.pos.x,
                self.pos.y,
                14.0,
                Color::from_rgba(25, 19, 45, 255),
            );
            let visor_offset = match self.facing {
                Direction::Up => vec2(0.0, -8.0),
                Direction::Down => vec2(0.0, 8.0),
                Direction::Left => vec2(-8.0, 0.0),
                Direction::Right => vec2(8.0, 0.0),
            };
            draw_circle(
                self.pos.x + visor_offset.x,
                self.pos.y + visor_offset.y,
                4.0,
                Color::from_rgba(238, 166, 255, 255),
            );
        }
    }
}
