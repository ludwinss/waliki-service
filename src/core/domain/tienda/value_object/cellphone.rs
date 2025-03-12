#[derive(Clone)]
pub struct Cellphone(String);

impl Cellphone {
    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn value(&self) -> String {
        self.0.clone()
    }
}
