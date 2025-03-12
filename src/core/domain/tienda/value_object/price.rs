#[derive(Clone)]
pub struct Price(f64);

impl Price {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}
