use serde_json::{from_reader, Map, Value};

pub struct SceneLoader {
    scenes: Vec<Scene>,
    curr_idx: usize,
}

pub struct Scene {
    entities: Vec<Rect>,
}

pub impl Scene {
    fn create() {
        Scene { entities: vec![] }
    }
}
