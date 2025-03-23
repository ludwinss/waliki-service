use crate::core::shared::domain::value_object::uuid::Uuid;

#[derive(Clone)]
pub struct OwnerUuid(Uuid);

impl Default for OwnerUuid {
    fn default() -> Self {
        Self::new()
    }
}

impl OwnerUuid {
    pub fn new() -> Self {
        Self(Uuid::new())
    }

    pub fn value(&self) -> String {
        self.0.to_string()
    }
}
