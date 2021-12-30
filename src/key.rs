#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum Key {
    String(String),
    Str(&'static str),
    Int(isize),
}

impl From<String> for Key {
    fn from(s: String) -> Self {
        Key::String(s)
    }
}

impl From<&'static str> for Key {
    fn from(s: &'static str) -> Self {
        Key::Str(s)
    }
}

impl From<isize> for Key {
    fn from(i: isize) -> Self {
        Key::Int(i)
    }
}
