use uuid::Uuid;

#[derive(Clone)]
pub struct ProveedorId(Uuid);

impl ProveedorId {
    pub fn new(id: String) -> Self {
        Self(id)
    }
}
