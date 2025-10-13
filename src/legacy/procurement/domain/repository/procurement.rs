pub trait ProcurementRepository {
    fn find_purchase_orders_by_supplier(&self, supplier_id: String) -> Vec<String>;

    fn find_purchase_order_by_id(&self, id: String) -> String;
}
