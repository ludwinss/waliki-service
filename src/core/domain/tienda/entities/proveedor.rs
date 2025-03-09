use crate::core::domain::tienda::value_object::identificadores::proveedor_id::ProveedorId;

pub struct Proveedor {
    id: ProveedorId,
    full_name: String,
    telefono: String,
    email: String,
    chapa: Option<String>,
}

impl Proveedor {
    pub fn new(
        id: ProveedorId,
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

    pub fn id(&self) -> ProveedorId {
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
