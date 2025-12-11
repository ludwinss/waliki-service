use crate::{code, context::shared_kernel::errors::code::Code};

// TODO: separar los errores por tipos por modulos
pub const DATE_SALE_IN_FUTURE: Code = code!("SALE", "DATE_SALE", "VO", "001");
pub const DATE_SALE_EXPIRED: Code = code!("SALE", "DATE_SALE", "VO", "002");
