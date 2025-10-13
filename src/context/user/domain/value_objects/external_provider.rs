#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ExternalProvider {
    Google,
}

impl ExternalProvider {
    pub fn as_str(&self) -> &'static str {
        match self {
            ExternalProvider::Google => "google",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "google" | "GOOGLE" | "Google" => Some(ExternalProvider::Google),
            _ => None,
        }
    }
}
