use crate::context::catalogs::products::domain::value_objects::{
    product_id::ProductID, product_name::ProductName,
};

pub struct Product {
    id: ProductID,
    name: ProductName,
}
