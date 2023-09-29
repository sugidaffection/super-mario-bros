use std::{cell::RefCell, collections::HashMap, rc::Rc};

use find_folder::Search;
use piston_window::{Filter, Flip, G2dTexture, G2dTextureContext, Texture, TextureSettings};

pub struct TextureManager {
    context: Rc<RefCell<G2dTextureContext>>,
    textures: HashMap<&'static str, Rc<G2dTexture>>,
}

impl TextureManager {
    pub fn new(context: Rc<RefCell<G2dTextureContext>>) -> Self {
        Self {
            context,
            textures: HashMap::new(),
        }
    }

    pub fn load_texture(
        &mut self,
        name: &'static str,
        path_str: &'static str,
    ) -> Result<&mut Self, String> {
        let assets = Search::Parents(1)
            .for_folder("assets")
            .map_err(|e| e.to_string())?;
        let path = assets.join(path_str);

        let mut texture_settings = TextureSettings::new();
        texture_settings.set_mag(Filter::Nearest);
        let texture = Texture::from_path(
            &mut self.context.borrow_mut(),
            path,
            Flip::None,
            &texture_settings,
        )
        .map_err(|e| format!("Error : {} {}", e.to_string(), path_str))?;
        let texture_rc = Rc::new(texture);
        self.textures.insert(name, texture_rc.clone());

        Ok(self)
    }

    pub fn get_texture(&self, name: &'static str) -> Result<Rc<G2dTexture>, String> {
        self.textures
            .get(name)
            .map(|texture| texture.clone())
            .ok_or(format!("Failed to get texture {}", name))
    }
}
