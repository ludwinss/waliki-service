use crate::core::shared::domain::value_object::uuid::Uuid;

pub struct Owner {
    id: Uuid,
    full_name: String,
    cellphone: String,
    email: String,
    nick_name: Option<String>,
}

impl Owner {
    pub fn new(
        id: Uuid,
        full_name: String,
        email: String,
        cellphone: String,
        nick_name: Option<String>,
    ) -> Self {
        Self {
            id,
            full_name,
            email,
            cellphone,
            nick_name,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id.clone()
    }

    pub fn full_name(&self) -> String {
        self.full_name.clone()
    }

    pub fn email(&self) -> String {
        self.email.clone()
    }

    pub fn cellphone(&self) -> String {
        self.cellphone.clone()
    }

    pub fn nick_name(&self) -> Option<String> {
        self.nick_name.clone()
    }
}
