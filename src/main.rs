use app_core::math::vector2::Vector2;
use application::scene::Scene;
use entities::{actor::Actor, entity::Entity, transform::Transform};

mod app_core;
mod application;
mod entities;

fn main() {
    let mut scene = Scene::new();

    let player = Entity::new();
    let player_actor = Actor::new(player, "player".into());
    let player_transform = Transform::new(player, Vector2::zero());
    scene.actor_manager.add_node(player_actor);
    scene.transform_manager.add_node(player_transform);

    loop {
        scene
            .transform_manager
            .set_world_position(
                player,
                scene.transform_manager.get_world_position(player).unwrap()
                    + Vector2::new(1.0, 0.0),
            )
            .unwrap();

        println!(
            "{:?}",
            scene.transform_manager.get_world_position(player).unwrap()
        );
    }
}
