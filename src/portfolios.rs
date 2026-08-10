use hashbrown::HashMap;
use stock_trek_types::cex::{account_id::AccountId, asset_id::AssetId, cex_id::CexId, tag::Tag};

#[derive(Debug, Clone)]
pub enum Portfolio {
    Stub,
    InMemory {
        cex_account_assets: HashMap<CexId, HashMap<AccountId, Assets>>,
    },
}

impl Portfolio {
    pub fn has_account(&self, account_id: &AccountId) -> bool {
        match self {
            Portfolio::InMemory { cex_account_assets } => cex_account_assets
                .values()
                .any(|accounts| accounts.contains_key(account_id)),
            Portfolio::Stub => true,
        }
    }

    pub fn has_account_in_cex(&self, cex_id: &CexId) -> bool {
        match self {
            Portfolio::InMemory { cex_account_assets } => cex_account_assets.contains_key(cex_id),
            Portfolio::Stub => true,
        }
    }

    pub fn owns_asset(&self, asset_id: &AssetId) -> bool {
        match self {
            Portfolio::InMemory { cex_account_assets } => cex_account_assets
                .values()
                .flat_map(|accounts| accounts.values())
                .any(|assets| assets.asset_counts.contains_key(asset_id)),
            Portfolio::Stub => true,
        }
    }

    pub fn owns_asset_in_cex(&self, asset_id: &AssetId, cex_id: &CexId) -> bool {
        match self {
            Portfolio::InMemory { cex_account_assets } => cex_account_assets
                .get(cex_id)
                .map(|accounts| {
                    accounts
                        .values()
                        .any(|assets| assets.asset_counts.contains_key(asset_id))
                })
                .unwrap_or(false),
            Portfolio::Stub => true,
        }
    }

    pub fn owns_asset_in_account(&self, asset_id: &AssetId, account_id: &AccountId) -> bool {
        match self {
            Portfolio::InMemory { cex_account_assets } => cex_account_assets
                .values()
                .filter_map(|accounts| accounts.get(account_id))
                .any(|assets| assets.asset_counts.contains_key(asset_id)),
            Portfolio::Stub => true,
        }
    }

    pub fn owns_asset_in_account_in_cex(
        &self,
        asset_id: &AssetId,
        account_id: &AccountId,
        cex_id: &CexId,
    ) -> bool {
        match self {
            Portfolio::InMemory { cex_account_assets } => cex_account_assets
                .get(cex_id)
                .and_then(|accounts| accounts.get(account_id))
                .map(|assets| assets.asset_counts.contains_key(asset_id))
                .unwrap_or(false),
            Portfolio::Stub => true,
        }
    }

    pub fn asset_total(&self, asset_id: &AssetId) -> f64 {
        match self {
            Portfolio::InMemory { cex_account_assets } => cex_account_assets
                .values()
                .flat_map(|accounts| accounts.values())
                .map(|assets| assets.asset_counts.get(asset_id).unwrap_or(&0.0))
                .sum(),
            Portfolio::Stub => 1_000_000.0,
        }
    }

    pub fn asset_in_cex(&self, asset_id: &AssetId, cex_id: &CexId) -> f64 {
        match self {
            Portfolio::InMemory { cex_account_assets } => cex_account_assets
                .get(cex_id)
                .map(|accounts| {
                    accounts
                        .values()
                        .map(|assets| assets.asset_counts.get(asset_id).unwrap_or(&0.0))
                        .sum()
                })
                .unwrap_or(0.0),
            Portfolio::Stub => 1_000_000.0,
        }
    }

    pub fn asset_in_account(&self, asset_id: &AssetId, account_id: &AccountId) -> f64 {
        match self {
            Portfolio::InMemory { cex_account_assets } => cex_account_assets
                .values()
                .filter_map(|accounts| accounts.get(account_id))
                .map(|assets| assets.asset_counts.get(asset_id).unwrap_or(&0.0))
                .sum(),
            Portfolio::Stub => 1_000_000.0,
        }
    }

    pub fn asset_in_account_in_cex(
        &self,
        asset_id: &AssetId,
        account_id: &AccountId,
        cex_id: &CexId,
    ) -> f64 {
        match self {
            Portfolio::InMemory { cex_account_assets } => cex_account_assets
                .get(cex_id)
                .and_then(|accounts| accounts.get(account_id))
                .and_then(|assets| assets.asset_counts.get(asset_id))
                .copied()
                .unwrap_or(0.0),
            Portfolio::Stub => 1_000_000.0,
        }
    }

    pub fn active_orders(&self) -> f64 {
        0.0
    }

    pub fn active_orders_with_tag(&self, _tag: &Tag) -> f64 {
        0.0
    }

    pub fn active_orders_in_account(&self, _account_id: &AccountId) -> f64 {
        0.0
    }

    pub fn active_orders_in_account_with_tag(&self, _account_id: &AccountId, _tag: &Tag) -> f64 {
        0.0
    }

    pub fn active_orders_in_cex(&self, _cex_id: &CexId) -> f64 {
        0.0
    }

    pub fn active_orders_in_cex_with_tag(&self, _cex_id: &CexId, _tag: &Tag) -> f64 {
        0.0
    }

    pub fn active_orders_in_account_in_cex(&self, _account_id: &AccountId, _cex_id: &CexId) -> f64 {
        0.0
    }

    pub fn active_orders_in_account_in_cex_with_tag(
        &self,
        _account_id: &AccountId,
        _cex_id: &CexId,
        _tag: &Tag,
    ) -> f64 {
        0.0
    }
}

#[derive(Debug, Clone, Default)]
pub struct Assets {
    asset_counts: HashMap<AssetId, f64>,
}

impl Assets {
    pub fn new(asset_counts: HashMap<AssetId, f64>) -> Self {
        Self { asset_counts }
    }
}

#[derive(Debug, Clone, Default)]
pub struct InMemoryPortfolioBuilder {
    cex_account_assets: HashMap<CexId, HashMap<AccountId, Assets>>,
}

impl InMemoryPortfolioBuilder {
    pub fn new() -> Self {
        Self {
            cex_account_assets: HashMap::new(),
        }
    }

    pub fn assets(&mut self, cex_id: CexId, asset_id: AssetId, quantity: f64) -> &mut Self {
        self.assets_in_account(cex_id, AccountId::new(""), asset_id, quantity)
    }

    pub fn assets_in_account(
        &mut self,
        cex_id: CexId,
        account_id: AccountId,
        asset_id: AssetId,
        quantity: f64,
    ) -> &mut Self {
        self.cex_account_assets
            .entry(cex_id)
            .or_default()
            .entry(account_id)
            .or_insert_with(|| Assets::new(HashMap::new()))
            .asset_counts
            .entry(asset_id)
            .and_modify(|prev| *prev += quantity)
            .or_insert(quantity);
        self
    }

    pub fn build(&self) -> Portfolio {
        Portfolio::InMemory {
            cex_account_assets: self.cex_account_assets.clone(),
        }
    }
}

pub struct PortfolioFactory;

impl PortfolioFactory {
    pub fn stub() -> Portfolio {
        Portfolio::Stub
    }

    pub fn in_memory_builder() -> InMemoryPortfolioBuilder {
        InMemoryPortfolioBuilder::new()
    }
}
