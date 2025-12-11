use crate::context::sales::domain::{
    entities::sale_item::SaleItem,
    value_objects::{
        payment_method::PaymentMethod, sale_id::SaleID, seller_id::SellerID, store_id::StoreID,
    },
};

pub struct Sales {
    id: SaleID,
    store_id: StoreID,
    perfomed_by: SellerID,
    payment_method: PaymentMethod,
    sale_items: Vec<SaleItem>,
}
