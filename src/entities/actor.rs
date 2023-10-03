use super::entity::Entity;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Actor {
    entity: Entity,
    name: String,
}

impl Actor {
    pub fn new(entity: Entity, name: String) -> Self {
        Self { entity, name }
    }
}

#[derive(Debug)]
pub struct ActorManager {
    pub nodes: HashMap<Entity, Actor>,
}

impl ActorManager {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node_actor: Actor) -> Entity {
        let node_entity = node_actor.entity;
        self.nodes.insert(node_entity, node_actor);
        node_entity
    }

    pub fn remove_node(&mut self, target: Entity) -> Result<(), String> {
        if self.nodes.remove(&target).is_some() {
            return Ok(());
        }

        Err(format!("Entity {:?} not found", target))
    }
}
