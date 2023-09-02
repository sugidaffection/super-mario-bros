use std::{cell::RefCell, collections::HashMap, fs::File, rc::Rc};

use cgmath::Vector2;
use find_folder::Search;
use graphics::ImageSize;
use piston_window::{G2dTexture, Size};
use serde_json::{from_reader, Value};
use sprite::Sprite;

use super::{
    bricks::{Brick, BrickType},
    camera::Camera,
    collider::Collision,
    core::{Drawable, Object2D, Updatable},
    enemies::Enemy,
    object::Object,
    player::Player,
    sprite_sheet::{SpriteSheet, SpriteSheetConfig},
    textures::TextureManager,
    transform::Trans,
};

pub struct StageMap {
    map: Sprite<G2dTexture>,
    size: Size,
}

impl StageMap {
    pub fn new(texture: Rc<G2dTexture>) -> Self {
        let size = texture.get_size().into();
        let mut map = Sprite::from_texture(texture);
        map.set_anchor(0.0, 0.0);
        Self { map, size }
    }

    pub fn get_size(&self) -> Size {
        self.size
    }

    pub fn set_src_rect(&mut self, src_rect: [f64; 4]) {
        self.map.set_src_rect(src_rect);
    }
}

impl Drawable for StageMap {
    fn draw(&mut self, t: graphics::types::Matrix2d, b: &mut piston_window::G2d) {
        self.map.draw(t, b)
    }
}

pub struct StageObject {
    pub name: String,
    pub pos: Vector2<f64>,
    pub size: Size,
}

pub struct Stage {
    pub map: StageMap,
    pub objects: Vec<StageObject>,
}

impl Stage {
    pub fn new(map_texture: Rc<G2dTexture>) -> Self {
        Self {
            map: StageMap::new(map_texture),
            objects: Vec::new(),
        }
    }

    pub fn load_objects_from_file(&mut self, path: &'static str) -> Result<(), String> {
        let assets = Search::Parents(1).for_folder("assets").unwrap();
        let path = assets.join(path);
        let file = File::open(path).unwrap();
        let json_objects: Value = from_reader(file).unwrap();
        let layers: &Vec<Value> = json_objects
            .as_object()
            .ok_or("Cannot Load Object")?
            .get("layers")
            .ok_or("Cannot read layers")?
            .as_array()
            .ok_or("Layers is not array")?;

        let objects: Vec<Value> = layers
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

        for obj in objects {
            let json_obj = obj.as_object().unwrap();
            let x = json_obj.get("x").unwrap().as_f64().unwrap();
            let y = json_obj.get("y").unwrap().as_f64().unwrap();
            let w = json_obj.get("width").unwrap().as_f64().unwrap();
            let h = json_obj.get("height").unwrap().as_f64().unwrap();
            let name = json_obj.get("name").unwrap().as_str().unwrap().to_string();

            self.objects.push(StageObject {
                name,
                pos: Vector2::from([x, y]),
                size: Size::from([w, h]),
            });
        }

        Ok(())
    }
}

impl Drawable for Stage {
    fn draw(&mut self, t: graphics::types::Matrix2d, b: &mut piston_window::G2d) {
        self.map.draw(t, b);
    }
}

pub struct StageManager {
    texture_manager: Rc<RefCell<TextureManager>>,
    stages: HashMap<&'static str, Stage>,
    current_stage: &'static str,
    pub enemies: Vec<Enemy>,
    pub objects: Vec<Object>,
    pub bricks: Vec<Brick>,
    players: Vec<Player>,
}

impl StageManager {
    pub fn new(texture_manager: Rc<RefCell<TextureManager>>) -> Self {
        Self {
            texture_manager,
            stages: HashMap::new(),
            current_stage: "",
            enemies: Vec::new(),
            objects: Vec::new(),
            bricks: Vec::new(),
            players: Vec::new(),
        }
    }

    pub fn register_stage(&mut self, name: &'static str, stage: Stage) {
        self.stages.insert(name, stage);
    }

    pub fn set_current_stage(&mut self, name: &'static str) {
        self.current_stage = name;
    }

    pub fn get_current_stage(&mut self) -> Option<&mut Stage> {
        self.stages.get_mut(self.current_stage)
    }

    pub fn collide_with(&mut self, player: &mut Player) {
        for enemy in self.enemies.iter_mut() {
            for object in self.objects.iter_mut() {
                player.collide_with(object.get_transform());
                enemy.collide_with(object.get_transform());
            }

            for brick in self.bricks.iter_mut() {
                player.collide_with(brick.get_transform());
                enemy.collide_with(brick.get_transform());
            }

            player.collide_with(enemy.get_transform());
        }
    }

    pub fn load_enemies(&mut self) -> Result<(), String> {
        let enemies_texture = self.texture_manager.borrow_mut().get_texture("enemies")?;
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
        self.enemies.push(goomba);
        Ok(())
    }

    pub fn load_objects(&mut self) -> Result<(), String> {
        let texture = self.texture_manager.borrow().get_texture("tileset")?;
        let mut tileset = SpriteSheet::new(
            texture,
            SpriteSheetConfig {
                offset: Vector2::from([0.0, 0.0]),
                spacing: Vector2::from([0.0, 0.0]),
                grid: [1, 2],
                sprite_size: Size::from([16.0, 16.0]),
                scale: Vector2::new(1.0, 1.0),
            },
        );
        let brick_sprite = Rc::new(RefCell::new(tileset.clone_sprite_from(0, 0)));
        let coin_sprite = Rc::new(RefCell::new(tileset.clone_sprite_from(0, 1)));

        let mut bricks = Vec::new();
        let mut objects = Vec::new();

        if let Some(stage) = self.get_current_stage() {
            for object in stage.objects.iter_mut() {
                if object.name.trim() == "brick".trim() {
                    let mut o = Brick::new(BrickType::Block, Rc::clone(&brick_sprite));
                    o.get_transform_mut()
                        .set_size(object.size.width, object.size.height);
                    o.get_transform_mut()
                        .set_position(object.pos.x, object.pos.y);
                    bricks.push(o);
                } else if object.name.trim() == "coin".trim() {
                    let mut o = Brick::new(BrickType::Coin, Rc::clone(&coin_sprite));
                    o.get_transform_mut()
                        .set_size(object.size.width, object.size.height);
                    o.get_transform_mut()
                        .set_position(object.pos.x, object.pos.y);
                    bricks.push(o);
                } else {
                    let mut o = Object::new("ground".to_string());
                    o.get_transform_mut()
                        .set_size(object.size.width, object.size.height);
                    o.get_transform_mut()
                        .set_position(object.pos.x, object.pos.y);
                    objects.push(o);
                }
            }
        }

        self.bricks.extend(bricks);
        self.objects.extend(objects);

        Ok(())
    }
}

impl Drawable for StageManager {
    fn draw(&mut self, t: graphics::types::Matrix2d, b: &mut piston_window::G2d) {
        for brick in self.bricks.iter_mut() {
            brick.draw(t, b);
        }

        for enemies in self.enemies.iter_mut() {
            enemies.draw(t, b);
        }
    }
}

impl Updatable for StageManager {
    fn update(&mut self, dt: f64) {
        for enemies in self.enemies.iter_mut() {
            enemies.update(dt);
        }
    }
}
