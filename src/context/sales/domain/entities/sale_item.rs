use crate::context::sales::domain::value_objects::{
    line_total::LineTotal, product_id::ProductID, quantity::Quantity, unit_price::UnitPrice,
};

pub struct SaleItem {
    pub product_id: ProductID,
    pub quantity: Quantity,
    pub unit_price: UnitPrice,
    pub line_total: LineTotal,
}
