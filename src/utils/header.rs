#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
    key: String,
    value: String,
}

impl Header {
    pub fn new(key: String, value: String) -> Self {
        Self { key, value }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}