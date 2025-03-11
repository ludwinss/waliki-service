use crate::core::{domain::tienda::entities::producto::Producto, shared::domain::value_object::uuid::Uuid};

trait ProductoRepository {
    fn save(&self, producto: Producto) -> Result<(), String>;
    fn find_by_id(&self, id: Uuid) -> Result<Producto, String>;
    fn find_all(&self) -> Result<Vec<Producto>, String>;
    fn find_by_dueno(&self, dueno: Uuid) -> Result<Vec<Producto>, String>;
    fn delete(&self, id: Uuid) -> Result<(), String>;
}
