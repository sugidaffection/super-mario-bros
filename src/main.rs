use cgmath::Vector2;
use find_folder::Search;
use fps_counter::FPSCounter;
use graphics::Transformed;
use piston_window::{
    clear, Button, ButtonArgs, ButtonEvent, EventLoop, G2dTexture, G2dTextureContext, GenericEvent,
    ImageSize, PistonWindow, RenderEvent, Size, UpdateEvent, WindowSettings,
};
use serde_json::{from_reader, Value};
use std::fs::File;
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
use libs::sprites_manager::SpriteManager;
use libs::spritesheet::{SpriteSheet, SpriteSheetConfig};
use libs::transform::{Rect, Trans};

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum Music {
    World1_1,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum Sound {}

pub struct Game {
    window: PistonWindow,
    size: Size,
    camera: Camera,
    player: Player<G2dTexture>,
    tilemap: SpriteSheet<G2dTexture>,
    objects: Vec<Object<G2dTexture>>,
}
impl Game {
    pub fn new(size: Size, viewport_size: Size) -> Self {
        let mut window: PistonWindow = WindowSettings::new("Super Goomba Bros", size)
            .exit_on_esc(true)
            .build()
            .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));
        window.set_ups(30);

        let mut context = window.create_texture_context();

        let map_texture = Self::load_texture(&mut context, "world_1.png");
        let map_size = map_texture.get_size();
        let tilemap = SpriteSheet::new(map_texture);

        let player = Self::load_player(&mut context);

        let camera = Camera::new(
            viewport_size.width,
            viewport_size.height,
            map_size.0.into(),
            map_size.1 as f64,
            3.0,
        );

        let objects = Self::load_objects();

        let game = Self {
            window,
            size,
            camera,
            player,
            tilemap,
            objects,
        };

        game
    }

    fn load_texture(context: &mut G2dTextureContext, path: &'static str) -> Rc<G2dTexture> {
        let assets = Search::Parents(1).for_folder("assets").unwrap();
        let path = assets.join(path);
        SpriteManager::<G2dTexture>::load_texture(context, &path)
    }

    fn load_player(context: &mut G2dTextureContext) -> Player<G2dTexture> {
        let player_texture = Self::load_texture(context, "player.png");
        let player_sprite_sheet = SpriteSheet::new(player_texture);
        let player_config_default = SpriteSheetConfig {
            offset: Vector2::from([80.0, 34.0]),
            spacing: Vector2::from([1.0, 47.0]),
            grid: [21, 11],
            sprite_size: Size::from([16.0, 16.0]),
        };
        let mut player = Player::new(player_sprite_sheet);
        player.add_config("default", player_config_default);
        player.set_current_config("default");

        player
    }

    fn load_objects() -> Vec<Object<G2dTexture>> {
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
        let solid_objects: &Vec<Value> = layers
            .iter()
            .find(|x| x.get("name").unwrap() == "solid_objects")
            .unwrap()
            .get("objects")
            .unwrap()
            .as_array()
            .unwrap();

        let mut objects: Vec<Object<G2dTexture>> = Vec::default();
        for obj in solid_objects {
            let json_obj = obj.as_object().unwrap();
            let x = json_obj.get("x").unwrap().as_f64().unwrap();
            let y = json_obj.get("y").unwrap().as_f64().unwrap();
            let w = json_obj.get("width").unwrap().as_f64().unwrap();
            let h = json_obj.get("height").unwrap().as_f64().unwrap();
            let mut o = Object::<G2dTexture>::new();
            o.set_size(w, h);
            o.set_position(x, y);
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
        let objects = &mut self.objects;

        let tilemap = &mut self.tilemap;

        tilemap.get_sprite().unwrap().set_src_rect([
            camera.position.x.max(0.0),
            0.0,
            camera_width,
            camera_height,
        ]);

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
                if obj.x() < window_size.width && obj.xw() >= 0.0 {
                    object.draw(transform, g);
                }
            }

            player.draw(transform, g);
        });
    }

    pub fn update(&mut self, dt: f64) {
        self.player.update(dt);

        for object in self.objects.iter() {
            self.player.collide_with(object);
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
