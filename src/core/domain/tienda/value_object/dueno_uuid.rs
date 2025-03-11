use crate::core::shared::domain::value_object::uuid::Uuid;

#[derive(Clone)]
pub struct DuenoUuid(Uuid);

impl DuenoUuid {
    pub fn new() -> Self {
        Self(Uuid::new())
    }

    pub fn value(&self) -> String {
        self.0.to_string()
    }
}
