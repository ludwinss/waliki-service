use std::fmt::Display;

#[derive(Debug)]
pub struct Code(pub &'static str);

impl Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

#[macro_export]
macro_rules! code {
    ($bc:literal, $ent:literal, $cat:literal, $id3:literal) => {
        $crate::context::shared::errors::code::Code(concat!($bc, $ent, $cat, $id3))
    };
}
