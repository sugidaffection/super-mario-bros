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
        path: &'static str,
    ) -> Result<Rc<G2dTexture>, String> {
        let assets = Search::Parents(1)
            .for_folder("assets")
            .map_err(|e| e.to_string())?;
        let path = assets.join(path);

        let mut texture_settings = TextureSettings::new();
        texture_settings.set_mag(Filter::Nearest);
        let texture = Texture::from_path(
            &mut self.context.borrow_mut(),
            path,
            Flip::None,
            &texture_settings,
        )
        .map_err(|e| e.to_string())?;
        let texture_rc = Rc::new(texture);
        self.textures.insert(name, texture_rc.clone());

        Ok(texture_rc)
    }
}
