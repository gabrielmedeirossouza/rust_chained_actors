pub struct Observable {
  observers: Vec<Box<dyn Fn()>>
}

impl Observable {
  pub fn new() -> Self {
    Self {
      observers: vec!()
    }
  }

  pub fn subscribe(&mut self, observer: Box<dyn Fn()>) {
    self.observers.push(observer)
  }

  pub fn notify(&self) {
    for observer in &self.observers {
      observer();
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn should_create_empty_observable() {
    let sut = Observable::new();
    assert_eq!(sut.observers.len(), 0);
  }

  #[test]
  fn should_subscribe_an_observer() {
    let mut sut = Observable::new();
    sut.subscribe(Box::new(|| {}));
    assert_eq!(sut.observers.len(), 1)
  }

  #[test]
  #[should_panic]
  fn should_notify_an_observer() {
    let mut sut = Observable::new();
    sut.subscribe(Box::new(|| {
      panic!()
    }));
    sut.notify();
  }
}