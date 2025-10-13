use crate::context::shared_kernel::value_objects::uuid::Uuid;

pub trait IdGenerator: Send + Sync {
    fn new_uuid(&self) -> Uuid;
}
