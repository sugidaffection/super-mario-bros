use cgmath::Vector2;
use find_folder::Search;
use fps_counter::FPSCounter;
use graphics::Transformed;
use piston_window::{
    clear, Button, ButtonArgs, ButtonEvent, EventLoop, Filter, Flip, G2dTexture, G2dTextureContext,
    GenericEvent, ImageSize, PistonWindow, RenderEvent, Size, Texture, TextureSettings,
    UpdateEvent, Window, WindowSettings,
};
use serde_json::{from_reader, Value};
use sprite::Sprite;
use std::cell::RefCell;
use std::fs::File;
use std::path;
use std::rc::Rc;

mod libs {
    pub mod animations;
    pub mod camera;
    pub mod collider;
    pub mod controller;
    pub mod object;
    pub mod physics;
    pub mod player;
    pub mod sprites_manager;
    pub mod spritesheet;
    pub mod transform;
}

use libs::camera::Camera;
use libs::object::{Object, Object2D};
use libs::player::Player;
use libs::spritesheet::{SpriteSheet, SpriteSheetConfig};
use libs::transform::{Rect, Trans};

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum Music {
    World1_1,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum Sound {
    Jump,
}

pub struct Game {
    window: PistonWindow,
    size: Size,
    camera: Camera,
    player: Player<G2dTexture>,
    player2: Player<G2dTexture>,
    tilemap: SpriteSheet<G2dTexture>,
    objects: Vec<Object<G2dTexture>>,
}
impl Game {
    pub fn new(size: Size, viewport_size: Size) -> Self {
        let mut window: PistonWindow = WindowSettings::new("Super Goomba Bros", size)
            .exit_on_esc(true)
            .build()
            .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));
        window.set_ups(60);

        let mut context = window.create_texture_context();

        let map_texture = Self::load_texture(&mut context, "world_1_1.png");
        let map_size = map_texture.get_size();
        let tilemap = SpriteSheet::new(map_texture);

        let player_texture = Self::load_texture(&mut context, "player.png");
        let mut player = Self::load_player(Rc::clone(&player_texture));
        Self::load_mario(&mut player);

        let mut player2 = Self::load_player(Rc::clone(&player_texture));
        Self::load_luigi(&mut player2);

        let camera = Camera::new(
            viewport_size.width,
            viewport_size.height,
            map_size.0.into(),
            map_size.1 as f64,
            window.draw_size().height as f64 / map_size.1 as f64,
        );

        let tileset_texture = Self::load_texture(&mut context, "tileset1.png");
        let objects = Self::load_objects(tileset_texture);

        let game = Self {
            window,
            size,
            camera,
            player,
            player2,
            tilemap,
            objects,
        };

        game
    }

    fn load_texture(context: &mut G2dTextureContext, path: &'static str) -> Rc<G2dTexture> {
        let assets = Search::Parents(1).for_folder("assets").unwrap();
        let path = assets.join(path);

        let mut texture_settings = TextureSettings::new();
        texture_settings.set_mag(Filter::Nearest);
        let texture = Texture::from_path(context, path, Flip::None, &texture_settings).unwrap();
        Rc::new(texture)
    }

    fn load_player(texture: Rc<G2dTexture>) -> Player<G2dTexture> {
        let player_sprite_sheet = SpriteSheet::new(texture);
        let player_config_default = SpriteSheetConfig {
            offset: Vector2::from([80.0, 34.0]),
            spacing: Vector2::from([1.0, 47.0]),
            grid: [21, 11],
            sprite_size: Size::from([16.0, 16.0]),
        };

        let mut player = Player::new();
        player.set_sprite_sheet(player_sprite_sheet, player_config_default);

        player
    }

    fn load_mario(player: &mut Player<G2dTexture>) {
        player.add_animation("idle", vec![[0, 0]]);
        player.add_animation("jump", vec![[0, 5]]);
        player.add_animation("walk", vec![[0, 1], [0, 2], [0, 3]]);
        player.add_animation("skid", vec![[0, 4]]);
    }

    fn load_luigi(player: &mut Player<G2dTexture>) {
        player.add_animation("idle", vec![[1, 0]]);
        player.add_animation("jump", vec![[1, 5]]);
        player.add_animation("walk", vec![[1, 1], [1, 2], [1, 3]]);
    }

    fn load_objects(texture: Rc<G2dTexture>) -> Vec<Object<G2dTexture>> {
        let mut tileset = SpriteSheet::new(texture.clone());
        let tileset_config = SpriteSheetConfig {
            offset: Vector2::from([0.0, 0.0]),
            spacing: Vector2::from([0.0, 0.0]),
            grid: [1, 2],
            sprite_size: Size::from([16.0, 16.0]),
        };
        tileset.set_config(&tileset_config);
        tileset.set_current_tiles(0, 0);
        let brick_sprite = Rc::new(RefCell::new(tileset.clone_sprite()));
        tileset.set_current_tiles(0, 1);
        let coin_sprite = Rc::new(RefCell::new(tileset.clone_sprite()));

        let assets = Search::Parents(1).for_folder("assets").unwrap();
        let path = assets.join("world_1_1.tmj");
        let file = File::open(path).unwrap();
        let json_objects: Value = from_reader(file).unwrap();
        let layers: &Vec<Value> = json_objects
            .as_object()
            .unwrap()
            .get("layers")
            .unwrap()
            .as_array()
            .unwrap();

        let solid_objects: Vec<Value> = layers
            .iter()
            .flat_map(|x| {
                if ["ground", "solid_objects", "bricks", "pipes"]
                    .contains(&x.get("name")?.as_str()?)
                {
                    x.get("objects")
                        .and_then(|objects| objects.as_array())
                        .map(|x| x.to_vec())
                } else {
                    None
                }
            })
            .flatten()
            .collect();

        let mut objects: Vec<Object<G2dTexture>> = Vec::default();
        for obj in solid_objects {
            let json_obj = obj.as_object().unwrap();
            let x = json_obj.get("x").unwrap().as_f64().unwrap();
            let y = json_obj.get("y").unwrap().as_f64().unwrap();
            let w = json_obj.get("width").unwrap().as_f64().unwrap();
            let h = json_obj.get("height").unwrap().as_f64().unwrap();
            let name = json_obj.get("name").unwrap().as_str().unwrap().to_string();
            let mut o = Object::<G2dTexture>::new(name.clone());
            o.set_size(w, h);
            o.set_position(x, y);
            if name.to_string().trim() == "brick".to_string().trim() {
                o.set_sprite(Rc::clone(&brick_sprite));
            }

            if name.to_string().trim() == "coin".to_string().trim() {
                o.set_sprite(Rc::clone(&coin_sprite));
            }
            objects.push(o);
        }

        objects
    }

    pub fn render<E>(&mut self, e: &E)
    where
        E: GenericEvent,
    {
        let window_size = self.size;
        let camera = &mut self.camera;
        let camera_width = camera.viewport_width;
        let camera_height = camera.viewport_height;
        let scaled_width = camera_width * camera.scale;
        let scaled_height = camera_height * camera.scale;
        let translate_x = (window_size.width - scaled_width) / (2.0 * camera.scale);
        let translate_y = (window_size.height - scaled_height) / (2.0 * camera.scale);

        let player = &mut self.player;
        let player2 = &mut self.player2;
        let objects = &mut self.objects;

        let tilemap = &mut self.tilemap;

        tilemap.set_src_rect([camera.position.x.max(0.0), 0.0, camera_width, camera_height]);

        self.window.draw_2d(e, |_, g, _d| {
            clear([0.0, 0.0, 0.0, 0.5], g);
        });

        // render map
        self.window.draw_2d(e, |c, g, _d| {
            let map_transform = c
                .scale(camera.scale, camera.scale)
                .trans(translate_x, translate_y)
                .transform;
            tilemap.draw(map_transform, g);
        });

        // render player and objects
        self.window.draw_2d(e, |c, g, _d| {
            let transform = c
                .scale(camera.scale, camera.scale)
                .trans(
                    -camera.position.x + translate_x,
                    -camera.position.y + translate_y,
                )
                .transform;
            for object in objects.iter_mut() {
                let obj = object.get_transform();
                if obj.x() < camera.position.x + camera.viewport_width
                    && obj.xw() >= camera.position.x
                {
                    object.draw(transform, g);
                }
            }

            player.draw(transform, g);
            if player2.get_transform().x() < camera.position.x + camera.viewport_width
                && player2.get_transform().xw() >= camera.position.x
            {
                player2.draw(transform, g);
            }
        });
    }

    pub fn update(&mut self, dt: f64) {
        self.player.update(dt);
        // self.player2.update(dt);

        for object in self.objects.iter() {
            self.player.collide_with(&object.get_transform());
            self.player2.collide_with(&object.get_transform());
        }

        self.camera.follow_player(&self.player);
    }

    pub fn update_input(&mut self, args: ButtonArgs) {
        if let Button::Keyboard(key) = args.button {
            self.player.update_input(key, args.state);
        }
    }
}

fn main() {
    let scale = 3.0;
    let width = 352.0;
    let height = 224.0;
    let window_size: Size = Size::from([width * scale, height * scale]);
    let viewport_size: Size = Size::from([width, height]);
    let mut timer = FPSCounter::default();

    let mut game: Game = Game::new(window_size, viewport_size);

    music::start::<Music, Sound, _>(16, || {
        music::bind_music_file(Music::World1_1, "./assets/main_theme.mp3");
        music::bind_sound_file(Sound::Jump, "./assets/jump.mp3");
        music::set_volume(music::MAX_VOLUME);
        music::play_music(&Music::World1_1, music::Repeat::Forever);

        while let Some(e) = game.window.next() {
            if let Some(_) = e.render_args() {
                game.render(&e);
            }

            if let Some(u) = e.update_args() {
                game.update(u.dt);
            }

            if let Some(args) = e.button_args() {
                game.update_input(args);
            }
        }
    });
}
