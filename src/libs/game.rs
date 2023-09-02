use super::bricks::{Brick, BrickType};
use super::camera::Camera;
use super::collider::{Collision, Side};
use super::core::{Drawable, Object2D, Updatable};
use super::enemies::Enemy;
use super::object::Object;
use super::player::Player;
use super::progress_bar::ProgressBar;
use super::sprite_sheet::{SpriteSheet, SpriteSheetConfig};
use super::stages::{Stage, StageManager};
use super::textures::TextureManager;
use super::transform::Trans;
use cgmath::Vector2;
use find_folder::Search;
use fps_counter::FPSCounter;
use graphics::{text, Transformed};
use piston_window::{
    clear, Button, ButtonArgs, ButtonEvent, ButtonState, EventLoop, G2dTexture, G2dTextureContext,
    GenericEvent, Glyphs, Key, PistonWindow, RenderEvent, Size, UpdateEvent, WindowSettings,
};
use serde_json::{from_reader, Value};
use std::cell::RefCell;
use std::fs::File;
use std::rc::Rc;

#[derive(PartialEq, Debug)]
pub enum GameState {
    Loading,
    Run,
    Pause,
    Stop,
}
pub struct Game {
    window: PistonWindow,
    size: Size,
    camera: Camera,
    texture_manager: Rc<RefCell<TextureManager>>,
    stage_manager: StageManager,
    players: Vec<Player>,
    current_player: usize,
    state: GameState,
    glyphs: Glyphs,
    timer: FPSCounter,
    fps: usize,
    loader: ProgressBar,
    loading_progress_value: f64,
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
            .map_err(|err| err.to_string())?;

        let glyphs = window
            .load_font(assets.join("FiraSans-Regular.ttf"))
            .map_err(|err| err.to_string())?;

        let context = Rc::new(RefCell::new(window.create_texture_context()));
        let texture_manager = Rc::new(RefCell::new(TextureManager::new(context.clone())));
        let stage_manager = StageManager::new(texture_manager.clone());
        let camera = Camera::new(size, viewport_size);

        let mut progress_bar = ProgressBar::new([1.0, 0.0, 0.0, 1.0], [1.0, 0.0, 0.0, 1.0]);
        let progress_bar_size = size.width * 0.8;
        progress_bar.set_pos([
            (size.width - progress_bar_size) / 2.0,
            size.height / 2.0 - 8.0,
        ]);
        progress_bar.set_size([progress_bar_size, 16.0]);

        let mut game = Self {
            window,
            size,
            camera,
            texture_manager,
            stage_manager,
            current_player: 0,
            state: GameState::Loading,
            glyphs,
            timer: FPSCounter::default(),
            fps: 0,
            loader: progress_bar,
            loading_progress_value: 0.0,
            players: Vec::new(),
        };

        game.init()?;

        Ok(game)
    }

    pub fn start(&mut self) {
        while let Some(e) = self.window.next() {
            if let Some(_) = e.render_args() {
                self.render(&e);
            }

            if let Some(u) = e.update_args() {
                self.update(u.dt);
            }

            if let Some(args) = e.button_args() {
                self.update_input(args);
            }
        }
    }

    pub fn init(&mut self) -> Result<(), String> {
        self.load_textures()?;
        self.load_stages()?;
        self.load_players()?;
        Ok(())
    }

    pub fn load_textures(&mut self) -> Result<(), String> {
        self.texture_manager
            .borrow_mut()
            .load_texture("level_1_map", "background/world_1_1.png")?;
        self.texture_manager
            .borrow_mut()
            .load_texture("mario", "spritesheet/Mario.png")?;
        self.texture_manager
            .borrow_mut()
            .load_texture("player", "spritesheet/player.png")?;
        self.texture_manager
            .borrow_mut()
            .load_texture("tileset", "spritesheet/tileset1.png")?;
        self.texture_manager
            .borrow_mut()
            .load_texture("enemies", "spritesheet/enemies.png")?;

        Ok(())
    }

    pub fn load_stages(&mut self) -> Result<(), String> {
        let map_texture = self.texture_manager.borrow().get_texture("level_1_map")?;
        let mut stage = Stage::new(map_texture);
        stage.load_objects_from_file("world_1_1.tmj")?;
        self.stage_manager.register_stage("level 1", stage);
        self.stage_manager.set_current_stage("level 1");
        self.stage_manager.load_objects()?;
        self.stage_manager.load_enemies()?;

        Ok(())
    }

    fn load_player(texture: Rc<G2dTexture>, config: SpriteSheetConfig) -> Player {
        let player_sprite_sheet = SpriteSheet::new(texture, config);
        let mut player = Player::new();
        player.set_sprite_sheet(player_sprite_sheet);

        player
    }

    fn load_player_one(&mut self) -> Result<(), String> {
        let player_config = Self::load_player_mario_sonic_style_sprite_sheet_config();
        let mario_texture = self.texture_manager.borrow().get_texture("mario")?;
        let mut player = Self::load_player(mario_texture, player_config);
        Self::load_mario_sonic_animation(&mut player);

        self.players.push(player);

        Ok(())
    }

    fn load_player_two(&mut self) -> Result<(), String> {
        let player_config = Self::load_mario_default_sprite_sheet_config();
        let player_texture = self.texture_manager.borrow().get_texture("player")?;
        let mut player = Self::load_player(Rc::clone(&player_texture), player_config);
        Self::load_luigi_animation(&mut player);

        self.players.push(player);

        Ok(())
    }

    fn load_players(&mut self) -> Result<(), String> {
        self.load_player_one()?;
        self.load_player_two()?;

        Ok(())
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

    fn load_objects(&mut self) -> Result<(), String> {
        self.stage_manager.load_objects()?;
        Ok(())
    }

    pub fn render<E>(&mut self, e: &E)
    where
        E: GenericEvent,
    {
        let loader = &mut self.loader;

        if self.state == GameState::Loading {
            self.window.draw_2d(e, |c, g, _d| {
                clear([0.0, 0.0, 0.0, 0.5], g);
                loader.draw(c.transform, g);
            });
        } else {
            self.render_game(e);
        }
    }

    fn render_game<E>(&mut self, e: &E)
    where
        E: GenericEvent,
    {
        let window_size = self.size;
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

        self.window.draw_2d(e, |c, g, _d| {
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
                glyphs,
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
                    glyphs,
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

            stage_manager.draw(transform, g);
            for player in players {
                player.draw(transform, g);
            }
        });
    }

    pub fn update(&mut self, dt: f64) {
        if self.loading_progress_value <= 1.0 {
            self.loading_progress_value += dt;
        } else if self.state == GameState::Loading {
            self.state = GameState::Run;
        }
        self.loader.set_value(self.loading_progress_value);
        self.fps = self.timer.tick();
        if self.state == GameState::Run {
            self.stage_manager.update(dt);

            for player in self.players.iter_mut() {
                player.update(dt);
                player.respawn_player_if_overflow(self.camera.viewport_size.height + 100.0);

                self.stage_manager.collide_with(player);
            }

            if let Some(player) = self.players.get(self.current_player) {
                self.camera.follow_player(player);
            }

            if let Some(stage) = self.stage_manager.get_current_stage() {
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
