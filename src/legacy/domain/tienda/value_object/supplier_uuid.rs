use crate::core::shared::domain::value_object::uuid::Uuid;

#[derive(Clone)]
pub struct SupplierUuid(Uuid);

impl Default for SupplierUuid {
    fn default() -> Self {
        Self::new()
    }
}

impl SupplierUuid {
    pub fn new() -> Self {
        Self(Uuid::new())
    }

    pub fn value(&self) -> String {
        self.0.to_string()
    }
}
