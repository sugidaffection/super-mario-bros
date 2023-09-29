use crate::{Music, Sound};

use super::core::camera::Camera;
use super::core::sprite_sheet::{SpriteSheet, SpriteSheetConfig};
use super::core::stages::{Stage, StageManager};
use super::core::textures::TextureManager;
use super::entities::player::Player;
use super::prelude::{Drawable, GameBuilder, Object2D, Trans, Updatable};
use super::ui::progress_bar::ProgressBar;
use cgmath::Vector2;
use fps_counter::FPSCounter;
use graphics::{text, Transformed};
use piston_window::{
    clear, Button, ButtonArgs, ButtonEvent, ButtonState, EventLoop, G2dTexture, GenericEvent,
    Glyphs, Key, PistonWindow, RenderEvent, Size, UpdateEvent, WindowSettings,
};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Debug)]
pub enum GameState {
    Init,
    Loading,
    Run,
    Pause,
    Stop,
}
pub struct Game {
    window_settings: WindowSettings,
    window: Option<PistonWindow>,
    glyphs: Option<Glyphs>,
    texture_manager: Option<Rc<RefCell<TextureManager>>>,
    stage_manager: Option<StageManager>,
    players: Vec<Player>,
    current_player: usize,
    state: GameState,
    timer: FPSCounter,
    fps: usize,
    camera: Camera,
}

impl Game {
    pub fn start(&mut self) {
        music::start::<Music, Sound, _>(16, || {
            music::bind_music_file(Music::World1_1, "./assets/sounds/main_theme.mp3");
            music::bind_sound_file(Sound::Jump, "./assets/sounds/jump.mp3");
            music::bind_sound_file(Sound::Brick, "./assets/sounds/brick.wav");
            music::bind_sound_file(Sound::Coin, "./assets/sounds/coin.wav");

            music::set_volume(music::MAX_VOLUME);

            let size = self.window_settings.get_size();
            let mut progress_bar = ProgressBar::new([1.0, 0.0, 0.0, 1.0], [1.0, 0.0, 0.0, 1.0]);
            let progress_bar_size = size.width * 0.8;
            progress_bar.set_pos([
                (size.width - progress_bar_size) / 2.0,
                size.height / 2.0 - 8.0,
            ]);
            progress_bar.set_size([progress_bar_size, 16.0]);
            let mut loading_progress_value = 0.0;
            self.state = GameState::Loading;
            while let Some(e) = self.window.as_mut().and_then(|w| w.next()) {
                if self.state == GameState::Loading {
                    self.window.as_mut().unwrap().draw_2d(&e, |c, g, _d| {
                        clear([0.0, 0.0, 0.0, 0.5], g);
                        progress_bar.draw(c.transform, g);
                    });

                    if loading_progress_value <= 1.0 {
                        if let Some(u) = e.update_args() {
                            loading_progress_value += u.dt;
                        }
                    } else if self.state == GameState::Loading {
                        self.state = GameState::Run;
                        music::play_music(&Music::World1_1, music::Repeat::Forever);
                    }
                    progress_bar.set_value(loading_progress_value);
                } else {
                    if let Some(_) = e.render_args() {
                        self.render(&e);
                    }

                    if self.state == GameState::Run {
                        if let Some(u) = e.update_args() {
                            self.fps = self.timer.tick();
                            self.update(u.dt);
                        }

                        if let Some(args) = e.button_args() {
                            self.update_input(args);
                        }
                    }
                }
            }
        });
    }

    pub fn init(&mut self) -> Result<(), String> {
        self.load_textures()?;
        self.load_stages()?;
        self.load_players()?;
        Ok(())
    }

    pub fn load_textures(&mut self) -> Result<(), String> {
        if let Some(texture_manager) = self.texture_manager.as_mut() {
            texture_manager
                .borrow_mut()
                .load_texture("level_1_map", "background/world_1_1.png")?
                .load_texture("mario", "spritesheet/Mario.png")?
                .load_texture("player", "spritesheet/player.png")?
                .load_texture("tileset", "spritesheet/tileset1.png")?
                .load_texture("enemies", "spritesheet/enemies.png")?;
        }

        Ok(())
    }

    pub fn load_stages(&mut self) -> Result<(), String> {
        if let Some(texture_manager) = self.texture_manager.as_mut() {
            let map_texture = texture_manager.borrow().get_texture("level_1_map")?;
            let mut stage = Stage::new(map_texture);
            stage.load_objects_from_file("world_1_1.tmj")?;
            if let Some(stage_manager) = self.stage_manager.as_mut() {
                stage_manager.register_stage("level 1", stage);
                stage_manager.set_current_stage("level 1");
                stage_manager.load_stage()?;
            }
        }

        Ok(())
    }

    fn create_player(texture: Rc<G2dTexture>, config: SpriteSheetConfig) -> Player {
        let player_sprite_sheet = SpriteSheet::new(texture, config);
        let mut player = Player::new();
        player.set_sprite_sheet(player_sprite_sheet);

        player
    }

    fn create_player_one(&mut self) -> Result<(), String> {
        let player_config = SpriteSheetConfig {
            offset: Vector2::from([0.0, 0.0]),
            spacing: Vector2::from([0.0, 0.0]),
            grid: [10, 6],
            sprite_size: Size::from([42.0, 42.0]),
            scale: Vector2::new(16.0 / 42.0, 16.0 / 42.0),
        };
        let mario_texture = self
            .texture_manager
            .as_ref()
            .unwrap()
            .borrow()
            .get_texture("mario")?;
        let mut player = Self::create_player(mario_texture, player_config);
        player.add_animation("idle", vec![[0, 0]]);
        player.add_animation("jump", vec![[4, 0]]);
        player.add_animation("jump-right", vec![[4, 0]]);
        player.add_animation("fall", vec![[4, 1]]);
        player.add_animation("skid", vec![[3, 0], [3, 1]]);
        player.add_animation("walk", vec![[5, 0], [5, 1], [5, 2], [5, 3], [5, 4], [5, 5]]);
        player.add_animation("run", vec![[6, 0], [6, 1], [6, 2]]);
        player.add_animation("push", vec![[8, 0], [8, 1], [8, 2], [8, 3]]);

        self.players.push(player);

        Ok(())
    }

    fn create_player_two(&mut self) -> Result<(), String> {
        let player_config = SpriteSheetConfig {
            offset: Vector2::new(80.0, 34.0),
            spacing: Vector2::new(1.0, 47.0),
            grid: [21, 11],
            sprite_size: Size::from([16.0, 16.0]),
            scale: Vector2::new(1.0, 1.0),
        };
        let player_texture = self
            .texture_manager
            .as_ref()
            .unwrap()
            .borrow()
            .get_texture("player")?;
        let mut player = Self::create_player(Rc::clone(&player_texture), player_config);
        player.add_animation("idle", vec![[0, 0]]);
        player.add_animation("jump", vec![[0, 5]]);
        player.add_animation("jump-right", vec![[0, 9]]);
        player.add_animation("fall", vec![[0, 8]]);
        player.add_animation("walk", vec![[0, 1], [0, 2], [0, 3]]);
        player.add_animation("skid", vec![[0, 4]]);

        self.players.push(player);

        Ok(())
    }

    fn load_players(&mut self) -> Result<(), String> {
        self.create_player_one()?;
        self.create_player_two()?;

        Ok(())
    }

    fn render<E>(&mut self, e: &E)
    where
        E: GenericEvent,
    {
        let window_size = self.window_settings.get_size();
        let camera = &mut self.camera;
        let translate_x = (window_size.width - (camera.viewport_size.width * camera.scale)) / 2.0;
        let translate_y = (window_size.height - (camera.viewport_size.height * camera.scale)) / 2.0;

        let players = &mut self.players;
        let current_player = self.current_player;
        let stage_manager = &mut self.stage_manager;
        let glyphs = &mut self.glyphs;
        let fps = self.fps;
        let is_pause = self.state == GameState::Pause;
        let text = text::Text::new_color([1.0, 1.0, 1.0, 1.0], 32);
        let stage_manager = stage_manager.as_mut().unwrap();
        let window = self.window.as_mut().unwrap();

        window.draw_2d(e, |c, g, _d| {
            clear([0.0, 0.0, 0.0, 0.5], g);

            if let Some(stage) = stage_manager.get_current_stage() {
                stage.draw(
                    c.trans(translate_x, translate_y)
                        .scale(camera.scale, camera.scale)
                        .transform,
                    g,
                );
            }

            let transform = c.transform.trans(0.0, 30.0);
            text.draw(
                &format!("FPS: {}", fps),
                glyphs.as_mut().unwrap(),
                &c.draw_state,
                transform,
                g,
            )
            .unwrap();

            let transform = c.transform.trans(0.0, 60.0);

            if let Some(player) = players.get_mut(current_player) {
                let player_pos = player.get_transform().get_position();
                text.draw(
                    &format!("Player 1: {:.1}, {:.1}", player_pos.x, player_pos.y),
                    glyphs.as_mut().unwrap(),
                    &c.draw_state,
                    transform,
                    g,
                )
                .unwrap();
            }

            if is_pause {
                let transform = c
                    .transform
                    .trans((window_size.width) / 2.0, (window_size.height - 64.0) / 2.0);
                text.draw(
                    &format!("PAUSED"),
                    glyphs.as_mut().unwrap(),
                    &c.draw_state,
                    transform,
                    g,
                )
                .unwrap();
            }

            glyphs.as_mut().unwrap().factory.encoder.flush(_d);

            let transform = c
                .trans(
                    -camera.position.x * camera.scale + translate_x,
                    -camera.position.y * camera.scale + translate_y,
                )
                .scale(camera.scale, camera.scale)
                .transform;

            stage_manager.draw(transform, g);
            for player in players {
                player.draw(transform, g);
            }
        });
    }

    pub fn update(&mut self, dt: f64) {
        if let Some(stage_manager) = self.stage_manager.as_mut() {
            stage_manager.update(dt);

            for player in self.players.iter_mut() {
                player.update(dt);
                player.respawn_player_if_overflow(self.camera.viewport_size.height + 100.0);

                stage_manager.collide_with(player);
            }
            if let Some(player) = self.players.get(self.current_player) {
                self.camera.follow_player(player);
            }

            if let Some(stage) = stage_manager.get_current_stage() {
                self.camera.update_camera_view(&mut stage.map);
            }
            self.camera.update(dt);
        }
    }

    pub fn update_input(&mut self, args: ButtonArgs) {
        if let Button::Keyboard(key) = args.button {
            if args.state == ButtonState::Release && key == Key::Return {
                self.state = if self.state == GameState::Run {
                    GameState::Pause
                } else {
                    GameState::Run
                };
            }

            if let Some(player) = self.players.get_mut(self.current_player) {
                player.update_input(key, args.state);
            }

            if args.state == ButtonState::Press && key == Key::F1 {
                for player in self.players.iter_mut() {
                    player.reset_input();
                }
                self.current_player += 1;
                self.current_player %= self.players.len();
            }
        }
    }
}

impl GameBuilder for Game {
    fn new() -> Self {
        let scale = 3.0;
        let width = 352.0;
        let height = 224.0;
        let window_size: Size = Size::from([width * scale, height * scale]);
        let viewport_size: Size = Size::from([width, height]);

        let window_settings: WindowSettings = WindowSettings::new("Super Mario Bros", window_size);
        let camera = Camera::new(window_size, viewport_size);

        Self {
            window_settings,
            window: None,
            glyphs: None,
            state: GameState::Init,
            timer: FPSCounter::default(),
            fps: 0,
            players: Vec::new(),
            current_player: 0,
            texture_manager: None,
            stage_manager: None,
            camera,
        }
    }

    fn build(mut self) -> Result<(), String> {
        let mut window: PistonWindow = self
            .window_settings
            .build()
            .map_err(|e| format!("Failed to build PistonWindow: {:?}", e))?;
        window.set_ups(60);

        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .map_err(|err| err.to_string())?;

        self.glyphs = window
            .load_font(assets.join("FiraSans-Regular.ttf"))
            .map_err(|err| err.to_string())?
            .into();

        let context = Rc::new(RefCell::new(window.create_texture_context()));
        self.window = Some(window);

        let texture_manager = Rc::new(RefCell::new(TextureManager::new(context.clone())));
        let stage_manager = StageManager::new(texture_manager.clone());
        self.texture_manager = Some(texture_manager);
        self.stage_manager = Some(stage_manager);

        self.init()?;
        self.start();

        Ok(())
    }
}
