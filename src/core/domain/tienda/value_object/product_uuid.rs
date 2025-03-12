use crate::core::shared::domain::value_object::uuid::Uuid;

#[derive(Clone)]
pub struct ProductUuid(Uuid);

impl ProductUuid {
    pub fn new() -> Self {
        Self(Uuid::new())
    }

    pub fn value(&self) -> String {
        self.0.to_string()
    }
}
