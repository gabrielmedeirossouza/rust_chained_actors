use super::entity::Entity;
use crate::app_core::math::vector2::Vector2;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Transform {
    entity: Entity,
    parent: Option<Entity>,
    children: HashSet<Entity>,
    local_position: Vector2,
    world_position: Vector2,
}

impl Transform {
    pub fn new(entity: Entity, position: Vector2) -> Self {
        Self {
            entity,
            parent: None,
            children: HashSet::new(),
            local_position: position,
            world_position: position,
        }
    }
}

#[derive(Debug)]
pub struct TransformManager {
    nodes: HashMap<Entity, Transform>,
}

impl TransformManager {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node_transform: Transform) -> Entity {
        let node_entity = node_transform.entity;
        self.nodes.insert(node_entity, node_transform);
        node_entity
    }

    pub fn remove_node(&mut self, target: Entity) -> Result<(), String> {
        if self.nodes.remove(&target).is_some() {
            return Ok(());
        }

        Err(format!("Entity {:?} not found", target))
    }

    pub fn get_nodes(&self) -> &HashMap<Entity, Transform> {
        &self.nodes
    }

    pub fn attach_parent(&mut self, target: Entity, parent: Entity) -> Result<(), String> {
        if let Some(node) = self.nodes.get_mut(&target) {
            node.parent = Some(parent);

            let child_local_position = self.get_local_position(target).unwrap();
            let parent_local_position = self.get_local_position(parent).unwrap();

            self.set_local_position(target, child_local_position - parent_local_position)
                .unwrap();

            return Ok(());
        }

        Err(format!("Entity {:?} not found", target))
    }

    pub fn detach_parent(&mut self, target: Entity) -> Result<(), String> {
        if let Some(node) = self.nodes.get_mut(&target) {
            node.parent = None;
            return Ok(());
        }

        Err(format!("Entity {:?} not found", target))
    }

    pub fn get_parent(&self, target: Entity) -> Result<Option<Entity>, String> {
        if let Some(node) = self.nodes.get(&target) {
            return Ok(node.parent);
        }

        Err(format!("Entity {:?} not found", target))
    }

    pub fn attach_child(&mut self, target: Entity, child: Entity) -> Result<(), String> {
        if let Some(node) = self.nodes.get_mut(&target) {
            node.children.insert(child);
            return Ok(());
        }

        Err(format!("Entity {:?} not found", target))
    }

    pub fn detach_child(&mut self, target: Entity, child: Entity) -> Result<(), String> {
        if let Some(node) = self.nodes.get_mut(&target) {
            node.children.remove(&child);
            return Ok(());
        }

        Err(format!("Entity {:?} not found", target))
    }

    pub fn get_children(&self, target: Entity) -> Result<HashSet<Entity>, String> {
        if let Some(node) = self.nodes.get(&target) {
            return Ok(node.children.clone());
        }

        Err(format!("Entity {:?} not found", target))
    }

    pub fn set_local_position(&mut self, target: Entity, position: Vector2) -> Result<(), String> {
        if let Some(node) = self.nodes.get_mut(&target) {
            node.local_position = position;
            return Ok(());
        }

        Err(format!("Entity {:?} not found", target))
    }

    pub fn get_local_position(&self, target: Entity) -> Result<Vector2, String> {
        if let Some(node) = self.nodes.get(&target) {
            return Ok(node.local_position);
        }

        Err(format!("Entity {:?} not found", target))
    }

    pub fn set_world_position(&mut self, target: Entity, position: Vector2) -> Result<(), String> {
        if let Some(node) = self.nodes.get_mut(&target) {
            node.world_position = position;
            return Ok(());
        }

        Err(format!("Entity {:?} not found", target))
    }

    pub fn get_world_position(&self, target: Entity) -> Result<Vector2, String> {
        if let Some(node) = self.nodes.get(&target) {
            return Ok(node.world_position);
        }

        Err(format!("Entity {:?} not found", target))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_initial_transform() {
        let transform = Transform::new(Entity::new(), Vector2::zero());

        assert_eq!(transform.parent, None);
        assert_eq!(transform.children, HashSet::new());
        assert_eq!(transform.local_position.x, 0.0);
        assert_eq!(transform.local_position.y, 0.0);
        assert_eq!(transform.world_position.x, 0.0);
        assert_eq!(transform.world_position.y, 0.0);
    }

    #[test]
    fn should_attach_correct_parent() {
        let mut manager = TransformManager::new();
        let parent_transform = Transform::new(Entity::new(), Vector2::new(20.0, 30.0));
        let child_transform = Transform::new(Entity::new(), Vector2::new(10.0, 15.0));

        let parent = manager.add_node(parent_transform);
        let child = manager.add_node(child_transform);

        manager.attach_parent(child, parent).unwrap();

        assert_eq!(manager.get_local_position(child).unwrap().x, -10.0);
        assert_eq!(manager.get_local_position(child).unwrap().y, -15.0);
        assert_eq!(manager.get_world_position(child).unwrap().x, 10.0);
        assert_eq!(manager.get_world_position(child).unwrap().y, 15.0);
        assert_eq!(manager.get_local_position(parent).unwrap().x, 20.0);
        assert_eq!(manager.get_local_position(parent).unwrap().y, 30.0);
        assert_eq!(manager.get_world_position(parent).unwrap().x, 20.0);
        assert_eq!(manager.get_world_position(parent).unwrap().y, 30.0);
    }

    #[test]
    fn should_attach_correct_child() {
        // let mut parent = Cell::new(Transform::new(Vector2::new(10.0, 15.0)));
        // let mut child_1 = Transform::new(Vector2::new(20.0, 30.0));
        // let mut child_2 = Transform::new(Vector2::new(40.0, 50.0));

        // parent.get_mut().attach_child(&mut child_1);
        // parent.get_mut().attach_child(&mut child_2);
        // assert_eq!(parent.get_mut().get_local_position().x, 10.0);
        // assert_eq!(parent.get_mut().get_local_position().y, 15.0);
        // assert_eq!(parent.get_mut().get_world_position().x, 10.0);
        // // assert_eq!(parent.get_world_position().y, 15.0);
        // assert_eq!(child_1.get_local_position().x, 10.0);
        // assert_eq!(child_1.get_local_position().y, 15.0);
        // assert_eq!(child_1.get_world_position().x, 20.0);
        // assert_eq!(child_1.get_world_position().y, 30.0);
        // assert_eq!(child_2.get_local_position().x, 30.0);
        // assert_eq!(child_2.get_local_position().y, 35.0);
        // assert_eq!(child_2.get_world_position().x, 40.0);
        // assert_eq!(child_2.get_world_position().y, 50.0);
    }

    #[test]
    fn should_correct_transform_child_on_parent_move() {
        // let mut parent = Transform::new(Vector2::new(10.0, 15.0));
        // let mut child_1 = Transform::new(Vector2::new(20.0, 30.0));
        // let mut child_2 = Transform::new(Vector2::new(40.0, 50.0));

        // parent.attach_child(&mut child_1);
        // parent.attach_child(&mut child_2);

        // parent.set_local_position(Vector2::new(25.0, -5.0));

        // assert_eq!(child_1.get_local_position().x, 10.0);
        // assert_eq!(child_1.get_local_position().y, 15.0);
        // assert_eq!(child_1.get_world_position().x, 35.0);
        // assert_eq!(child_1.get_world_position().y, 10.0);
        // assert_eq!(child_2.get_local_position().x, 30.0);
        // assert_eq!(child_2.get_local_position().y, 35.0);
        // assert_eq!(child_2.get_world_position().x, 55.0);
        // assert_eq!(child_2.get_world_position().y, 30.0);
    }
}
