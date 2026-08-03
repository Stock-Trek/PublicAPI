use crate::portfolios::{in_memory_portfolio::InMemoryPortfolio, stub_portfolio::StubPortfolio};
use stock_trek_types::cex::{asset_id::AssetId, cex_id::CexId, tag::Tag};

#[derive(Debug, Clone)]
pub enum Portfolio {
    InMemory(InMemoryPortfolio),
    Stub(StubPortfolio),
}

impl Portfolio {
    pub fn has_account_in_cex(&self, cex_id: &CexId) -> bool {
        match self {
            Portfolio::InMemory(portfolio) => portfolio.has_account_in_cex(cex_id),
            Portfolio::Stub(portfolio) => portfolio.has_account_in_cex(cex_id),
        }
    }
    pub fn owns_asset(&self, asset_id: &AssetId) -> bool {
        match self {
            Portfolio::InMemory(portfolio) => portfolio.owns_asset(asset_id),
            Portfolio::Stub(portfolio) => portfolio.owns_asset(asset_id),
        }
    }
    pub fn owns_asset_in_cex(&self, asset_id: &AssetId, cex_id: &CexId) -> bool {
        match self {
            Portfolio::InMemory(portfolio) => portfolio.owns_asset_in_cex(asset_id, cex_id),
            Portfolio::Stub(portfolio) => portfolio.owns_asset_in_cex(asset_id, cex_id),
        }
    }
    pub fn asset_total(&self, asset_id: &AssetId) -> f64 {
        match self {
            Portfolio::InMemory(portfolio) => portfolio.asset_total(asset_id),
            Portfolio::Stub(portfolio) => portfolio.asset_total(asset_id),
        }
    }
    pub fn asset_in_cex(&self, asset_id: &AssetId, cex_id: &CexId) -> f64 {
        match self {
            Portfolio::InMemory(portfolio) => portfolio.asset_in_cex(asset_id, cex_id),
            Portfolio::Stub(portfolio) => portfolio.asset_in_cex(asset_id, cex_id),
        }
    }
    pub fn active_orders(&self) -> f64 {
        match self {
            Portfolio::InMemory(portfolio) => portfolio.active_orders(),
            Portfolio::Stub(portfolio) => portfolio.active_orders(),
        }
    }
    pub fn active_orders_with_tag(&self, order_tag: &Tag) -> f64 {
        match self {
            Portfolio::InMemory(portfolio) => portfolio.active_orders_with_tag(order_tag),
            Portfolio::Stub(portfolio) => portfolio.active_orders_with_tag(order_tag),
        }
    }
    pub fn active_orders_in_cex(&self, cex_id: &CexId) -> f64 {
        match self {
            Portfolio::InMemory(portfolio) => portfolio.active_orders_in_cex(cex_id),
            Portfolio::Stub(portfolio) => portfolio.active_orders_in_cex(cex_id),
        }
    }
    pub fn active_orders_in_cex_with_tag(&self, cex_id: &CexId, order_tag: &Tag) -> f64 {
        match self {
            Portfolio::InMemory(portfolio) => {
                portfolio.active_orders_in_cex_with_tag(cex_id, order_tag)
            }
            Portfolio::Stub(portfolio) => {
                portfolio.active_orders_in_cex_with_tag(cex_id, order_tag)
            }
        }
    }
}

impl From<InMemoryPortfolio> for Portfolio {
    fn from(value: InMemoryPortfolio) -> Self {
        Portfolio::InMemory(value)
    }
}

impl From<StubPortfolio> for Portfolio {
    fn from(value: StubPortfolio) -> Self {
        Portfolio::Stub(value)
    }
}
