use crate::entities::{actor::ActorManager, transform::TransformManager};

#[derive(Debug)]
pub struct Scene {
    pub actor_manager: ActorManager,
    pub transform_manager: TransformManager,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            actor_manager: ActorManager::new(),
            transform_manager: TransformManager::new(),
        }
    }
}
