use crate::core::domain::tienda::value_object::{
    dueno_uuid::DuenoUuid, producto_uuid::ProductoUuid,
};

pub struct Producto {
    id: ProductoUuid,
    nombre: String,
    descripcion: String,
    precio: f64,
    tienda: String,
    dueno: DuenoUuid,
}

impl Producto {
    pub fn new(
        id: ProductoUuid,
        nombre: String,
        descripcion: String,
        precio: f64,
        tienda: String,
        dueno: DuenoUuid,
    ) -> Self {
        Self {
            id,
            nombre,
            descripcion,
            precio,
            tienda,
            dueno,
        }
    }

    pub fn id(&self) -> ProductoUuid {
        self.id.clone()
    }

    pub fn nombre(&self) -> String {
        self.nombre.clone()
    }

    pub fn descripcion(&self) -> String {
        self.descripcion.clone()
    }

    pub fn precio(&self) -> f64 {
        self.precio.clone()
    }

    pub fn tienda(&self) -> String {
        self.tienda.clone()
    }
    pub fn dueno(&self) -> DuenoUuid {
        self.dueno.clone()
    }
}
