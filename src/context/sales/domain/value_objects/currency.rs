pub enum Currency {
    BoB,
}

impl Currency {
    pub fn minor_unit_decimal(&self) -> u8 {
        match self {
            Currency::BoB => 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn must_get_minor_unit_decimal() {
        assert_eq!(Currency::BoB.minor_unit_decimal(), 2);
    }
}
