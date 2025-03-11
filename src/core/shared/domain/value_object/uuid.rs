use uuid::Uuid as UuidCrate;

#[derive(Clone)]
pub struct Uuid(pub UuidCrate);

impl Uuid {
    pub fn new() -> Self {
        Self(UuidCrate::new_v4())
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

