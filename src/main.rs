use libs::game::Game;
use piston_window::Size;

mod libs {
    pub mod animations;
    pub mod bricks;
    pub mod camera;
    pub mod collider;
    pub mod controller;
    pub mod core;
    pub mod enemies;
    pub mod game;
    pub mod object;
    pub mod physics;
    pub mod player;
    pub mod progress_bar;
    pub mod sprite_sheet;
    pub mod sprite_sheet_manager;
    pub mod stages;
    pub mod textures;
    pub mod transform;
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
    let scale = 3.0;
    let width = 352.0;
    let height = 224.0;
    let window_size: Size = Size::from([width * scale, height * scale]);
    let viewport_size: Size = Size::from([width, height]);

    match Game::new(window_size, viewport_size) {
        Ok(mut game) => {
            game.start();
        }
        Err(error) => {
            eprintln!("Error! {:?}", error);
        }
    }
}
