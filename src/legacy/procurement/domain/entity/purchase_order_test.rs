use chrono::Utc;

use crate::core::procurement::domain::{
    entity::purchase_order::PurchaseOrder,
    value_object::{
        product_uuid::ProductUuid, purchase_order_line::PurchaseOrderLine,
        supplier_uuid::SupplierUuid,
    },
};

#[test]
fn should_create_purchase_order() {
    let id_supplier = SupplierUuid::new();
    let purchase_order = PurchaseOrder::new(id_supplier.clone());

    assert_eq!(purchase_order.get_lines().len(), 0);
    assert_eq!(purchase_order.total_quantity(), 0);
}

#[test]
fn should_add_single_line() {
    let mut order = PurchaseOrder::new(SupplierUuid::new());

    let line = PurchaseOrderLine::new(ProductUuid::new(), 3, 10.0).unwrap();
    order.add_line(line);

    assert_eq!(order.get_lines().len(), 1);
    assert_eq!(order.total_quantity(), 3);
}

#[test]
fn should_add_multiple_lines_and_sum_quantities() {
    let mut order = PurchaseOrder::new(SupplierUuid::new());

    order.add_line(PurchaseOrderLine::new(ProductUuid::new(), 2, 10.0).unwrap());
    order.add_line(PurchaseOrderLine::new(ProductUuid::new(), 5, 20.0).unwrap());
    order.add_line(PurchaseOrderLine::new(ProductUuid::new(), 8, 30.0).unwrap());

    assert_eq!(order.get_lines().len(), 3);
    assert_eq!(order.total_quantity(), 15);
}

#[test]
fn should_accept_duplicate_product_lines() {
    let mut order = PurchaseOrder::new(SupplierUuid::new());
    let product_id = ProductUuid::new();

    order.add_line(PurchaseOrderLine::new(product_id.clone(), 2, 10.0).unwrap());
    order.add_line(PurchaseOrderLine::new(product_id.clone(), 2, 10.0).unwrap());

    assert_eq!(order.get_lines().len(), 2);
    assert_eq!(order.total_quantity(), 4);
}

#[test]
fn date_order_should_be_now_or_earlier() {
    let now = Utc::now();
    let order = PurchaseOrder::new(SupplierUuid::new());

    assert!(now <= *order.get_date_order());
}
