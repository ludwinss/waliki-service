use thiserror::Error;

use crate::procurement::value_object::product_uuid::ProductUuid;

#[derive(Error, Debug)]
pub enum PurchaseOrderLineError {
    #[error("La cantidad debe ser positiva")]
    InvalidQuantity,
    #[error("La cantidad debe ser positiva")]
    InvalidPrice,
}

pub struct PurchaseOrderLine {
    product: ProductUuid,
    quantity: u32,
    // TODO: Agregar el value object para verificar el tipo de moneda
    price: f32,
}

impl PurchaseOrderLine {
    pub fn new(
        product_uuid: ProductUuid,
        quantity: u32,
        price: f32,
    ) -> Result<Self, PurchaseOrderLineError> {
        Self::ensure_price_is_positive(&price)?;
        Ok(Self {
            product: product_uuid,
            quantity,
            price,
        })
    }

    pub fn get_quantity(&self) -> u32 {
        self.quantity
    }

    pub fn get_price(&self) -> f32 {
        self.price
    }

    pub fn get_product(&self) -> &ProductUuid {
        &self.product
    }

    fn ensure_price_is_positive(price: &f32) -> Result<(), PurchaseOrderLineError> {
        if *price < 0.0 {
            return Err(PurchaseOrderLineError::InvalidPrice);
        }
        Ok(())
    }
}
