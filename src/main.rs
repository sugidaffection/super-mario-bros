use cgmath::{InnerSpace, Vector2};
use find_folder::Search;
use piston_window::*;
use serde_json::{from_reader, Map, Value};
use sprite::*;
use std::fs::File;
use std::rc::Rc;

mod player;
use player::*;

mod libs {
    pub mod collider;
    pub mod controller;
    pub mod object;
    pub mod physics;
    pub mod sprites_manager;
    pub mod transform;
}

use libs::controller::Controller;
use libs::object::{Object, Object2D};
use libs::sprites_manager::SpriteManager;
use libs::transform::{Rect, Trans};

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum Music {
    WORLD_1_1,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum Sound {
    Ding,
}

fn main() {
    let window_size: Size = Size::from([640, 480]);

    let mut window: PistonWindow = WindowSettings::new("Super Goomba Bros", window_size)
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));
    window.set_lazy(false);
    window.set_max_fps(30);

    let assets = Search::Parents(1).for_folder("assets").unwrap();

    let mut sm = SpriteManager::new();

    let map_texture = Texture::from_path(
        &mut window.create_texture_context(),
        &assets.join("world_1_1.png"),
        Flip::None,
        &TextureSettings::new(),
    )
    .unwrap();

    let map_width = map_texture.get_width() as f64;
    let map_height = map_texture.get_height() as f64;
    let map_scale = window_size.height as f64 / map_height;
    let map_rc = Rc::new(map_texture);
    sm.load(
        "map",
        &map_rc,
        [0.0, 0.0, map_width, map_height],
        [map_scale, map_scale],
    );

    let player_texture = Texture::from_path(
        &mut window.create_texture_context(),
        &assets.join("enemies.png"),
        Flip::None,
        &TextureSettings::new(),
    )
    .unwrap();

    let player_rc = Rc::new(player_texture);
    let mut player_sprites = SpriteManager::new();
    // let animations = [
    //     [80.0, 34.0, 16.0, 16.0],
    //     [97.0, 34.0, 16.0, 16.0],
    //     [114.0, 34.0, 16.0, 16.0],
    //     [131.0, 34.0, 16.0, 16.0],
    //     [148.0, 34.0, 16.0, 16.0],
    //     [165.0, 34.0, 16.0, 16.0],
    //     [182.0, 34.0, 16.0, 16.0],
    //     [199.0, 34.0, 16.0, 16.0],
    //     [216.0, 34.0, 16.0, 16.0],
    //     [233.0, 34.0, 16.0, 16.0],
    //     [250.0, 34.0, 16.0, 16.0],
    //     [267.0, 34.0, 16.0, 16.0],
    //     [284.0, 34.0, 16.0, 16.0],
    //     [301.0, 34.0, 16.0, 16.0],
    // ];
    let animations = [
        [0.0, 16.0, 16.0, 16.0],
        [16.0, 16.0, 16.0, 16.0],
        [32.0, 16.0, 16.0, 16.0],
    ];
    player_sprites.loads(
        "default",
        &player_rc,
        animations.to_vec(),
        [map_scale, map_scale],
    );

    let mut player = Player::new();
    player.set_sprites(player_sprites);
    player.set_scale(map_scale, map_scale);
    player.push_animation("idle", 0);
    player.push_animation("jump", 1);
    player.append_animation("walk", [0, 1].to_vec());
    let mut controller = Controller::new();

    let mut map_pos = Vector2::from([0.0, 0.0]);
    let mut objects = Vec::new();

    let json_objects: Value = from_reader(File::open(assets.join("map.json")).unwrap()).unwrap();
    let grounds: &Vec<Value> = json_objects
        .as_object()
        .unwrap()
        .get("ground")
        .unwrap()
        .as_array()
        .unwrap();
    let pipes: &Vec<Value> = json_objects
        .as_object()
        .unwrap()
        .get("pipe")
        .unwrap()
        .as_array()
        .unwrap();
    let bricks: &Vec<Value> = json_objects
        .as_object()
        .unwrap()
        .get("brick")
        .unwrap()
        .as_array()
        .unwrap();
    let tile1: &Map<String, Value> = json_objects
        .as_object()
        .unwrap()
        .get("tile1")
        .unwrap()
        .as_object()
        .unwrap();
    let tile2: &Map<String, Value> = json_objects
        .as_object()
        .unwrap()
        .get("tile2")
        .unwrap()
        .as_object()
        .unwrap();

    for obj in grounds.iter().chain(pipes.iter()).chain(bricks.iter()) {
        let mut ground = Object::new();
        ground.set_scale(map_scale, map_scale);
        ground.set_position(
            obj["x"].as_f64().unwrap() * map_scale,
            obj["y"].as_f64().unwrap() * map_scale,
        );
        ground.set_size(
            obj["width"].as_f64().unwrap(),
            obj["height"].as_f64().unwrap(),
        );
        objects.push(ground);
    }

    let tile_texture = Texture::from_path(
        &mut window.create_texture_context(),
        &assets.join("tileset.png"),
        Flip::None,
        &TextureSettings::new(),
    )
    .unwrap();

    let tile_rc = Rc::new(tile_texture);

    for (i, obj) in tile1
        .get("positions")
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .chain(tile2.get("positions").unwrap().as_array().unwrap().iter())
        .enumerate()
    {
        let s1 = tile1.get("sprite").unwrap().as_object().unwrap();
        let s2 = tile2.get("sprite").unwrap().as_object().unwrap();
        let mut sprite = Sprite::from_texture_rect(
            tile_rc.clone(),
            [
                s1.get("x").unwrap().as_f64().unwrap(),
                s1.get("y").unwrap().as_f64().unwrap(),
                s1.get("width").unwrap().as_f64().unwrap(),
                s1.get("height").unwrap().as_f64().unwrap(),
            ],
        );

        if i >= tile1.get("positions").unwrap().as_array().unwrap().len() {
            sprite = Sprite::from_texture_rect(
                tile_rc.clone(),
                [
                    s2.get("x").unwrap().as_f64().unwrap(),
                    s2.get("y").unwrap().as_f64().unwrap(),
                    s2.get("width").unwrap().as_f64().unwrap(),
                    s2.get("height").unwrap().as_f64().unwrap(),
                ],
            )
        };

        let mut ground = Object::new();
        ground.set_scale(map_scale, map_scale);
        ground.set_sprite(sprite);
        ground.set_position(
            obj["x"].as_f64().unwrap() * map_scale,
            obj["y"].as_f64().unwrap() * map_scale,
        );
        ground.set_size(
            if i < tile1.get("positions").unwrap().as_array().unwrap().len() {
                s1.get("width").unwrap().as_f64().unwrap()
            } else {
                s2.get("width").unwrap().as_f64().unwrap()
            },
            if i < tile1.get("positions").unwrap().as_array().unwrap().len() {
                s1.get("height").unwrap().as_f64().unwrap()
            } else {
                s2.get("height").unwrap().as_f64().unwrap()
            },
        );
        objects.push(ground);
    }

    music::start::<Music, Sound, _>(16, || {
        music::bind_music_file(Music::WORLD_1_1, "./assets/main_theme.mp3");
        music::set_volume(music::MAX_VOLUME);
        music::play_music(&Music::WORLD_1_1, music::Repeat::Forever);

        let mut mouse_pos = [0.0, 0.0];

        while let Some(e) = window.next() {
            window.draw_2d(&e, |c, g, _d| {
                clear([0.0, 0.0, 0.0, 0.5], g);
                sm.get_first("map")
                    .unwrap()
                    .draw(c.transform.trans(-map_pos.x, -map_pos.y), g);
                for object in objects.iter_mut() {
                    let obj = object.get_transform();
                    if obj.x() < window_size.width && obj.xw() >= 0.0 {
                        object.draw(c.transform, g);
                    }
                }
                player.draw(c.transform, g);
            });

            if let Some(pos) = e.mouse_cursor_args() {
                mouse_pos = pos;
            }

            if let Some(u) = e.update_args() {
                player.limit_move_size(window_size);
                if !player.is_can_move()
                    && player.dir_right()
                    && map_pos.x < map_width * map_scale - window_size.width
                {
                    map_pos.x += player.get_vel_x();
                    for object in objects.iter_mut() {
                        object.translate(-player.get_vel_x(), 0.0);
                    }
                } else {
                    player.update_position_x(u.dt * 100.0);
                }

                if map_pos.x < 0.0 {
                    map_pos.x = 0.0;
                }

                if map_pos.x > map_width * map_scale - window_size.width {
                    map_pos.x = map_width * map_scale - window_size.width;
                }

                controller.update(&mut player);
                player.update(u.dt * 100.0);
                // player.set_position(mouse_pos[0], mouse_pos[1]);

                for object in objects.iter() {
                    player.collide_with(object);
                }

                player.set_inside_window(window_size);
            }

            if let Some(args) = e.button_args() {
                if let Button::Keyboard(key) = args.button {
                    controller.keyboard_event(key, args.state);
                }
            }
        }
    });
}
