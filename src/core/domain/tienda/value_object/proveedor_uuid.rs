use crate::core::shared::domain::value_object::uuid::Uuid;

#[derive(Clone)]
pub struct ProveedorUuid(Uuid);

impl ProveedorUuid {
    pub fn new() -> Self {
        Self(Uuid::new())
    }

    pub fn value(&self) -> String {
        self.0.to_string()
    }
}
