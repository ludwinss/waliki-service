use crate::context::shared_kernel::domain::value_objects::uuid::Uuid;

pub struct StoreID {
    value: Uuid,
}

impl StoreID {
    pub fn new(value: Uuid) -> Self {
        Self { value }
    }
}
