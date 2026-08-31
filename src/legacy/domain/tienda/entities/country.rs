#[derive(Clone)]
pub struct Country {
    pub iso_code: &'static str,
    pub name: &'static str,
    pub currency: &'static str,
    pub phone_prefix: &'static str,
    pub phone_digits: usize,
    pub reg_exp: &'static str,
}

impl Country {
    pub fn get_reg_exp(&self) -> &str {
        self.reg_exp
    }
}
