use cgmath::Vector2;
use find_folder::Search;
use piston_window::*;
use sprite::*;
use std::collections::HashMap;
use std::rc::Rc;
use serde_json::{from_reader, Value, Map};
use std::fs::File;

mod player;
use player::*;

mod libs {
    pub mod controller;
    pub mod object;
    pub mod physics;
    pub mod transform;
}

use libs::controller::Controller;
use libs::object::Object;

pub struct SpriteManager<I: ImageSize> {
    sprites: HashMap<String, Vec<Sprite<I>>>,
}

impl<I> SpriteManager<I>
where
    I: ImageSize,
{
    fn new() -> Self {
        Self {
            sprites: HashMap::default(),
        }
    }

    fn load(&mut self, name: &'static str, rc: &Rc<I>, rect: [f64; 4], scale: [f64; 2]) {
        let mut sprite: Sprite<I> = Sprite::from_texture_rect(rc.clone(), rect);
        sprite.set_scale(scale[0], scale[0]);
        sprite.set_position(rect[2] * scale[0] / 2.0, rect[3] * scale[1] / 2.0);

        if let Some(sprites) = self.sprites.get_mut(name) {
            sprites.push(sprite);
        } else {
            self.sprites.insert(name.to_owned(), vec![sprite]);
        }
    }

    fn loads(&mut self, name: &'static str, rc: &Rc<I>, rects: Vec<[f64; 4]>, scale: [f64; 2]) {
        for rect in rects {
            self.load(name, rc, rect, scale);
        }
    }

    #[allow(dead_code)]
    fn get(&self, name: &'static str, index: usize) -> Option<&Sprite<I>> {
        self.sprites.get(name).map(|x| &x[index])
    }

    fn get_mut(&mut self, name: &'static str, index: usize) -> Option<&mut Sprite<I>> {
        self.sprites.get_mut(name).map(|x| &mut x[index])
    }

    fn get_first(&self, name: &'static str) -> Option<&Sprite<I>> {
        self.sprites.get(name).map(|x| x.first().unwrap())
    }
}

fn main() {
    let window_size: Size = Size::from([640, 480]);

    let mut window: PistonWindow = WindowSettings::new("Super Goomba Bros", window_size)
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));
    window.set_lazy(false);
    window.set_max_fps(60);

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
        "mario_small_red",
        &player_rc,
        animations
        .to_vec(),
        [map_scale, map_scale],
    );

    let mut player = Player::new(player_sprites);
    player.set_scale(map_scale, map_scale);
    player.push_animation("idle", 0);
    player.push_animation("jump", 1);
    player.append_animation("walk", [0, 1].to_vec());
    let mut controller = Controller::new();

    let mut map_pos = Vector2::from([0.0, 0.0]);
    let mut objects = Vec::new();

    let json_objects:Value = from_reader(File::open(assets.join("map.json")).unwrap()).unwrap();
    let grounds: &Vec<Value> = json_objects.as_object().unwrap().get("ground").unwrap().as_array().unwrap();
    let pipes: &Vec<Value> = json_objects.as_object().unwrap().get("pipe").unwrap().as_array().unwrap();
    let bricks: &Vec<Value> = json_objects.as_object().unwrap().get("brick").unwrap().as_array().unwrap();
    let tile1: &Map<String, Value> = json_objects.as_object().unwrap().get("tile1").unwrap().as_object().unwrap();
    let tile2: &Map<String, Value> = json_objects.as_object().unwrap().get("tile2").unwrap().as_object().unwrap();

    for obj in grounds.iter()
        .chain(pipes.iter())
        .chain(bricks.iter()) {

        let mut ground = Object::new();
        ground.set_position(obj["x"].as_f64().unwrap() * map_scale, obj["y"].as_f64().unwrap() * map_scale);
        ground.set_size(obj["width"].as_f64().unwrap() * map_scale, obj["height"].as_f64().unwrap() * map_scale);
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

    for (i, obj) in tile1.get("positions").unwrap().as_array().unwrap().iter()
        .chain(tile2.get("positions").unwrap().as_array().unwrap().iter()).enumerate() {

        let s1 = tile1.get("sprite").unwrap().as_object().unwrap();
        let s2 = tile2.get("sprite").unwrap().as_object().unwrap();

        let mut ground = Object::new();
        ground.set_scale(map_scale, map_scale);
        ground.set_sprite(
        if i < tile1.get("positions").unwrap().as_array().unwrap().len() { 
            Sprite::from_texture_rect(tile_rc.clone(), [
                s1.get("x").unwrap().as_f64().unwrap(), 
                s1.get("y").unwrap().as_f64().unwrap(), 
                s1.get("width").unwrap().as_f64().unwrap(), 
                s1.get("height").unwrap().as_f64().unwrap()
            ])
        } else { 
            Sprite::from_texture_rect(tile_rc.clone(), [
                s2.get("x").unwrap().as_f64().unwrap(), 
                s2.get("y").unwrap().as_f64().unwrap(), 
                s2.get("width").unwrap().as_f64().unwrap(), 
                s2.get("height").unwrap().as_f64().unwrap()
            ])
        });
        ground.set_position(obj["x"].as_f64().unwrap() * map_scale, obj["y"].as_f64().unwrap() * map_scale);
        ground.set_size(
            if i < tile1.get("positions").unwrap().as_array().unwrap().len() { 
                s1.get("width").unwrap().as_f64().unwrap()
            } else { 
                s2.get("width").unwrap().as_f64().unwrap()
            } * map_scale, 
            if i < tile1.get("positions").unwrap().as_array().unwrap().len() { 
                s1.get("height").unwrap().as_f64().unwrap()
            } else {
                s2.get("height").unwrap().as_f64().unwrap()
            } * map_scale);
        objects.push(ground);

    }

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _d| {
            clear([0.0, 0.0, 0.0, 0.5], g);
            sm.get_first("map")
                .unwrap()
                .draw(c.transform.trans(-map_pos.x, -map_pos.y), g);
            for object in objects.iter() {
                let obj = object.get_transform();
                if obj.x() < window_size.width && obj.right() >= 0.0 {
                    object.draw(c.transform, g);
                }
            }
            player.draw(c.transform, g);
        });

        if let Some(u) = e.update_args() {
            if !player.is_less_center(window.size())
                && player.dir_right()
                && map_pos.x < map_width * map_scale - window_size.width
            {
                map_pos.x += player.get_vel_x();
                for object in objects.iter_mut() {
                    object.trans(-player.get_vel_x(), 0.0);
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

            controller.update(&mut player, u.dt * 100.0);
            player.update(u.dt * 100.0);
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

        e.mouse_cursor(|_| {
            // ground.set_position(pos[0], pos[1]);
        });
    }
}
