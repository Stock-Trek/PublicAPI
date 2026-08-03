use stock_trek_types::cex::{asset_id::AssetId, cex_id::CexId, tag::Tag};

#[derive(Debug, Clone, Default)]
pub struct StubPortfolio;

impl StubPortfolio {
    pub fn new() -> Self {
        Self
    }
}

impl StubPortfolio {
    pub fn has_account_in_cex(&self, _cex_id: &CexId) -> bool {
        true
    }
    pub fn owns_asset(&self, _asset_id: &AssetId) -> bool {
        true
    }
    pub fn owns_asset_in_cex(&self, _asset_id: &AssetId, _cex_id: &CexId) -> bool {
        true
    }
    pub fn asset_total(&self, _asset_id: &AssetId) -> f64 {
        1_000_000.0
    }
    pub fn asset_in_cex(&self, _asset_id: &AssetId, _cex_id: &CexId) -> f64 {
        1_000_000.0
    }
    pub fn active_orders(&self) -> f64 {
        0.0
    }
    pub fn active_orders_in_cex(&self, _cex_id: &CexId) -> f64 {
        0.0
    }
    pub fn active_orders_in_cex_with_tag(&self, _cex_id: &CexId, _tag: &Tag) -> f64 {
        0.0
    }
    pub fn active_orders_with_tag(&self, _tag: &Tag) -> f64 {
        0.0
    }
}
