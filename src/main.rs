use cgmath::Vector2;
use find_folder::Search;
use piston_window::*;
use sprite::*;
use std::rc::Rc;

mod player;
use player::*;

mod libs {
    pub mod controller;
    pub mod object;
    pub mod physics;
    pub mod transform;
}

use libs::object::Object;
use libs::controller::Controller;

fn main() {
    let window_size: Size = Size::from([640, 480]);

    let mut window: PistonWindow = WindowSettings::new("Super Mario Bros", window_size)
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));
    window.set_lazy(false);
    window.set_max_fps(60);

    let assets = Search::Parents(1).for_folder("assets").unwrap();

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
    let mut map = Sprite::from_texture_rect(map_rc.clone(), [0.0, 0.0, map_width, map_height]);
    map.set_scale(map_scale, map_scale);
    map.set_position(map_width * map_scale / 2.0, map_height * map_scale / 2.0);

    let player_texture = Texture::from_path(
        &mut window.create_texture_context(),
        &assets.join("players.png"),
        Flip::None,
        &TextureSettings::new(),
    )
    .unwrap();
    let player_rc = Rc::new(player_texture);


    let mut sprite = Sprite::from_texture(player_rc.clone());
    sprite.set_position(16.0, 16.0);

    let mut animations = vec![];

    let mut player = Player::new();
    let mut controller = Controller::new();

    for x in (80..256).step_by(17) {
        let rect = [(x as f64) + 0.425, 34.45, 15.25, 15.25];
        animations.push(rect);
    }
    
    let player_scale = map_height * map_scale / 16.0 / 16.0;
    player.set_scale(player_scale, player_scale);
    player.set_sprite(sprite);
    player.push_animation("idle", animations[0]);
    player.push_animation("jump", animations[5]);
    player.append_animation("walk", animations[0..4].to_vec());

    let mut map_pos = Vector2::from([0.0,0.0]);

    let grounds_pos = [[0.0, 1103.0], [1135.0, 240.0], [1423.0, 1024.0], [2479.0, 912.0]];
    let ground_y = map_height * map_scale - 52.0;

    let mut objects = Vec::new();
    for pos in grounds_pos.iter() {
        let mut ground = Object::new();
        ground.set_transparent(false);
        ground.set_border(true);
        ground.set_position(pos[0] * map_scale, ground_y);
        ground.set_size(pos[1] * map_scale, 52.0);
        objects.push(ground);
    }
    
    let pipes_pos = [
        [447.0, 32.0, 32.0],
        [607.0, 32.0, 48.0],
        [735.0, 32.0, 64.0],
        [912.0, 32.0, 64.0],
        [2608.0, 32.0, 32.0],
        [2864.0, 32.0, 32.0]
    ];

    for pos in pipes_pos.iter() {
        let mut pipe = Object::new();
        pipe.set_transparent(false);
        pipe.set_border(true);
        pipe.set_position(pos[0] * map_scale, ground_y - pos[2] * map_scale);
        pipe.set_size(pos[1] * map_scale, pos[2] * map_scale);
        objects.push(pipe);
    }

    let bricks_pos = [
        [2144.0, 64.0, 16.0],
        [2160.0, 48.0, 32.0],
        [2176.0, 32.0, 48.0],
        [2192.0, 16.0, 64.0]
    ];

    for pos in bricks_pos.iter() {
        let mut brick = Object::new();
        brick.set_transparent(false);
        brick.set_border(true);
        brick.set_position(pos[0] * map_scale, ground_y - pos[2] * map_scale);
        brick.set_size(pos[1] * map_scale, pos[2] * map_scale);
        objects.push(brick);

        let mut brick = Object::new();
        brick.set_transparent(false);
        brick.set_border(true);
        brick.set_position((pos[0] + pos[1] + 32.0) * map_scale, ground_y - pos[2] * map_scale);
        brick.set_size(pos[1] * map_scale, pos[2] * map_scale);
        objects.push(brick);

        let mut brick = Object::new();
        brick.set_transparent(false);
        brick.set_border(true);
        brick.set_position((pos[0] + pos[1] + 160.0 + (pos[2] - 16.0)) * map_scale, ground_y - pos[2] * map_scale);
        brick.set_size((pos[1] + 16.0) * map_scale, pos[2] * map_scale);
        objects.push(brick);

        let mut brick = Object::new();
        brick.set_transparent(false);
        brick.set_border(true);
        brick.set_position((pos[0] + pos[1] + 272.0) * map_scale, ground_y - pos[2] * map_scale);
        brick.set_size(pos[1] * map_scale, pos[2] * map_scale);
        objects.push(brick);

    }

    let bricks2_pos = [
        [2896.0, 144.0, 16.0],
        [2912.0, 128.0, 32.0],
        [2928.0, 112.0, 48.0],
        [2944.0, 96.0, 64.0],
        [2960.0, 80.0, 80.0],
        [2976.0, 64.0, 96.0],
        [2992.0, 48.0, 112.0],
        [3008.0, 32.0, 128.0],
    ];

    for pos in bricks2_pos.iter() {
        let mut brick = Object::new();
        brick.set_transparent(false);
        brick.set_border(true);
        brick.set_position(pos[0] * map_scale, ground_y - pos[2] * map_scale);
        brick.set_size(pos[1] * map_scale + 16.0, pos[2] * map_scale);
        objects.push(brick);
    }

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _d| {
            clear([0.0, 0.0, 0.0, 0.5], g);
            map.draw(c.transform.trans(-map_pos.x, -map_pos.y), g);
            for object in objects.iter() {
                object.draw(c.transform, g);
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
