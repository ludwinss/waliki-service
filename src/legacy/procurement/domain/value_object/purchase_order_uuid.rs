use crate::core::shared::domain::value_object::uuid::Uuid;

pub struct PurchaseOrderUuid {
    value: Uuid,
}

impl PurchaseOrderUuid {
    pub fn new() -> Self {
        Self { value: Uuid::new() }
    }

    pub fn value(&self) -> Uuid {
        self.value.clone()
    }
}

impl Default for PurchaseOrderUuid {
    fn default() -> Self {
        Self::new()
    }
}
