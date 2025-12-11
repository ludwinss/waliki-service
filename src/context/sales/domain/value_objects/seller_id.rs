use crate::context::shared_kernel::domain::value_objects::uuid::Uuid;

pub struct SellerID {
    value: Uuid,
}

impl SellerID {
    pub fn new(value: Uuid) -> Self {
        Self { value }
    }
}
