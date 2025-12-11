use crate::context::shared_kernel::domain::value_objects::uuid::Uuid;

pub struct ProductID {
    value: Uuid,
}

impl ProductID {
    pub fn new(value: Uuid) -> Self {
        Self { value }
    }
}
