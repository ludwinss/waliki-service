#[derive(Clone)]
pub struct Price {
    amount: f64,
}

impl Price {
    pub fn new(amount: f64) -> Result<Price, String> {
        if amount > 0.0 {
            Ok(Price { amount })
        } else {
            Err(String::from("El precio debe ser positivo"))
        }
    }

    pub fn amount(&self) -> f64 {
        self.amount
    }
}
