use crate::{code, context::shared_kernel::errors::code::Code};
// TODO: separar los errores por tipos por modulos

// Value Object EMAIL
pub const EMPTY_EMAIL: Code = code!("USR", "EMAIL", "V", "001");
pub const INVALID_EMAIL_LENGTH: Code = code!("USR", "EMAIL", "V", "002");
pub const INVALID_EMAIL: Code = code!("USR", "EMAIL", "V", "003");

// Value Object FULLNAME
pub const EMPTY_FULLNAME: Code = code!("USR", "NAME", "V", "001");
pub const INVALID_FULLNAME_LENGTH: Code = code!("USR", "NAME", "V", "002");

// Value Object EXTERNAL_SUBJECT
pub const EMPTY_EXTERNAL_SUBJECT: Code = code!("USR", "EXTERNAL_SUBJECT", "V", "001");
pub const INVALID_EXTERNAL_SUBJECT_LENGTH: Code = code!("USR", "EXTERNAL_SUBJECT", "V", "002");
pub const INVALID_EXTERNAL_SUBJECT: Code = code!("USR", "EXTERNAL_SUBJECT", "V", "003");
