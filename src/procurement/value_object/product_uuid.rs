use crate::core::shared::domain::value_object::uuid::Uuid;

#[derive(Clone)]
pub struct ProductUuid {
    value: Uuid,
}

impl ProductUuid {
    pub fn new() -> Self {
        Self { value: Uuid::new() }
    }

    pub fn get_value(&self) -> &Uuid {
        &self.value
    }
}

impl Default for ProductUuid {
    fn default() -> Self {
        Self::new()
    }
}
