extern crate piston_window;
extern crate sprite;
extern crate find_folder;


use piston_window::*;
use sprite::*;
use find_folder::Search;
use std::rc::Rc;


const FRICTION: f64 = -0.12;
const GRAVITY: f64 = 0.3;
const PLAYER_ACC: f64 = 0.01;
const PLAYER_SPEED: f64 = 0.3;
const PLAYER_MAX_SPEED: f64 = 1.0;
const PLAYER_JUMP: f64 = 5.0;

enum PlayerState {
    Walk,
    Run,
    Jump,
    Fall,
    Idle
}

enum PlayerDirection {
    Left,
    Right
}


fn main() {
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", (640, 448))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

    let assets = Search::Parents(1).for_folder("assets").unwrap();

    let map_texture = Texture::from_path(
        &mut window.create_texture_context(), 
        &assets.join("world_1-1.png"),
        Flip::None,
        &TextureSettings::new()
    ).unwrap();
    let map_width = map_texture.get_width() as f64;
    let map_rc = Rc::new(map_texture);
    
    let mut map = Sprite::from_texture_rect(map_rc.clone(), [0.0, 0.0, map_width, 224.0]);
    map.set_scale(2.0, 2.0);
    map.set_position(map_width, 224.0);

    let texture = Rc::new(Texture::from_path(
        &mut window.create_texture_context(), 
        &assets.join("players.png"),
        Flip::None,
        &TextureSettings::new()
    ).unwrap());

    let mut sprites = vec![];
    let mut idx = 0.0;

    for x in (80..256).step_by(17) {
        let mut sprite = Sprite::from_texture_rect(texture.clone(), [(x as f64) + 0.425, 34.45, 15.25, 15.25]);
        sprite.set_scale(2.0, 2.0);
        sprite.set_position(15.25, 15.25);
        sprites.push(sprite);
    }

    

    let mut player_state = PlayerState::Idle;
    let mut player_dir = PlayerDirection::Right;
    let mut player_pos = vec![0.0, 0.0];
    let mut player_flip = false;
    let mut player_can_jump = false;

    let mut acc = vec![0.0, 0.0];
    let mut vel = vec![0.0, 0.0];
    let mut map_pos = vec![0.0, 0.0];

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _d| {
            clear([0.5, 1.0, 0.5, 1.0], g);
            map.draw(c.transform.trans(map_pos[0], map_pos[1]), g);
            sprites[idx as usize].set_flip_x(player_flip);
            sprites[idx as usize].draw(c.transform.trans(player_pos[0], player_pos[1]), g);
        });

        if let Some(u) = e.update_args() {
            let dt = u.dt * 100.0;

            idx =  (idx as f64 + dt * 0.125) % 4.0;

            // acceleration
            match (&player_state, &player_dir) {
                (PlayerState::Walk, PlayerDirection::Right) => {
                    acc[0] += PLAYER_ACC + dt * PLAYER_SPEED;
                },
                (PlayerState::Walk, PlayerDirection::Left) => {
                    acc[0] -= PLAYER_ACC + dt * PLAYER_SPEED;
                },
                (PlayerState::Run, PlayerDirection::Right) => {
                    acc[0] += PLAYER_ACC + dt * PLAYER_MAX_SPEED;
                },
                (PlayerState::Run, PlayerDirection::Left) => {
                    acc[0] -= PLAYER_ACC + dt * PLAYER_MAX_SPEED;
                },
                (PlayerState::Jump, _) => {
                    if player_can_jump {
                        acc[1] -= PLAYER_JUMP * dt;
                        player_can_jump = false;
                    }

                    idx = 5.0;

                    if vel[1] > 0.0 {
                        player_state = PlayerState::Fall;
                    }
                },
                (PlayerState::Fall, _) => {
                    idx = 0.0;
                },
                (PlayerState::Idle, _) => {
                    idx = 0.0;
                }
                _ => {}
            }

            // update movement
            acc[0] += vel[0] * FRICTION;
            vel[0] += acc[0];
            vel[0] = (vel[0] * 1000.0).round() / 1000.0;

            acc[1] += GRAVITY * dt;
            vel[1] += acc[1];
            vel[1] = (vel[1] * 1000.0).round() / 1000.0;

            player_pos[1] += vel[1];


            if player_pos[1] >= window.size().height - (16.0 * 5.0) {
                acc[1] = 0.0;
                vel[1] = 0.0;
                player_pos[1] = window.size().height - (16.0 * 5.0);
                
                player_can_jump = true;
            }


            // add position
            match player_dir {
                PlayerDirection::Right => {
                    player_flip = false;
                    if player_pos[0] > window.size().width / 2.0 - (16.0 * 2.0) && map_pos[0] > (map_width * -2.0 + window.size().width + 16.0) {
                        map_pos[0] -= vel[0] + dt * acc[0];
                    }else if player_pos[0] + 16.0 < window.size().width - 16.0 {
                        player_pos[0] += vel[0] + dt * acc[0];
                    }

                    if vel[0] < 0.0 {
                        idx = 4.0;
                    }
                },
                PlayerDirection::Left => {
                    player_flip = true;

                    if player_pos[0] > 0.0 {
                        player_pos[0] += vel[0] + dt * acc[0];
                    }

                    if vel[0] > 0.0 {
                        idx = 4.0;
                    }
                }
            }

            if map_pos[0] < map_width * -2.0 + window.size().width + 16.0{
                map_pos[0] = map_width * -2.0 + window.size().width + 16.0;
            }

            if player_pos[0] < 0.0 {
                player_pos[0] = 0.0;
            }else if player_pos[0] + 16.0 > window.size().width - 16.0 {
                player_pos[0] = window.size().width - (16.0 * 2.0);
            }

            
            println!("Acceleration x,y : {},{}", acc[0], acc[1]);

            acc[0] *= 0.0;
            acc[1] *= 0.0;

            println!("Velocity x,y: {},{}", vel[0], vel[1]);

        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::Right => {
                    player_state = PlayerState::Walk;
                    player_dir = PlayerDirection::Right;
                },
                Key::Left => {
                    player_state = PlayerState::Walk;
                    player_dir = PlayerDirection::Left;
                },
                Key::Space => {
                    player_state = PlayerState::Jump;
                },
                _ => {}
                
            }
        }

        if let Some(Button::Keyboard(key)) = e.release_args() {
            match key {
                Key::Right => {
                    player_state = PlayerState::Idle;
                },
                Key::Left => {
                    player_state = PlayerState::Idle;
                },
                _ => {}
            }
        }
    }
}