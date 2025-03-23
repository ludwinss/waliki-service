use chrono::{DateTime, Utc};

use crate::procurement::value_object::{
    purchase_order_line::PurchaseOrderLine, purchase_order_uuid::PurchaseOrderUuid,
    supplier_uuid::SupplierUuid,
};

pub enum PurchaseOrderStatus {
    Pending,
}

pub struct PurchaseOrder {
    id: PurchaseOrderUuid,
    id_supplier: SupplierUuid,
    lines: Vec<PurchaseOrderLine>,
    date_order: DateTime<Utc>,
    status: PurchaseOrderStatus,
}

impl PurchaseOrder {
    pub fn new(id_supplier: SupplierUuid) -> Self {
        let date_order = Utc::now();
        let status = PurchaseOrderStatus::Pending;
        Self {
            id: PurchaseOrderUuid::new(),
            id_supplier,
            lines: Vec::new(),
            date_order,
            status,
        }
    }

    pub fn add_line(&mut self, line: PurchaseOrderLine) {
        self.lines.push(line);
    }

    pub fn get_lines(&self) -> &Vec<PurchaseOrderLine> {
        &self.lines
    }

    pub fn total_quantity(&self) -> u32 {
        self.lines.iter().map(|line| line.get_quantity()).sum()
    }

    pub fn get_id(&self) -> &PurchaseOrderUuid {
        &self.id
    }

    pub fn get_id_supplier(&self) -> &SupplierUuid {
        &self.id_supplier
    }

    pub fn get_date_order(&self) -> &DateTime<Utc> {
        &self.date_order
    }

    pub fn get_status(&self) -> &PurchaseOrderStatus {
        &self.status
    }
}
