use crate::app_core::math::vector2::Vector2;

#[derive(Debug)]
pub struct Actor {
  pub name: String,
  pub pos: Vector2
}

impl Actor {
  pub fn new(name: String, pos: Vector2) -> Self {
    Self {
      name,
      pos
    }
  }
}