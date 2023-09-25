use crate::entities::actor::Actor;

#[derive(Debug)]
pub struct World<'a> {
  pub actors: Vec<&'a mut Actor>
}

impl<'a> World<'a> {
  pub fn new() -> Self {
    Self {
      actors: Vec::new()
    }
  }

  pub fn add_actor(&mut self, actor: &'a mut Actor) {
    if self.get_actor_by_name(actor.name.clone()).is_none() {
      self.actors.push(actor);
    }
  }

  pub fn remove_actor(&mut self, name: String) {
    if let Some(index) = self.actors.iter().position(|a| a.name == name) {
      self.actors.remove(index);
    }
  }

  pub fn get_actor_by_name(&mut self, name: String) -> Option<&mut &'a mut Actor> {
    self.actors.iter_mut().find(|a| a.name == name)
  }
}