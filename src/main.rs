mod app_core;
mod application;
mod entities;

use app_core::math::vector2::Vector2;
use application::world::World;
use entities::{actor::Actor, observable::Observable};

fn main() {
    // let mut world = World::new();

    // let mut player_a = Actor::new(String::from("Player 1"), Vector2::new(0.0, 0.0));

    // let mut player_b = Actor::new(String::from("Player 2"), Vector2::new(0.0, 0.0));

    // world.add_actor(&mut player_a);
    // world.add_actor(&mut player_b);

    // let player_a = world.get_actor_by_name(String::from("Player 1")).unwrap();
    // player_a.pos.x = 10.0;

    // world.remove_actor(String::from("Player 2"));

    // let player_a = world.get_actor_by_name(String::from("Player 1")).unwrap();
    // player_a.pos.x += 5.0;

    // println!("{:?}", world);

    // ### observable.rs
    // let mut observable = Observable::new();

    // observable.subscribe(Box::new(|| {
    //     println!("Hello, world!");
    // }));

    // observable.notify();
    // observable.notify();
    // observable.notify();
    // ### observable.rs
}
