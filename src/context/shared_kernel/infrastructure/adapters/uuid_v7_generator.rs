use crate::context::{
    shared_kernel::domain::value_objects::uuid::Uuid,
    user::application::ports::id_generator::IdGenerator,
};

pub struct UuidV7Generator;

impl IdGenerator for UuidV7Generator {
    fn new_uuid(&self) -> Uuid {
        uuid::Uuid::now_v7()
    }
}
