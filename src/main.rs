mod board;
mod game;
mod renderer;
mod sprite;
mod point;

use crate::game::Rustivanders;
use std::io::stdout;

fn main() {
    Rustivanders::new(stdout()).run();
}
