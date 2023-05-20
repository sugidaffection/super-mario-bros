use cgmath::Vector2;
use find_folder::Search;
use fps_counter::FPSCounter;
use graphics::Transformed;
use piston_window::texture::Filter;
use piston_window::{
    clear, Button, ButtonEvent, EventLoop, Flip, G2dTexture, G2dTextureContext, ImageSize,
    MouseCursorEvent, PistonWindow, RenderEvent, Size, Texture, TextureSettings, UpdateEvent,
    WindowSettings,
};
use serde_json::{from_reader, Value};
use std::fs::File;
use std::path::PathBuf;
use std::rc::Rc;

mod libs {
    pub mod animations;
    pub mod collider;
    pub mod controller;
    pub mod object;
    pub mod physics;
    pub mod player;
    pub mod sprites_manager;
    pub mod spritesheet;
    pub mod transform;
}

use libs::object::{Object, Object2D};
use libs::player::Player;
use libs::sprites_manager::SpriteManager;
use libs::spritesheet::{SpriteSheet, SpriteSheetConfig};
use libs::transform::{Rect, Trans};

fn load_texture(mut context: &mut G2dTextureContext, p: &PathBuf) -> Rc<G2dTexture> {
    let mut texture_settings = TextureSettings::new();
    texture_settings.set_mag(Filter::Nearest);
    let texture = Texture::from_path(&mut context, p, Flip::None, &texture_settings).unwrap();
    Rc::new(texture)
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum Music {
    World1_1,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum Sound {}

fn main() {
    let width = 352.0;
    let height = 224.0;
    let aspect_ratio = height / width;
    let window_size: Size = Size::from([width, height]);
    let mut timer = FPSCounter::default();

    let mut window: PistonWindow = WindowSettings::new("Super Goomba Bros", window_size)
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));
    window.set_lazy(false);
    window.set_max_fps(30);
    window.set_ups(30);

    let assets = Search::Parents(1).for_folder("assets").unwrap();

    let mut texture_context = window.create_texture_context();

    let map_path = assets.join("world_1.png");
    let map_texture = SpriteManager::<G2dTexture>::load_texture(&mut texture_context, &map_path);
    let map_size = map_texture.get_size();
    let tilemap_spritesheet = SpriteSheet::new(map_texture);
    let mut map_pos_x = 0.0;
    let [map_width, map_height] = [map_size.0 as f64, map_size.1 as f64];
    let map_scale = map_height / height;

    let player_path = assets.join("player.png");
    let player_texture = load_texture(&mut texture_context, &player_path);
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

    let json_objects: Value =
        from_reader(File::open(assets.join("world_1_1.tmj")).unwrap()).unwrap();
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
        let x = obj.as_object().unwrap().get("x").unwrap().as_f64().unwrap();
        let y = obj.as_object().unwrap().get("y").unwrap().as_f64().unwrap();
        let w = obj
            .as_object()
            .unwrap()
            .get("width")
            .unwrap()
            .as_f64()
            .unwrap();
        let h = obj
            .as_object()
            .unwrap()
            .get("height")
            .unwrap()
            .as_f64()
            .unwrap();
        let mut o = Object::<G2dTexture>::new();
        o.set_size(w, h);
        o.set_position(x, y);
        objects.push(o);
    }
    music::start::<Music, Sound, _>(16, || {
        music::bind_music_file(Music::World1_1, "./assets/main_theme.mp3");
        music::set_volume(music::MAX_VOLUME);
        music::play_music(&Music::World1_1, music::Repeat::Forever);

        while let Some(e) = window.next() {
            if let Some(r) = e.render_args() {
                window.draw_2d(&e, |c, g, _d| {
                    clear([0.0, 0.0, 0.0, 0.5], g);
                    tilemap_spritesheet.draw(c.transform.trans(-map_pos_x, 0.0), g);
                    // map_img.draw(&*map_rc, &draw_state::DrawState::default(), c.transform, g);
                    // sm.get_first("map")
                    //     .unwrap()
                    //     .draw(c.transform.trans(-map_pos.x, -map_pos.y), g);
                    for object in objects.iter_mut() {
                        let obj = object.get_transform();
                        if obj.x() < window_size.width && obj.xw() >= 0.0 {
                            object.draw(c.transform, g);
                        }
                    }
                    player.draw(c.transform, g);
                });
            }

            if let Some(pos) = e.mouse_cursor_args() {
                // player.set_position(pos[0], pos[1]);
            }

            if let Some(u) = e.update_args() {
                if map_pos_x < 0.0 {
                    map_pos_x = 0.0;
                }

                if map_pos_x > map_width * map_scale - window_size.width {
                    map_pos_x = map_width * map_scale - window_size.width;
                }

                player.update(u.dt);

                for object in objects.iter() {
                    player.collide_with(object);
                }

                player.set_inside_window(window_size);
            }

            if let Some(args) = e.button_args() {
                if let Button::Keyboard(key) = args.button {
                    player.update_input(key, args.state);
                }
            }
        }
    });
}
