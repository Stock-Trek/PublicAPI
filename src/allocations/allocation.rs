use crate::allocations::{
    in_memory_allocation::InMemoryAllocation, stub_allocation::StubAllocation,
};
use stock_trek_types::cex::{asset_id::AssetId, cex_id::CexId};

#[derive(Debug, Clone)]
pub enum Allocation {
    Stub(StubAllocation),
    InMemory(InMemoryAllocation),
}

impl Default for Allocation {
    fn default() -> Self {
        Allocation::Stub(StubAllocation)
    }
}

impl Allocation {
    pub fn allocation_for_asset_total(&self, asset_id: &AssetId) -> f64 {
        match self {
            Allocation::Stub(allocation) => allocation.allocation_for_asset_total(asset_id),
            Allocation::InMemory(allocation) => allocation.allocation_for_asset_total(asset_id),
        }
    }
    pub fn allocation_for_asset_in_cex(&self, asset_id: &AssetId, cex_id: &CexId) -> f64 {
        match self {
            Allocation::Stub(allocation) => {
                allocation.allocation_for_asset_in_cex(asset_id, cex_id)
            }
            Allocation::InMemory(allocation) => {
                allocation.allocation_for_asset_in_cex(asset_id, cex_id)
            }
        }
    }
}
