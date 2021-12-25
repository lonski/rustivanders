mod board;
mod events;
mod game;
mod point;
mod renderer;
mod sprite;

use crate::game::Rustivanders;

fn main() {
    Rustivanders::new().run();
}
