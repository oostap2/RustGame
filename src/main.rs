use crate::background::Background;
use crate::enemies::Enemies;
use crate::streak::Streak;
use macroquad::prelude::*;

mod background;
mod enemies;
mod math;
mod particles;
mod streak;

#[macroquad::main("Fight Reds")]
async fn main() {
    let mut background = Background::new(30);
    let mut enemies = Enemies::new();
    let mut streak = Streak::new();

    loop {
        background.draw();
        enemies.update(&mut streak);
        streak.draw(background.get_theme());

        next_frame().await
    }
}
