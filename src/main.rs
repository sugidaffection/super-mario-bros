use cgmath::Vector2;
use find_folder::Search;
use fps_counter::FPSCounter;
use graphics::glyph_cache::rusttype::GlyphCache;
use graphics::{text, Context, Transformed};
use libs::bricks::{Brick, BrickType};
use libs::collider::{Collision, Side};
use libs::enemies::Enemy;
use libs::textures::TextureManager;
use libs::tilemap::TileMap;
use piston_window::{
    clear, Button, ButtonArgs, ButtonEvent, ButtonState, EventLoop, G2dTexture, G2dTextureContext,
    GenericEvent, Glyphs, Key, PistonWindow, RenderEvent, Size, UpdateEvent, WindowSettings,
};
use serde_json::{from_reader, Value};
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
    pub mod enemies;
    pub mod object;
    pub mod physics;
    pub mod player;
    pub mod sprite_sheet;
    pub mod sprite_sheet_manager;
    pub mod textures;
    pub mod tilemap;
    pub mod transform;
}

use libs::camera::Camera;
use libs::core::{Destroyable, Drawable, Entity, Object2D, Updatable};
use libs::object::Object;
use libs::player::Player;
use libs::sprite_sheet::{SpriteSheet, SpriteSheetConfig};
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

#[derive(PartialEq)]
pub enum GameState {
    Run,
    Pause,
    Stop,
}
pub struct Game {
    window: PistonWindow,
    size: Size,
    camera: Camera,
    tilemap: TileMap,
    player: Player,
    player2: Player,
    objects: Vec<Brick>,
    enemies: Vec<Enemy>,
    player_index: usize,
    state: GameState,
    glyphs: Glyphs,
    timer: FPSCounter,
    fps: usize,
}
impl Game {
    pub fn new(size: Size, viewport_size: Size) -> Result<Self, String> {
        let mut window: PistonWindow = WindowSettings::new("Super Goomba Bros", size)
            .exit_on_esc(true)
            .build()
            .map_err(|e| format!("Failed to build PistonWindow: {:?}", e))?;
        window.set_ups(60);

        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();

        let glyphs = window
            .load_font(assets.join("FiraSans-Regular.ttf"))
            .unwrap();

        let context = Rc::new(RefCell::new(window.create_texture_context()));
        let mut texture_manager = Self::load_textures(Rc::clone(&context))?;
        let enemies = Self::load_enemies(&mut texture_manager)?;
        let tilemap = Self::load_map(&mut texture_manager)?;

        let player = Self::load_player_one(&mut texture_manager)?;
        let player2 = Self::load_player_one(&mut texture_manager)?;

        let camera = Camera::new(size, viewport_size, tilemap.get_size());

        let tileset_texture = texture_manager.get_texture("tileset")?;
        let objects = Self::load_objects(tileset_texture);

        let game = Self {
            window,
            size,
            camera,
            tilemap,
            player,
            player2,
            objects,
            enemies,
            player_index: 0,
            state: GameState::Run,
            glyphs,
            timer: FPSCounter::default(),
            fps: 0,
        };

        Ok(game)
    }

    pub fn load_textures(
        context: Rc<RefCell<G2dTextureContext>>,
    ) -> Result<TextureManager, String> {
        let mut texture_manager = TextureManager::new(context);
        texture_manager.load_texture("level_1_map", "background/world_1_1.png")?;
        texture_manager.load_texture("mario", "spritesheet/Mario.png")?;
        texture_manager.load_texture("player", "spritesheet/player.png")?;
        texture_manager.load_texture("tileset", "spritesheet/tileset1.png")?;
        texture_manager.load_texture("enemies", "spritesheet/enemies.png")?;

        Ok(texture_manager)
    }

    pub fn load_enemies(texture_manager: &mut TextureManager) -> Result<Vec<Enemy>, String> {
        let enemies_texture = texture_manager.get_texture("enemies")?;
        let enemies_spritesheet = SpriteSheet::new(
            enemies_texture,
            SpriteSheetConfig {
                grid: [8, 50],
                sprite_size: Size::from([16.0, 16.0]),
                spacing: Vector2::from([0.0, 0.0]),
                offset: Vector2::from([0.0, 0.0]),
                scale: Vector2::from([1.0, 1.0]),
            },
        );

        let mut goomba = Enemy::new("Goomba", Vector2::from([480.0, 10.0]));
        goomba.set_sprite_sheet(enemies_spritesheet);
        goomba.add_animation("walk", vec![[1, 0], [1, 1]]);
        goomba.add_animation("dead", vec![[1, 2]]);
        goomba.play_animation("walk");
        let enemies = vec![goomba];
        Ok(enemies)
    }

    pub fn load_map(texture_manager: &mut TextureManager) -> Result<TileMap, String> {
        let map_texture = texture_manager.get_texture("level_1_map")?;

        Ok(TileMap::new(map_texture))
    }

    fn load_player(texture: Rc<G2dTexture>, config: SpriteSheetConfig) -> Player {
        let player_sprite_sheet = SpriteSheet::new(texture, config);
        let mut player = Player::new();
        player.set_sprite_sheet(player_sprite_sheet);

        player
    }

    fn load_player_one(texture_manager: &mut TextureManager) -> Result<Player, String> {
        let player_config = Self::load_player_mario_sonic_style_sprite_sheet_config();
        let mario_texture = texture_manager.get_texture("mario")?;
        let mut player = Self::load_player(mario_texture, player_config);
        Self::load_mario_sonic_animation(&mut player);

        Ok(player)
    }

    fn load_player_two(texture_manager: &mut TextureManager) -> Result<Player, String> {
        let player_config = Self::load_mario_default_sprite_sheet_config();
        let player_texture = texture_manager.get_texture("player")?;
        let mut player = Self::load_player(Rc::clone(&player_texture), player_config);
        Self::load_luigi_animation(&mut player);

        Ok(player)
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
        let mut tileset = SpriteSheet::new(
            texture.clone(),
            SpriteSheetConfig {
                offset: Vector2::from([0.0, 0.0]),
                spacing: Vector2::from([0.0, 0.0]),
                grid: [1, 2],
                sprite_size: Size::from([16.0, 16.0]),
                scale: Vector2::new(1.0, 1.0),
            },
        );
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
        let enemies = &mut self.enemies;

        let tilemap = &mut self.tilemap;
        let glyphs = &mut self.glyphs;
        let fps = self.fps;
        let is_pause = self.state == GameState::Pause;
        let text = text::Text::new_color([1.0, 1.0, 1.0, 1.0], 32);

        self.window.draw_2d(e, |c, g, _d| {
            clear([0.0, 0.0, 0.0, 0.5], g);
            tilemap.draw(
                c.trans(translate_x, translate_y)
                    .scale(camera.scale, camera.scale)
                    .transform,
                g,
            );

            let transform = c.transform.trans(0.0, 30.0);
            text.draw(
                &format!("FPS: {}", fps),
                glyphs,
                &c.draw_state,
                transform,
                g,
            )
            .unwrap();

            let transform = c.transform.trans(0.0, 60.0);
            let player_pos = player.get_transform().get_position();
            text.draw(
                &format!("Player 1: {:.1}, {:.1}", player_pos.x, player_pos.y),
                glyphs,
                &c.draw_state,
                transform,
                g,
            )
            .unwrap();

            if is_pause {
                let transform = c
                    .transform
                    .trans((window_size.width) / 2.0, (window_size.height - 64.0) / 2.0);
                text.draw(&format!("PAUSED"), glyphs, &c.draw_state, transform, g)
                    .unwrap();
            }

            glyphs.factory.encoder.flush(_d);

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

            for enemy in enemies.iter_mut() {
                enemy.draw(transform, g);
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
        self.fps = self.timer.tick();
        if self.state == GameState::Run {
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

                for enemy in self.enemies.iter_mut() {
                    enemy.collide_with(&transform);
                }
            }

            for enemy in self.enemies.iter_mut() {
                let transform = *enemy.get_transform();
                enemy.update(dt);
                if !enemy.is_dead {
                    match self.player.collide_with(&transform) {
                        Some(Side::BOTTOM) => {
                            enemy.dead();
                        }
                        _ => {}
                    };
                    match self.player2.collide_with(&transform) {
                        Some(Side::BOTTOM) => {
                            enemy.dead();
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
    }

    pub fn update_input(&mut self, args: ButtonArgs) {
        if let Button::Keyboard(key) = args.button {
            if args.state == ButtonState::Press && key == Key::Return {
                self.state = if self.state == GameState::Run {
                    GameState::Pause
                } else {
                    GameState::Run
                };
            }
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

    music::start::<Music, Sound, _>(16, || {
        music::bind_music_file(Music::World1_1, "./assets/sounds/main_theme.mp3");
        music::bind_sound_file(Sound::Jump, "./assets/sounds/jump.mp3");
        music::bind_sound_file(Sound::Brick, "./assets/sounds/brick.wav");
        music::bind_sound_file(Sound::Coin, "./assets/sounds/coin.wav");
        music::set_volume(music::MAX_VOLUME);
        music::play_music(&Music::World1_1, music::Repeat::Forever);
        match Game::new(window_size, viewport_size) {
            Ok(mut game) => {
                let mut fps = 0;
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
            }
            Err(error) => {
                eprintln!("Error! {:?}", error);
            }
        }
    });
}
