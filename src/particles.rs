use crate::math::Vector2;
use macroquad::prelude::*;

pub struct Particles {
    particles: Vec<Particle>,
}

impl Particles {
    pub fn new() -> Self {
        Particles {
            particles: Vec::new(),
        }
    }

    pub fn explosion(&mut self, position: &Vector2, power: u32) {
        for _ in 0..power {
            self.particles.push(Particle::new(
                Vector2 {
                    x: position.x,
                    y: position.y,
                },
                power,
            ));
        }
    }

    pub fn update(&mut self) {
        let mut remove_index = None;
        let mut index = 0;
        for particle in &mut self.particles {
            particle.update();
            if particle.remove {
                remove_index = Some(index);
            }
            index += 1;
        }
        if let Some(index) = remove_index {
            self.particles.remove(index);
        }
    }
}

struct Particle {
    position: Vector2,
    velocity: Vector2,
    alpha: f32,
    color: Color,
    remove: bool,
}

impl Particle {
    pub fn new(position: Vector2, power: u32) -> Self {
        let green = rand::gen_range(0.7, 0.99);
        let min = -3.0 * power as f32;
        let max = 3.0 * power as f32;
        Particle {
            position,
            velocity: Vector2 {
                x: rand::gen_range(min, max),
                y: rand::gen_range(min, max),
            },
            alpha: rand::gen_range(0.6, 1.4),
            color: Color::new(rand::gen_range(green, 1.0), green, 0.0, 1.0),
            remove: false,
        }
    }

    fn update(&mut self) {
        self.draw();
        let delta = Vector2 {
            x: self.velocity.x * get_frame_time(),
            y: self.velocity.y * get_frame_time(),
        };
        self.position.x += delta.x;
        self.position.y += delta.y;
        let alpha = self.alpha - get_frame_time();
        if alpha <= 0.0 {
            self.remove = true;
            self.alpha = 0.0;
        } else {
            self.alpha = alpha;
        }
    }

    fn draw(&self) {
        draw_circle(
            self.position.x,
            self.position.y,
            8.0,
            self.color.with_alpha(self.alpha),
        );
    }
}
