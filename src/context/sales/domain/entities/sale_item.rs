use crate::context::sales::domain::value_objects::{
    money::Money, product_id::ProductID, quantity::Quantity,
};

pub struct SaleItem {
    pub product_id: ProductID,
    pub quantity: Quantity,
    pub unit_price: Money,
}
