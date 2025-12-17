use crate::context::sales::domain::{
    entities::sale_item::SaleItem,
    value_objects::{
        date_sale::DateSale, money::Money, payment_method::PaymentMethod, sale_id::SaleID,
        seller_id::SellerID, store_id::StoreID,
    },
};

pub struct Sale {
    id: SaleID,
    store_id: StoreID,
    perfomed_by: SellerID,
    payment_method: PaymentMethod,
    sale_items: Vec<SaleItem>,
    date: DateSale,
    total_amount: Money,
}
