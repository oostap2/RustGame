use crate::math::{Vector2, check_circle_point_collision};
use crate::particles::Particles;
use crate::streak::Streak;
use macroquad::prelude::*;

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

struct Enemy {
    position: Vector2,
    speed: f32,
    size: f32,
    direction: Direction,
    hover: bool,
}

impl Enemy {
    fn new(position: Vector2, speed: f32, size: f32, direction: Direction) -> Self {
        Self {
            position,
            speed,
            size,
            direction,
            hover: false,
        }
    }

    fn update(&mut self) {
        let delta = self.speed * get_frame_time();
        match self.direction {
            Direction::Left => self.position.x -= delta,
            Direction::Right => self.position.x += delta,
            Direction::Up => self.position.y -= delta,
            Direction::Down => self.position.y += delta,
        }
        let mouse_vector = {
            let mouse_position = mouse_position();
            Vector2 {
                x: mouse_position.0,
                y: mouse_position.1,
            }
        };
        self.hover = check_circle_point_collision(&self.position, &self.size, &mouse_vector);
    }

    fn draw(&self) {
        let color = match self.hover {
            false => RED,
            true => RED.with_alpha(0.5),
        };
        draw_circle(self.position.x, self.position.y, self.size, color);
    }
}

pub struct Enemies {
    enemies: Vec<Enemy>,
    new_count: f32,
    particles: Particles,
    red_alpha: f32,
}

impl Enemies {
    pub fn new() -> Self {
        let enemies = Vec::new();
        Enemies {
            enemies,
            new_count: 3.0,
            particles: Particles::new(),
            red_alpha: 0.0,
        }
    }

    pub fn update(&mut self, streak: &mut Streak) {
        self.particles.update();

        for i in (0..self.enemies.len()).rev() {
            let enemy = &mut self.enemies[i];
            enemy.update();
            enemy.draw();

            if enemy.position.x < -100.0
                || enemy.position.y < -100.0
                || 100.0 + screen_width() < enemy.position.x
                || 100.0 + screen_height() < enemy.position.y
            {
                self.enemies.clear();
                streak.reset();
                self.red_alpha = 1.0;
                break;
            }
        }

        self.new_count -= get_frame_time();
        if self.new_count <= 0.0 && (self.enemies.len() == 0 || streak.get_current_streak() != 0) {
            self.new_count = 1.0;
            self.new_enemy();
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            match self.click() {
                true => streak.increment(),
                false => {
                    streak.reset();
                    self.enemies.clear();
                    self.red_alpha = 1.0;
                }
            }
        }

        if self.red_alpha != 0.0 {
            let reduction = self.red_alpha - get_frame_time();
            if reduction < 0.0 {
                self.red_alpha = 0.0;
                return;
            }
            draw_rectangle(
                0.0,
                0.0,
                screen_width(),
                screen_height(),
                RED.with_alpha(self.red_alpha),
            );
            self.red_alpha = reduction;
        }
    }

    fn click(&mut self) -> bool {
        for i in (0..self.enemies.len()).rev() {
            let enemy = &mut self.enemies[i];
            if enemy.hover {
                self.particles.explosion(&enemy.position, enemy.size as u32);
                self.enemies.remove(i);
                return true;
            }
        }
        false
    }

    fn new_enemy(&mut self) {
        let direction = match rand::gen_range(0, 4) {
            1 => Direction::Right,
            2 => Direction::Up,
            3 => Direction::Down,
            _ => Direction::Left,
        };
        self.enemies.push(Enemy::new(
            match &direction {
                Direction::Down => Vector2 {
                    x: rand::gen_range(0.0, screen_width()),
                    y: -50.0,
                },
                Direction::Left => Vector2 {
                    x: screen_width() + 50.0,
                    y: rand::gen_range(0.0, screen_height()),
                },
                Direction::Right => Vector2 {
                    x: -50.0,
                    y: rand::gen_range(0.0, screen_height()),
                },
                Direction::Up => Vector2 {
                    x: rand::gen_range(0.0, screen_width()),
                    y: screen_height() + 50.0,
                },
            },
            rand::gen_range(100.0, 300.0),
            rand::gen_range(20.0, 35.0),
            direction,
        ));
    }
}
