use crate::core::{
    domain::tienda::value_object::{
        price::Price, product_uuid::ProductUuid, supplier_uuid::SupplierUuid,
    },
    shared::domain::{agregate_root::AggregateRoot, domain_events::DomainEvent},
};

pub struct Product {
    id: ProductUuid,
    name: String,
    description: String,
    price: Price,
    supplier: SupplierUuid,
    domain_events: Vec<Box<dyn DomainEvent>>,
}

impl Product {
    pub fn new(
        id: ProductUuid,
        name: String,
        description: String,
        price: Price,
        supplier: SupplierUuid,
    ) -> Self {
        Self {
            id,
            name,
            description,
            price,
            supplier,
            domain_events: Vec::new(),
        }
    }

    pub fn id(&self) -> ProductUuid {
        self.id.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn description(&self) -> String {
        self.description.clone()
    }

    pub fn price(&self) -> Price {
        self.price.clone()
    }

    pub fn supplier(&self) -> SupplierUuid {
        self.supplier.clone()
    }
}

impl AggregateRoot for Product {
    fn domain_events(&self) -> &Vec<Box<dyn DomainEvent>> {
        &self.domain_events
    }

    fn domain_events_mut(&mut self) -> &mut Vec<Box<dyn DomainEvent>> {
        &mut self.domain_events
    }
}
