use uuid::Uuid;

#[derive(Clone)]
pub struct DuenoId(Uuid);

impl DuenoId {
    pub fn new(uuid: Uuid) -> Self {
        Self(uuid)
    }
}
