#[derive(Clone)]
pub struct Email (String);

impl Email {
    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn value(&self) -> String {
        self.0.clone()
    }
}
