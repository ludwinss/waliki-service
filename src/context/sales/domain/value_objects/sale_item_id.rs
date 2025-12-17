use crate::context::shared_kernel::domain::value_objects::uuid::Uuid;

pub struct SaleItemID {
    value: Uuid,
}

impl SaleItemID {
    pub fn new(value: Uuid) -> Self {
        Self { value }
    }
}
