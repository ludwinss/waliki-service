use anyhow::Result;

pub struct ProductName {
    value: String,
}

static PRODUCT_NAME_MAX_LENGTH: usize = 150;

impl ProductName {
    pub fn new(value: String) -> Result<Self> {
        Self::ensure_is_not_empty(&value)?;
        Self::ensure_is_not_too_long(&value)?;
        Ok(Self { value })
    }

    fn ensure_is_not_empty(value: &String) -> Result<()> {
        if value.is_empty() {
            // TODO: Create a manage error for domains
            return Err(anyhow::anyhow!("Product name cannot be empty"));
        }
        Ok(())
    }

    fn ensure_is_not_too_long(value: &String) -> Result<()> {
        if value.len() > PRODUCT_NAME_MAX_LENGTH {
            // TODO: Create a manage error for domains
            return Err(anyhow::anyhow!(
                "Product name cannot be longer than 150 characters"
            ));
        }
        Ok(())
    }
}
