use crate::math::Vector2;
use macroquad::prelude::*;

pub struct Background {
    elements: Vec<BackElement>,
    theme: Theme,
}

impl Background {
    pub fn new(count: u16) -> Self {
        let mut elements = Vec::new();

        for _ in 0..count {
            elements.push(BackElement::new());
        }
        Self {
            elements,
            theme: Theme::White,
        }
    }

    pub fn draw(&mut self) {
        match self.theme {
            Theme::Black => clear_background(BLACK),
            Theme::White => clear_background(WHITE),
        }
        for element in &mut self.elements {
            element.update();
        }
        if is_key_pressed(KeyCode::T) {
            self.change_theme();
        }
    }

    pub fn get_theme(&self) -> &Theme {
        &self.theme
    }

    fn change_theme(&mut self) {
        self.theme = match self.theme {
            Theme::Black => Theme::White,
            Theme::White => Theme::Black,
        };
    }
}

pub enum Theme {
    Black,
    White,
}

struct BackElement {
    alpha: f32,
    alpha_velocity: f32,
    position: Vector2,
    velocity: Vector2,
}

impl BackElement {
    fn new() -> Self {
        Self {
            alpha: rand::gen_range(0.0, 1.0),
            alpha_velocity: 0.0,
            position: Vector2 { x: -99.9, y: -99.9 },
            velocity: Vector2 {
                x: rand::gen_range(-10.0, 10.0),
                y: rand::gen_range(-10.0, 10.0),
            },
        }
    }

    fn update(&mut self) {
        self.alpha_velocity -= 0.0003 * get_frame_time();
        self.alpha += self.alpha_velocity;
        if self.alpha < 0.0 {
            self.respawn();
        }
        self.position.x += self.velocity.x * get_frame_time();
        self.position.y += self.velocity.y * get_frame_time();
        self.draw();
    }

    fn respawn(&mut self) {
        self.alpha = 0.0;
        self.alpha_velocity = 0.001;
        self.position.x = rand::gen_range(0.0, screen_width());
        self.position.y = rand::gen_range(0.0, screen_height());
    }

    fn draw(&self) {
        draw_circle(
            self.position.x,
            self.position.y,
            15.0,
            GRAY.with_alpha(self.alpha),
        );
    }
}
