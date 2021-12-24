mod board;
mod game;
mod renderer;

use crate::game::Rustivanders;
use std::io::stdout;

fn main() {
    Rustivanders::new(stdout()).run();
}
