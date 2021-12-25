mod board;
mod events;
mod game;
mod point;
mod renderer;
mod sprite;
mod ai;

use crate::game::Rustivanders;

fn main() {
    Rustivanders::new().run();
}
