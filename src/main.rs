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

    music::start::<Music, Sound, _>(16, || {
        music::bind_music_file(Music::World1_1, "./assets/sounds/main_theme.mp3");
        music::bind_sound_file(Sound::Jump, "./assets/sounds/jump.mp3");
        music::bind_sound_file(Sound::Brick, "./assets/sounds/brick.wav");
        music::bind_sound_file(Sound::Coin, "./assets/sounds/coin.wav");
        music::set_volume(music::MAX_VOLUME);
        music::play_music(&Music::World1_1, music::Repeat::Forever);
        match Game::new(window_size, viewport_size) {
            Ok(mut game) => {
                game.start();
            }
            Err(error) => {
                eprintln!("Error! {:?}", error);
            }
        }
    });
}
