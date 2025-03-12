use crate::core::{
    domain::tienda::entities::product::Product, shared::domain::value_object::uuid::Uuid,
};

trait ProductRepository {
    fn save(&self, producto: Product) -> Result<(), String>;
    fn find_by_id(&self, id: Uuid) -> Result<Product, String>;
    fn find_all(&self) -> Result<Vec<Product>, String>;
    fn find_by_dueno(&self, dueno: Uuid) -> Result<Vec<Product>, String>;
    fn delete(&self, id: Uuid) -> Result<(), String>;
}
