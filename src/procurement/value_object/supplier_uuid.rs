use crate::core::shared::domain::value_object::uuid::Uuid;

#[derive(Clone)]
pub struct SupplierUuid {
    value: Uuid,
}

impl SupplierUuid {
    pub fn new() -> Self {
        Self { value: Uuid::new() }
    }
}
