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

    pub fn attach_parent(&mut self, target: Entity, parent: Entity) -> Result<(), String> {
        let parent_local_position = self
            .get_local_position(parent)
            .ok()
            .ok_or(format!("Parent Entity {:?} not found", parent))?;

        let child_transform = self
            .nodes
            .get_mut(&target)
            .ok_or(format!("Child Entity {:?} not found", target))?;

        child_transform.parent = Some(parent);
        child_transform.local_position -= parent_local_position;
        Ok(())
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
            self.attach_parent(child, target).unwrap();
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
            let delta_local_position = position - node.local_position;
            node.local_position = position;
            node.world_position += delta_local_position;

            for child in self.get_children(target).unwrap() {
                let child_world_position = self.get_world_position(child).unwrap();
                self.set_world_position(child, child_world_position + delta_local_position)
                    .unwrap()
            }

            return Ok(());
        }

        Err(format!("Entity {:?} not found", target))
    }

    pub fn get_local_position(&self, target: Entity) -> Result<Vector2, String> {
        self.nodes
            .get(&target)
            .map(|node| node.local_position)
            .ok_or(format!("Entity {:?} not found", target))
    }

    pub fn set_world_position(&mut self, target: Entity, position: Vector2) -> Result<(), String> {
        let transform = self
            .nodes
            .get_mut(&target)
            .ok_or(format!("Transform Entity {:?} not found", target))?;
        let delta_position = position - transform.world_position;

        transform.world_position = position;

        for child in self
            .get_children(target)
            .ok()
            .ok_or(format!("Children Entity {:?} not found", target))?
        {
            let child_world_position = self
                .get_world_position(child)
                .ok()
                .ok_or(format!("Child Entity {:?} not found", child))?;
            self.set_world_position(child, child_world_position + delta_position)
                .unwrap();
        }

        Ok(())
    }

    pub fn get_world_position(&self, target: Entity) -> Result<Vector2, String> {
        self.nodes
            .get(&target)
            .map(|node| node.world_position)
            .ok_or(format!("Entity {:?} not found", target))
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
        let mut manager = TransformManager::new();
        let parent_transform = Transform::new(Entity::new(), Vector2::new(10.0, 15.0));
        let child_transform_1 = Transform::new(Entity::new(), Vector2::new(20.0, 30.0));
        let child_transform_2 = Transform::new(Entity::new(), Vector2::new(40.0, 50.0));

        let parent = manager.add_node(parent_transform);
        let child_1 = manager.add_node(child_transform_1);
        let child_2 = manager.add_node(child_transform_2);

        manager.attach_child(parent, child_1).unwrap();
        manager.attach_child(parent, child_2).unwrap();

        assert_eq!(manager.get_local_position(parent).unwrap().x, 10.0);
        assert_eq!(manager.get_local_position(parent).unwrap().y, 15.0);
        assert_eq!(manager.get_world_position(parent).unwrap().x, 10.0);
        assert_eq!(manager.get_world_position(parent).unwrap().y, 15.0);
        assert_eq!(manager.get_local_position(child_1).unwrap().x, 10.0);
        assert_eq!(manager.get_local_position(child_1).unwrap().y, 15.0);
        assert_eq!(manager.get_world_position(child_1).unwrap().x, 20.0);
        assert_eq!(manager.get_world_position(child_1).unwrap().y, 30.0);
        assert_eq!(manager.get_local_position(child_2).unwrap().x, 30.0);
        assert_eq!(manager.get_local_position(child_2).unwrap().y, 35.0);
        assert_eq!(manager.get_world_position(child_2).unwrap().x, 40.0);
        assert_eq!(manager.get_world_position(child_2).unwrap().y, 50.0);
    }

    #[test]
    fn should_correct_transform_child_on_parent_move() {
        let mut manager = TransformManager::new();
        let parent_transform = Transform::new(Entity::new(), Vector2::new(10.0, 15.0));
        let child_transform_1 = Transform::new(Entity::new(), Vector2::new(20.0, 30.0));
        let child_transform_2 = Transform::new(Entity::new(), Vector2::new(40.0, 50.0));

        let parent = manager.add_node(parent_transform);
        let child_1 = manager.add_node(child_transform_1);
        let child_2 = manager.add_node(child_transform_2);

        manager.attach_child(parent, child_1).unwrap();
        manager.attach_child(parent, child_2).unwrap();

        manager
            .set_local_position(parent, Vector2::new(25.0, -5.0))
            .unwrap();

        assert_eq!(manager.get_local_position(child_1).unwrap().x, 10.0);
        assert_eq!(manager.get_local_position(child_1).unwrap().y, 15.0);
        assert_eq!(manager.get_world_position(child_1).unwrap().x, 35.0);
        assert_eq!(manager.get_world_position(child_1).unwrap().y, 10.0);
        assert_eq!(manager.get_local_position(child_2).unwrap().x, 30.0);
        assert_eq!(manager.get_local_position(child_2).unwrap().y, 35.0);
        assert_eq!(manager.get_world_position(child_2).unwrap().x, 55.0);
        assert_eq!(manager.get_world_position(child_2).unwrap().y, 30.0);

        manager
            .set_world_position(parent, Vector2::new(50.0, 50.0))
            .unwrap();

        assert_eq!(manager.get_local_position(child_1).unwrap().x, 10.0);
        assert_eq!(manager.get_local_position(child_1).unwrap().y, 15.0);
        assert_eq!(manager.get_world_position(child_1).unwrap().x, 60.0);
        assert_eq!(manager.get_world_position(child_1).unwrap().y, 65.0);
        assert_eq!(manager.get_local_position(child_2).unwrap().x, 30.0);
        assert_eq!(manager.get_local_position(child_2).unwrap().y, 35.0);
        assert_eq!(manager.get_world_position(child_2).unwrap().x, 80.0);
        assert_eq!(manager.get_world_position(child_2).unwrap().y, 85.0);
    }
}
