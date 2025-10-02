#[derive(PartialEq, Eq, Clone)]
pub enum Status {
    Active,
}

impl Status {
    pub fn as_str(&self) -> &'static str {
        match self {
            Status::Active => "active",
        }
    }
}
