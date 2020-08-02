use cgmath::Vector2;
use find_folder::Search;
use piston_window::*;
use sprite::*;
use std::collections::HashMap;
use std::rc::Rc;

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

    let ground_pos = [
        [0.0, 200.0, 1104.0, 24.0],
        [1136.0, 200.0, 240.0, 24.0],
        [1424.0, 200.0, 1024.0, 24.0],
        [2480.0, 200.0, 912.0, 24.0]
    ];

    for pos in ground_pos.iter() {
        let mut ground = Object::new();
        ground.set_position(pos[0] * map_scale, pos[1] * map_scale);
        ground.set_size(pos[2] * map_scale, pos[3] * map_scale);
        objects.push(ground);
    }

    let pipe_pos = [
        [448.0, 168.0, 32.0, 32.0],
        [608.0, 152.0, 32.0, 48.0],
        [736.0, 136.0, 32.0, 64.0],
        [912.0, 136.0, 32.0, 64.0],
        [2608.0, 168.0, 32.0, 32.0],
        [2864.0, 168.0, 32.0, 32.0]
    ];

    for pos in pipe_pos.iter() {
        let mut pipe = Object::new();
        pipe.set_position(pos[0] * map_scale, pos[1] * map_scale);
        pipe.set_size(pos[2] * map_scale, pos[3] * map_scale);
        objects.push(pipe);
    }

    let brick_pos = [
        [2144.0, 184.0, 64.0, 16.0],
        [2160.0, 168.0, 48.0, 16.0],
        [2176.0, 152.0, 32.0, 16.0],
        [2192.0, 136.0, 16.0, 16.0],

        [2240.0, 136.0, 16.0, 16.0],
        [2240.0, 152.0, 32.0, 16.0],
        [2240.0, 168.0, 48.0, 16.0],
        [2240.0, 184.0, 64.0, 16.0],

        [2368.0, 184.0, 80.0, 16.0],
        [2384.0, 168.0, 64.0, 16.0],
        [2400.0, 152.0, 48.0, 16.0],
        [2416.0, 136.0, 32.0, 16.0],

        [2480.0, 136.0, 16.0, 16.0],
        [2480.0, 152.0, 32.0, 16.0],
        [2480.0, 168.0, 48.0, 16.0],
        [2480.0, 184.0, 64.0, 16.0],

        [2896.0, 184.0, 144.0, 16.0],
        [2912.0, 168.0, 128.0, 16.0],
        [2928.0, 152.0, 112.0, 16.0],
        [2944.0, 136.0, 96.0, 16.0],
        [2960.0, 120.0, 80.0, 16.0],
        [2976.0, 104.0, 64.0, 16.0],
        [2992.0, 88.0, 48.0, 16.0],
        [3008.0, 72.0, 32.0, 16.0],

        [3168.0, 184.0, 16.0, 16.0]
    ];


    for pos in brick_pos.iter() {
        let mut brick = Object::new();
        brick.set_position(pos[0] * map_scale, pos[1] * map_scale);
        brick.set_size(pos[2] * map_scale, pos[3] * map_scale);
        objects.push(brick);
    }

    let tile_qm = [
        [256.0, 136.0, 16.0, 16.0],
        [336.0, 136.0, 16.0, 16.0],
        [352.0, 72.0, 16.0, 16.0],
        [368.0, 136.0, 16.0, 16.0],
        [1248.0, 136.0, 16.0, 16.0],
        [1504.0, 136.0, 16.0, 16.0],
        [1696.0, 136.0, 16.0, 16.0],
        [1744.0, 136.0, 16.0, 16.0],
        [1744.0, 72.0, 16.0, 16.0],
        [1792.0, 136.0, 16.0, 16.0],
        [2064.0, 72.0, 16.0, 16.0],
        [2080.0, 72.0, 16.0, 16.0],
        [2720.0, 136.0, 16.0, 16.0],
    ];

    let tile_b = [
        [320.0, 136.0, 16.0, 16.0],
        [352.0, 136.0, 16.0, 16.0],
        [384.0, 136.0, 16.0, 16.0],
        [1232.0, 136.0, 16.0, 16.0],
        [1264.0, 136.0, 16.0, 16.0],
        [1280.0, 72.0, 16.0, 16.0],
        [1296.0, 72.0, 16.0, 16.0],
        [1312.0, 72.0, 16.0, 16.0],
        [1328.0, 72.0, 16.0, 16.0],
        [1344.0, 72.0, 16.0, 16.0],
        [1360.0, 72.0, 16.0, 16.0],
        [1376.0, 72.0, 16.0, 16.0],
        [1392.0, 72.0, 16.0, 16.0],
        [1456.0, 72.0, 16.0, 16.0],
        [1472.0, 72.0, 16.0, 16.0],
        [1488.0, 72.0, 16.0, 16.0],
        [1504.0, 136.0, 16.0, 16.0],
        [1600.0, 136.0, 16.0, 16.0],
        [1616.0, 136.0, 16.0, 16.0],
        [1888.0, 136.0, 16.0, 16.0],
        [1936.0, 72.0, 16.0, 16.0],
        [1936.0, 72.0, 16.0, 16.0],
        [1952.0, 72.0, 16.0, 16.0],
        [1968.0, 72.0, 16.0, 16.0],
        [2048.0, 72.0, 16.0, 16.0],
        [2096.0, 72.0, 16.0, 16.0],
        [2064.0, 136.0, 16.0, 16.0],
        [2080.0, 136.0, 16.0, 16.0],
        [2688.0, 136.0, 16.0, 16.0],
        [2704.0, 136.0, 16.0, 16.0],
        [2736.0, 136.0, 16.0, 16.0],
    ];

    let tile_texture = Texture::from_path(
        &mut window.create_texture_context(),
        &assets.join("tileset.png"),
        Flip::None,
        &TextureSettings::new(),
    )
    .unwrap();

    let tile_rc = Rc::new(tile_texture);
    let tile_scale = map_scale;
    for pos in tile_qm.iter() {
        let mut brick = Object::new();
        brick.set_sprite(Sprite::from_texture_rect(
            tile_rc.clone(),
            [384.0, 0.0, 16.0, 16.0],
        ));
        brick.set_scale(tile_scale, tile_scale);
        brick.set_position(pos[0] * tile_scale, pos[1] * tile_scale);
        brick.set_size(pos[2] * tile_scale, pos[3] * tile_scale);
        objects.push(brick);
    }

    for pos in tile_b.iter() {
        let mut brick = Object::new();
        brick.set_sprite(Sprite::from_texture_rect(
            tile_rc.clone(),
            [16.0, 0.0, 16.0, 16.0],
        ));
        brick.set_scale(tile_scale, tile_scale);
        brick.set_position(pos[0] * tile_scale, pos[1] * tile_scale);
        brick.set_size(pos[2] * tile_scale, pos[3] * tile_scale);
        objects.push(brick);
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
