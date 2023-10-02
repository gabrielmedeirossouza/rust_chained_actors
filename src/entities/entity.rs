use uuid::Uuid;

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub struct Entity(u128);

impl Entity {
    pub fn new() -> Self {
        let uuid = Uuid::new_v4();
        Self(uuid.as_u128())
    }
}
