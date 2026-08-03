use crate::portfolios::portfolio::{InMemoryPortfolioBuilder, Portfolio};

pub struct PortfolioFactory;

impl PortfolioFactory {
    pub fn stub() -> Portfolio {
        Portfolio::Stub
    }

    pub fn in_memory_builder() -> InMemoryPortfolioBuilder {
        InMemoryPortfolioBuilder::new()
    }
}
