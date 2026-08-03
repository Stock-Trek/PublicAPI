use crate::portfolios::portfolio::{Builder, Portfolio};

pub struct PortfolioFactory;

impl PortfolioFactory {
    pub fn stub() -> Portfolio {
        Portfolio::Stub
    }

    pub fn in_memory_builder() -> Builder {
        Builder::new()
    }
}
