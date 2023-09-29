use libs::{game::Game, prelude::GameBuilder};
use piston_window::Size;

mod libs {
    pub mod core;
    pub mod entities;
    pub mod game;
    pub mod prelude;
    pub mod ui;
    pub mod utils;
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum Music {
    World1_1,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum Sound {
    Jump,
    Brick,
    Coin,
}

fn main() {
    let game = Game::new().build();
    match game {
        Err(error) => println!("{}", error),
        _ => {}
    }
}
