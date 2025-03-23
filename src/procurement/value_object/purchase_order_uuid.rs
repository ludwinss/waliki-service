use crate::core::shared::domain::value_object::uuid::Uuid;

pub struct PurchaseOrderUuid {
    value: Uuid,
}

impl PurchaseOrderUuid {
    pub fn new() -> Self {
        Self { value: Uuid::new() }
    }
}
