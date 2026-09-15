use macroquad::prelude::*;

pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub color: Color,
    pub life: f32,
    pub max_life: f32,
    pub size: f32,
}

pub struct Shockwave {
    pub pos: Vec2,
    pub radius: f32,
    pub max_radius: f32,
    pub color: Color,
}

pub struct FloatingText {
    pub pos: Vec2,
    pub text: String,
    pub color: Color,
    pub life: f32,
    pub max_life: f32,
}

/// Bundles the three transient visual-feedback collections so gameplay and
/// render code pass one handle instead of three parallel `Vec`s.
#[derive(Default)]
pub struct EffectsState {
    pub particles: Vec<Particle>,
    pub shockwaves: Vec<Shockwave>,
    pub floating_texts: Vec<FloatingText>,
}

impl EffectsState {
    pub fn clear(&mut self) {
        self.particles.clear();
        self.shockwaves.clear();
        self.floating_texts.clear();
    }

    pub fn update(&mut self, dt: f32) {
        self.particles.retain_mut(|p| {
            p.pos += p.vel * dt;
            p.life -= dt;
            p.life > 0.0
        });

        self.shockwaves.retain_mut(|s| {
            s.radius += 380.0 * dt;
            s.radius < s.max_radius
        });

        self.floating_texts.retain_mut(|ft| {
            ft.pos.y -= 32.0 * dt;
            ft.life -= dt;
            ft.life > 0.0
        });
    }

    pub fn spawn_spark_burst(&mut self, center: Vec2, base_color: Color) {
        for _ in 0..32 {
            let angle = rand::gen_range(0.0, std::f32::consts::TAU);
            let speed = rand::gen_range(60.0, 260.0);
            let vel = vec2(angle.cos() * speed, angle.sin() * speed);
            let life = rand::gen_range(0.35, 0.75);
            self.particles.push(Particle {
                pos: center,
                vel,
                color: base_color,
                life,
                max_life: life,
                size: rand::gen_range(2.5, 6.0),
            });
        }
    }

    pub fn push_shockwave(&mut self, pos: Vec2, color: Color, max_radius: f32) {
        self.shockwaves.push(Shockwave {
            pos,
            radius: 10.0,
            max_radius,
            color,
        });
    }

    pub fn push_floating_text(&mut self, pos: Vec2, text: impl Into<String>, color: Color) {
        self.floating_texts.push(FloatingText {
            pos,
            text: text.into(),
            color,
            life: 1.4,
            max_life: 1.4,
        });
    }

    /// Drawn before the player, so sparks and shockwaves read as coming
    /// from behind/around the sprite.
    pub fn draw_particles_and_shockwaves(&self) {
        for s in &self.shockwaves {
            let alpha = 1.0 - (s.radius / s.max_radius);
            let mut c = s.color;
            c.a = alpha;
            draw_circle_lines(s.pos.x, s.pos.y, s.radius, 2.5, c);
        }

        for p in &self.particles {
            let alpha = p.life / p.max_life;
            let mut c = p.color;
            c.a = alpha;
            draw_circle(p.pos.x, p.pos.y, p.size * alpha, c);
        }
    }

    /// Drawn after the player, so feedback text stays legible on top.
    pub fn draw_floating_texts(&self) {
        for ft in &self.floating_texts {
            let alpha = ft.life / ft.max_life;
            let mut c = ft.color;
            c.a = alpha;
            let dim = measure_text(&ft.text, None, 22, 1.0);
            draw_text(&ft.text, ft.pos.x - dim.width / 2.0, ft.pos.y, 22.0, c);
        }
    }
}
