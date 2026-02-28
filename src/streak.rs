use crate::background::Theme;
use macroquad::prelude::*;

pub struct Streak {
    current_streak: u32,
    longest_streak: u32,
}

impl Streak {
    pub fn new() -> Self {
        Streak {
            current_streak: 0,
            longest_streak: 0,
        }
    }

    pub fn increment(&mut self) {
        self.current_streak += 1;
        if self.current_streak > self.longest_streak {
            self.longest_streak = self.current_streak;
        }
    }

    pub fn reset(&mut self) {
        self.current_streak = 0;
    }

    pub fn draw(&self, theme: &Theme) {
        let color = match theme {
            Theme::Black => WHITE,
            Theme::White => BLACK,
        };
        let size = 30.0;
        draw_text(
            &format!("Current Streak: {}", self.current_streak),
            10.0,
            30.0,
            size,
            color,
        );
        draw_text(
            &format!("Longest Streak: {}", self.longest_streak),
            10.0,
            30.0 + size,
            size,
            color,
        );
    }

    pub fn get_current_streak(&self) -> u32 {
        self.current_streak
    }
}
