use crate::{
    error::result::StockTrekResult,
    resolved_context::ResolvedContext,
    signal::key::SignalKey,
    value::{binary_operator::BinaryOperator, unary_operator::UnaryOperator},
};
use serde::{Deserialize, Serialize};
use stock_trek_types::cex::{account_id::AccountId, asset_id::AssetId, cex_id::CexId, tag::Tag};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AccountIdValue {
    Literal { literal: AccountId },
    Signal { signal: SignalKey<AccountId> },
}

impl AccountIdValue {
    pub fn account_id(&self, c: &ResolvedContext) -> StockTrekResult<AccountId> {
        match self {
            Self::Literal { literal } => Ok(literal.clone()),
            Self::Signal { signal } => signal.read(c),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum CexIdValue {
    Literal { literal: CexId },
    Signal { signal: SignalKey<CexId> },
}

impl CexIdValue {
    pub fn cex_id(&self, c: &ResolvedContext) -> StockTrekResult<CexId> {
        match self {
            Self::Literal { literal } => Ok(*literal),
            Self::Signal { signal } => signal.read(c),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AssetIdValue {
    Literal { literal: AssetId },
    Signal { signal: SignalKey<AssetId> },
}

impl AssetIdValue {
    pub fn asset_id(&self, c: &ResolvedContext) -> StockTrekResult<AssetId> {
        match self {
            Self::Literal { literal } => Ok(*literal),
            Self::Signal { signal } => signal.read(c),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum FlagValue {
    Literal { literal: bool },
    Signal { signal: SignalKey<bool> },
}

impl FlagValue {
    pub fn flag(&self, c: &ResolvedContext) -> StockTrekResult<bool> {
        match self {
            Self::Literal { literal } => Ok(*literal),
            Self::Signal { signal } => signal.read(c),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum NumberValue {
    Literal {
        literal: f64,
    },
    Signal {
        signal: SignalKey<f64>,
    },
    ActiveOrders,
    ActiveOrdersWithTag {
        tag: Tag,
    },
    ActiveOrdersInCexAccount {
        cex_id_value: CexIdValue,
        account_id_value: AccountIdValue,
    },
    ActiveOrdersInCexAccountWithTag {
        cex_id_value: CexIdValue,
        account_id_value: AccountIdValue,
        tag: Tag,
    },
    AllocationForAsset {
        asset_id_value: AssetIdValue,
    },
    AllocationForAssetInCexAccount {
        cex_id_value: CexIdValue,
        account_id_value: AccountIdValue,
        asset_id_value: AssetIdValue,
    },
    AssetTotal {
        asset_id_value: AssetIdValue,
    },
    AssetInCexAccount {
        cex_id_value: CexIdValue,
        account_id_value: AccountIdValue,
        asset_id_value: AssetIdValue,
    },
    BinaryCalculation {
        left: Box<NumberValue>,
        operator: BinaryOperator,
        right: Box<NumberValue>,
    },
    UnaryCalculation {
        operator: UnaryOperator,
        number: Box<NumberValue>,
    },
}

impl NumberValue {
    pub fn number(&self, c: &ResolvedContext) -> StockTrekResult<f64> {
        match self {
            Self::Literal { literal } => Ok(*literal),
            Self::Signal { signal } => signal.read(c),
            Self::ActiveOrders => Ok(c.portfolio.active_orders()),
            Self::ActiveOrdersWithTag { tag } => Ok(c.portfolio.active_orders_with_tag(tag)),
            Self::ActiveOrdersInCexAccount {
                cex_id_value,
                account_id_value,
            } => {
                let cex_id = cex_id_value.cex_id(c)?;
                let account_id = account_id_value.account_id(c)?;
                Ok(c.portfolio
                    .active_orders_in_cex_account(&account_id, &cex_id))
            }
            Self::ActiveOrdersInCexAccountWithTag {
                cex_id_value,
                account_id_value,
                tag,
            } => {
                let cex_id = cex_id_value.cex_id(c)?;
                let account_id = account_id_value.account_id(c)?;
                Ok(c.portfolio
                    .active_orders_in_cex_account_with_tag(&account_id, &cex_id, tag))
            }
            Self::AllocationForAsset { asset_id_value } => {
                let asset_id = asset_id_value.asset_id(c)?;
                Ok(c.allocation.allocation_for_asset_total(&asset_id))
            }
            Self::AllocationForAssetInCexAccount {
                cex_id_value,
                account_id_value,
                asset_id_value,
            } => {
                let cex_id = cex_id_value.cex_id(c)?;
                let account_id = account_id_value.account_id(c)?;
                let asset_id = asset_id_value.asset_id(c)?;
                Ok(c.allocation.allocation_for_asset_in_cex_account(
                    &asset_id,
                    &cex_id,
                    &account_id,
                ))
            }
            Self::AssetTotal { asset_id_value } => {
                let asset_id = asset_id_value.asset_id(c)?;
                Ok(c.portfolio.asset_total(&asset_id))
            }
            Self::AssetInCexAccount {
                cex_id_value,
                account_id_value,
                asset_id_value,
            } => {
                let cex_id = cex_id_value.cex_id(c)?;
                let account_id = account_id_value.account_id(c)?;
                let asset_id = asset_id_value.asset_id(c)?;
                Ok(c.portfolio
                    .asset_in_cex_account(&asset_id, &account_id, &cex_id))
            }
            Self::BinaryCalculation {
                left,
                operator,
                right,
            } => {
                let left_value = left.number(c)?;
                let right_value = right.number(c)?;
                operator.calculate(left_value, right_value)
            }
            Self::UnaryCalculation { number, operator } => {
                let value = number.number(c)?;
                operator.calculate(value)
            }
        }
    }
}
