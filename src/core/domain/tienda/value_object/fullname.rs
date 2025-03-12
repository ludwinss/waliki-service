#[derive(Clone)]
pub struct Fullname(String);

impl Fullname {
    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn value(&self) -> String {
        self.0.clone()
    }
}
