use cgmath::Vector2;
use find_folder::Search;
use fps_counter::FPSCounter;
use graphics::Transformed;
use libs::bricks::{Brick, BrickType};
use libs::collider::Side;
use libs::textures::TextureManager;
use libs::tilemap::TileMap;
use piston_window::{
    clear, Button, ButtonArgs, ButtonEvent, ButtonState, EventLoop, G2dTexture, GenericEvent,
    ImageSize, Key, PistonWindow, RenderEvent, Size, UpdateEvent, WindowSettings,
};
use serde_json::{from_reader, Value};
use sprite::Sprite;
use std::cell::RefCell;
use std::fs::File;
use std::rc::Rc;

mod libs {
    pub mod animations;
    pub mod bricks;
    pub mod camera;
    pub mod collider;
    pub mod controller;
    pub mod core;
    pub mod object;
    pub mod physics;
    pub mod player;
    pub mod sprites_manager;
    pub mod spritesheet;
    pub mod textures;
    pub mod tilemap;
    pub mod transform;
}

use libs::camera::Camera;
use libs::core::{Destroyable, Drawable, Entity, Object2D, Updatable};
use libs::object::Object;
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
    Brick,
    Coin,
}
pub struct Game {
    window: PistonWindow,
    size: Size,
    camera: Camera,
    tilemap: TileMap,
    player: Player,
    player2: Player,
    objects: Vec<Brick>,
    player_index: usize,
}
impl Game {
    pub fn new(size: Size, viewport_size: Size) -> Result<Self, String> {
        let mut window: PistonWindow = WindowSettings::new("Super Goomba Bros", size)
            .exit_on_esc(true)
            .build()
            .map_err(|e| format!("Failed to build PistonWindow: {:?}", e))?;
        window.set_ups(60);

        let context = Rc::new(RefCell::new(window.create_texture_context()));
        let mut texture_manager = TextureManager::new(context.clone());
        let map_texture = texture_manager.load_texture("level_1_map", "world_1_1.png")?;
        let mario_texture = texture_manager.load_texture("mario", "Mario.png")?;
        let player2_texture = texture_manager.load_texture("player", "player.png")?;
        let tileset_texture = texture_manager.load_texture("tileset", "tileset1.png")?;

        let mut tilemap = TileMap::new();

        let map_size: Size = map_texture.get_size().into();
        let mut map_sprite = Sprite::from_texture(map_texture);
        map_sprite.set_anchor(0.0, 0.0);
        tilemap.register_level(1, map_sprite);

        let player_config = Self::load_player_mario_sonic_style_sprite_sheet_config();
        let mut player = Self::load_player(Rc::clone(&mario_texture), player_config);
        Self::load_mario_sonic_animation(&mut player);

        let player2_config = Self::load_mario_default_sprite_sheet_config();
        let mut player2 = Self::load_player(Rc::clone(&player2_texture), player2_config);
        Self::load_luigi_animation(&mut player2);

        let camera = Camera::new(size, viewport_size, map_size);

        let objects = Self::load_objects(tileset_texture);

        let game = Self {
            window,
            size,
            camera,
            tilemap,
            player,
            player2,
            objects,
            player_index: 0,
        };

        Ok(game)
    }

    fn load_player(texture: Rc<G2dTexture>, config: SpriteSheetConfig) -> Player {
        let player_sprite_sheet = SpriteSheet::new(texture);
        let mut player = Player::new();
        player.set_sprite_sheet(player_sprite_sheet, config);

        player
    }

    fn load_mario_default_sprite_sheet_config() -> SpriteSheetConfig {
        SpriteSheetConfig {
            offset: Vector2::new(80.0, 34.0),
            spacing: Vector2::new(1.0, 47.0),
            grid: [21, 11],
            sprite_size: Size::from([16.0, 16.0]),
            scale: Vector2::new(1.0, 1.0),
        }
    }

    fn load_player_mario_sonic_style_sprite_sheet_config() -> SpriteSheetConfig {
        SpriteSheetConfig {
            offset: Vector2::from([0.0, 0.0]),
            spacing: Vector2::from([0.0, 0.0]),
            grid: [10, 6],
            sprite_size: Size::from([42.0, 42.0]),
            scale: Vector2::new(16.0 / 42.0, 16.0 / 42.0),
        }
    }

    fn load_mario_sonic_animation(player: &mut Player) {
        player.add_animation("idle", vec![[0, 0]]);
        player.add_animation("jump", vec![[4, 0]]);
        player.add_animation("jump-right", vec![[4, 0]]);
        player.add_animation("fall", vec![[4, 1]]);
        player.add_animation("skid", vec![[3, 0], [3, 1]]);
        player.add_animation("walk", vec![[5, 0], [5, 1], [5, 2], [5, 3], [5, 4], [5, 5]]);
        player.add_animation("run", vec![[6, 0], [6, 1], [6, 2]]);
        player.add_animation("push", vec![[8, 0], [8, 1], [8, 2], [8, 3]]);
    }

    fn load_mario_animation(player: &mut Player) {
        player.add_animation("idle", vec![[0, 0]]);
        player.add_animation("jump", vec![[0, 5]]);
        player.add_animation("jump-right", vec![[0, 9]]);
        player.add_animation("fall", vec![[0, 8]]);
        player.add_animation("walk", vec![[0, 1], [0, 2], [0, 3]]);
        player.add_animation("skid", vec![[0, 4]]);
    }

    fn load_luigi_animation(player: &mut Player) {
        player.add_animation("idle", vec![[1, 0]]);
        player.add_animation("jump", vec![[1, 5]]);
        player.add_animation("walk", vec![[1, 1], [1, 2], [1, 3]]);
    }

    fn load_objects(texture: Rc<G2dTexture>) -> Vec<Brick> {
        let mut tileset = SpriteSheet::new(texture.clone());
        let tileset_config = SpriteSheetConfig {
            offset: Vector2::from([0.0, 0.0]),
            spacing: Vector2::from([0.0, 0.0]),
            grid: [1, 2],
            sprite_size: Size::from([16.0, 16.0]),
            scale: Vector2::new(1.0, 1.0),
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

        let mut objects: Vec<Brick> = Vec::default();
        for obj in solid_objects {
            let json_obj = obj.as_object().unwrap();
            let x = json_obj.get("x").unwrap().as_f64().unwrap();
            let y = json_obj.get("y").unwrap().as_f64().unwrap();
            let w = json_obj.get("width").unwrap().as_f64().unwrap();
            let h = json_obj.get("height").unwrap().as_f64().unwrap();
            let name = json_obj.get("name").unwrap().as_str().unwrap().to_string();

            if name.to_string().trim() == "brick".to_string().trim() {
                let mut o = Brick::new(BrickType::Block, Rc::clone(&brick_sprite));
                o.get_transform_mut().set_size(w, h);
                o.get_transform_mut().set_position(x, y);
                objects.push(o);
            } else if name.to_string().trim() == "coin".to_string().trim() {
                let mut o = Brick::new(BrickType::Coin, Rc::clone(&coin_sprite));
                o.get_transform_mut().set_size(w, h);
                o.get_transform_mut().set_position(x, y);
                objects.push(o);
            } else {
                let mut o = Brick::new(BrickType::Ground, Rc::clone(&brick_sprite));
                o.get_transform_mut().set_size(w, h);
                o.get_transform_mut().set_position(x, y);
                objects.push(o);
            }
        }

        objects
    }

    pub fn render<E>(&mut self, e: &E)
    where
        E: GenericEvent,
    {
        let window_size = self.size;
        let camera = &mut self.camera;
        let translate_x = (window_size.width - (camera.viewport_size.width * camera.scale)) / 2.0;
        let translate_y = (window_size.height - (camera.viewport_size.height * camera.scale)) / 2.0;

        let player = &mut self.player;
        let player2 = &mut self.player2;
        let objects = &mut self.objects;

        let tilemap = &mut self.tilemap;

        self.window.draw_2d(e, |_, g, _d| {
            clear([0.0, 0.0, 0.0, 0.5], g);
        });

        // render map
        self.window.draw_2d(e, |c, g, _d| {
            tilemap.draw(
                c.trans(translate_x, translate_y)
                    .scale(camera.scale, camera.scale)
                    .transform,
                g,
            );
        });

        // render player and objects
        self.window.draw_2d(e, |c, g, _d| {
            let transform = c
                .trans(
                    -camera.position.x * camera.scale + translate_x,
                    -camera.position.y * camera.scale + translate_y,
                )
                .scale(camera.scale, camera.scale)
                .transform;
            for object in objects.iter_mut().filter(|o| !o.is_destroyed()) {
                let obj = object.get_transform();
                if obj.x() < camera.position.x + camera.viewport_size.width
                    && obj.xw() >= camera.position.x
                {
                    object.draw(transform, g);
                }
            }

            player.draw(transform, g);
            if player2.get_transform().x() < camera.position.x + camera.viewport_size.width
                && player2.get_transform().xw() >= camera.position.x
            {
                player2.draw(transform, g);
            }
        });
    }

    pub fn update(&mut self, dt: f64) {
        self.player.update(dt);
        self.player
            .respawn_player_if_overflow(self.camera.map_size.height + 100.0);

        self.player2.update(dt);
        self.player2
            .respawn_player_if_overflow(self.camera.map_size.height + 100.0);

        for object in self.objects.iter_mut().filter(|o| !o.is_destroyed()) {
            let transform = *object.get_transform();
            if let Some(side) = self.player.collide_with(&transform) {
                match side {
                    Side::TOP => {
                        let player_center = self.player.get_transform().center_xw();
                        if player_center > transform.x() && player_center < transform.xw() {
                            object.destroy();
                        }
                    }
                    _ => {}
                };
            }

            if let Some(side) = self.player2.collide_with(&transform) {
                match side {
                    Side::TOP => {
                        let player_center = self.player2.get_transform().center_xw();
                        if player_center > transform.x() && player_center < transform.xw() {
                            object.destroy();
                        }
                    }
                    _ => {}
                };
            }
        }

        if self.player_index == 0 {
            self.camera.follow_player(&self.player);
        } else {
            self.camera.follow_player(&self.player2);
        }
        self.camera.update_tilemap(&mut self.tilemap);
        self.camera.update(dt);
    }

    pub fn update_input(&mut self, args: ButtonArgs) {
        if let Button::Keyboard(key) = args.button {
            if self.player_index == 0 {
                self.player.update_input(key, args.state);
            } else {
                self.player2.update_input(key, args.state);
            }

            if args.state == ButtonState::Press && key == Key::F1 {
                self.player.reset_input();
                self.player2.reset_input();
                self.player_index = if self.player_index == 0 { 1 } else { 0 }
            }
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

    let object: Vec<Box<dyn Entity>> = vec![Box::new(Object::new("Object".to_string()))];
    object.first().unwrap().get_transform();

    if let Ok(mut game) = Game::new(window_size, viewport_size) {
        music::start::<Music, Sound, _>(16, || {
            music::bind_music_file(Music::World1_1, "./assets/main_theme.mp3");
            music::bind_sound_file(Sound::Jump, "./assets/jump.mp3");
            music::bind_sound_file(Sound::Brick, "./assets/brick.wav");
            music::bind_sound_file(Sound::Coin, "./assets/coin.wav");
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
    } else {
        eprintln!("Error!");
    }
}
