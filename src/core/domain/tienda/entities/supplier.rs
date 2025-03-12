use crate::core::domain::tienda::value_object::supplier_uuid::SupplierUuid;

pub struct Supplier {
    id: SupplierUuid,
    full_name: String,
    telefono: String,
    email: String,
    chapa: Option<String>,
}

impl Supplier {
    pub fn new(
        id: SupplierUuid,
        full_name: String,
        email: String,
        telefono: String,
        chapa: Option<String>,
    ) -> Self {
        Self {
            id,
            full_name,
            email,
            telefono,
            chapa,
        }
    }

    pub fn id(&self) -> SupplierUuid {
        self.id.clone()
    }

    pub fn full_name(&self) -> String {
        self.full_name.clone()
    }

    pub fn telefono(&self) -> String {
        self.telefono.clone()
    }

    pub fn chapa(&self) -> Option<String> {
        self.chapa.clone()
    }

    pub fn email(&self) -> String {
        self.email.clone()
    }
}
