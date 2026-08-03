use crate::allocations::allocation::Allocation;
use stock_trek_types::cex::{asset_id::AssetId, cex_id::CexId};

#[derive(Debug, Clone, Default)]
pub struct StubAllocation;

impl StubAllocation {
    pub fn new() -> Self {
        Self
    }
    pub fn allocation_for_asset_total(&self, _asset_id: &AssetId) -> f64 {
        100.0
    }
    pub fn allocation_for_asset_in_cex(&self, _asset_id: &AssetId, _cex_id: &CexId) -> f64 {
        100.0
    }
}

impl From<StubAllocation> for Allocation {
    fn from(value: StubAllocation) -> Self {
        Allocation::Stub(value)
    }
}
