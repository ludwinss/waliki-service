use crate::core::domain::tienda::value_object::{
    cellphone::Cellphone, email::Email, fullname::Fullname, supplier_uuid::SupplierUuid,
};

pub struct Supplier {
    id: SupplierUuid,
    full_name: Fullname,
    cellphone: Cellphone,
    email: Email,
    nick_name: Option<String>,
}

impl Supplier {
    pub fn new(
        id: SupplierUuid,
        full_name: Fullname,
        email: Email,
        cellphone: Cellphone,
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

    pub fn id(&self) -> SupplierUuid {
        self.id.clone()
    }

    pub fn full_name(&self) -> Fullname {
        self.full_name.clone()
    }

    pub fn email(&self) -> Email {
        self.email.clone()
    }

    pub fn cellphone(&self) -> Cellphone {
        self.cellphone.clone()
    }

    pub fn nick_name(&self) -> Option<String> {
        self.nick_name.clone()
    }
}
