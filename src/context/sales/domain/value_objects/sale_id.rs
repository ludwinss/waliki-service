use crate::context::shared_kernel::domain::value_objects::uuid::Uuid;

pub struct SaleID {
    value: Uuid,
}

impl SaleID {
    pub fn new(value: Uuid) -> Self {
        Self { value }
    }
}
