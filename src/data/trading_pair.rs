use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TradingPair {
    base: String,
    quote: String,
}

impl TradingPair {
    pub fn new(base: impl Into<String>, quote: impl Into<String>) -> Self {
        Self {
            base: base.into(),
            quote: quote.into(),
        }
    }
    pub fn base(&self) -> &str {
        &self.base
    }
    pub fn quote(&self) -> &str {
        &self.quote
    }
}

impl fmt::Display for TradingPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.base, self.quote)
    }
}
