use hashbrown::HashMap;
use stock_trek_types::cex::{account_id::AccountId, asset_id::AssetId, cex_id::CexId};

#[derive(Debug, Clone)]
pub enum Allocation {
    Stub,
    InMemory {
        cex_account_allocations: HashMap<CexId, HashMap<AccountId, Allocations>>,
    },
}

impl Allocation {
    pub fn allocation_for_asset_total(&self, asset_id: &AssetId) -> f64 {
        match self {
            Allocation::Stub => 100.0,
            Allocation::InMemory {
                cex_account_allocations,
            } => cex_account_allocations
                .values()
                .flat_map(|accounts| accounts.values())
                .map(|allocations| allocations.asset_allocations.get(asset_id).unwrap_or(&0.0))
                .sum(),
        }
    }
    pub fn allocation_for_asset_in_cex(&self, asset_id: &AssetId, cex_id: &CexId) -> f64 {
        match self {
            Allocation::Stub => 100.0,
            Allocation::InMemory {
                cex_account_allocations,
            } => cex_account_allocations
                .get(cex_id)
                .map(|accounts| {
                    accounts
                        .values()
                        .map(|allocations| {
                            allocations.asset_allocations.get(asset_id).unwrap_or(&0.0)
                        })
                        .sum()
                })
                .unwrap_or(0.0),
        }
    }
    pub fn allocation_for_asset_in_account(
        &self,
        asset_id: &AssetId,
        account_id: &AccountId,
    ) -> f64 {
        match self {
            Allocation::Stub => 100.0,
            Allocation::InMemory {
                cex_account_allocations,
            } => cex_account_allocations
                .values()
                .filter_map(|accounts| accounts.get(account_id))
                .map(|allocations| allocations.asset_allocations.get(asset_id).unwrap_or(&0.0))
                .sum(),
        }
    }
    pub fn allocation_for_asset_in_account_in_cex(
        &self,
        asset_id: &AssetId,
        account_id: &AccountId,
        cex_id: &CexId,
    ) -> f64 {
        match self {
            Allocation::Stub => 100.0,
            Allocation::InMemory {
                cex_account_allocations,
            } => cex_account_allocations
                .get(cex_id)
                .and_then(|accounts| accounts.get(account_id))
                .and_then(|allocations| allocations.asset_allocations.get(asset_id))
                .copied()
                .unwrap_or(0.0),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Allocations {
    asset_allocations: HashMap<AssetId, f64>,
}

impl Allocations {
    pub fn new(asset_allocations: HashMap<AssetId, f64>) -> Self {
        Self { asset_allocations }
    }
}

#[derive(Debug, Clone, Default)]
pub struct InMemoryAllocationBuilder {
    cex_account_allocations: HashMap<CexId, HashMap<AccountId, Allocations>>,
}

impl InMemoryAllocationBuilder {
    pub fn new() -> Self {
        Self {
            cex_account_allocations: HashMap::new(),
        }
    }
    pub fn allocation(&mut self, cex_id: CexId, asset_id: AssetId, quantity: f64) -> &mut Self {
        self.allocation_in_account(cex_id, AccountId::new(""), asset_id, quantity)
    }
    pub fn allocation_in_account(
        &mut self,
        cex_id: CexId,
        account_id: AccountId,
        asset_id: AssetId,
        quantity: f64,
    ) -> &mut Self {
        assert!(quantity > 0.0, "allocation must be greater than 0.0");
        assert!(
            quantity <= 100.0,
            "allocation must be less or equal to 100.0"
        );
        self.cex_account_allocations
            .entry(cex_id)
            .or_default()
            .entry(account_id)
            .or_insert_with(|| Allocations::new(HashMap::new()))
            .asset_allocations
            .entry(asset_id)
            .and_modify(|prev| *prev += quantity)
            .or_insert(quantity);
        self
    }
    pub fn build(&self) -> Allocation {
        Allocation::InMemory {
            cex_account_allocations: self.cex_account_allocations.clone(),
        }
    }
}

pub struct AllocationFactory;

impl AllocationFactory {
    pub fn stub() -> Allocation {
        Allocation::Stub
    }
    pub fn in_memory_builder() -> InMemoryAllocationBuilder {
        InMemoryAllocationBuilder::new()
    }
}
