use crate::context::sales::domain::value_objects::{
    money::Money, product_id::ProductID, quantity::Quantity, sale_item_id::SaleItemID,
};

pub struct SaleItem {
    id: SaleItemID,
    product_id: ProductID,
    quantity: Quantity,
    unit_price: Money,
}
