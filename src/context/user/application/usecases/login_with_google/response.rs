use crate::context::shared_kernel::domain::value_objects::uuid::Uuid;

pub struct LoginWithGoogleResponse {
    pub user_uuid: Uuid,
}
