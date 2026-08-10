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
    ActiveOrdersInAccount {
        account_id_value: AccountIdValue,
    },
    ActiveOrdersInAccountInCex {
        cex_id_value: CexIdValue,
        account_id_value: AccountIdValue,
    },
    ActiveOrdersInAccountInCexWithTag {
        cex_id_value: CexIdValue,
        account_id_value: AccountIdValue,
        tag: Tag,
    },
    ActiveOrdersInAccountWithTag {
        account_id_value: AccountIdValue,
        tag: Tag,
    },
    ActiveOrdersInCex {
        cex_id_value: CexIdValue,
    },
    ActiveOrdersInCexWithTag {
        cex_id_value: CexIdValue,
        tag: Tag,
    },
    ActiveOrdersWithTag {
        tag: Tag,
    },
    AllocationForAssetInAccount {
        account_id_value: AccountIdValue,
        asset_id_value: AssetIdValue,
    },
    AllocationForAssetInAccountInCex {
        cex_id_value: CexIdValue,
        account_id_value: AccountIdValue,
        asset_id_value: AssetIdValue,
    },
    AllocationForAssetInCex {
        cex_id_value: CexIdValue,
        asset_id_value: AssetIdValue,
    },
    AllocationForAssetTotal {
        asset_id_value: AssetIdValue,
    },
    AssetInAccount {
        account_id_value: AccountIdValue,
        asset_id_value: AssetIdValue,
    },
    AssetInAccountInCex {
        cex_id_value: CexIdValue,
        account_id_value: AccountIdValue,
        asset_id_value: AssetIdValue,
    },
    AssetInCex {
        cex_id_value: CexIdValue,
        asset_id_value: AssetIdValue,
    },
    AssetTotal {
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
            Self::ActiveOrdersInAccount { account_id_value } => {
                let account_id = account_id_value.account_id(c)?;
                Ok(c.portfolio.active_orders_in_account(&account_id))
            }
            Self::ActiveOrdersInAccountInCex {
                cex_id_value,
                account_id_value,
            } => {
                let cex_id = cex_id_value.cex_id(c)?;
                let account_id = account_id_value.account_id(c)?;
                Ok(c.portfolio
                    .active_orders_in_account_in_cex(&account_id, &cex_id))
            }
            Self::ActiveOrdersInAccountInCexWithTag {
                cex_id_value,
                account_id_value,
                tag,
            } => {
                let cex_id = cex_id_value.cex_id(c)?;
                let account_id = account_id_value.account_id(c)?;
                Ok(c.portfolio
                    .active_orders_in_account_in_cex_with_tag(&account_id, &cex_id, tag))
            }
            Self::ActiveOrdersInAccountWithTag {
                account_id_value,
                tag,
            } => {
                let account_id = account_id_value.account_id(c)?;
                Ok(c.portfolio
                    .active_orders_in_account_with_tag(&account_id, tag))
            }
            Self::ActiveOrdersInCex { cex_id_value } => {
                let cex_id = cex_id_value.cex_id(c)?;
                Ok(c.portfolio.active_orders_in_cex(&cex_id))
            }
            Self::ActiveOrdersInCexWithTag { cex_id_value, tag } => {
                let cex_id = cex_id_value.cex_id(c)?;
                Ok(c.portfolio.active_orders_in_cex_with_tag(&cex_id, tag))
            }
            Self::ActiveOrdersWithTag { tag } => Ok(c.portfolio.active_orders_with_tag(tag)),
            Self::AllocationForAssetInAccount {
                account_id_value,
                asset_id_value,
            } => {
                let account_id = account_id_value.account_id(c)?;
                let asset_id = asset_id_value.asset_id(c)?;
                Ok(c.allocation
                    .allocation_for_asset_in_account(&asset_id, &account_id))
            }
            Self::AllocationForAssetInAccountInCex {
                cex_id_value,
                account_id_value,
                asset_id_value,
            } => {
                let cex_id = cex_id_value.cex_id(c)?;
                let account_id = account_id_value.account_id(c)?;
                let asset_id = asset_id_value.asset_id(c)?;
                Ok(c.allocation.allocation_for_asset_in_account_in_cex(
                    &asset_id,
                    &account_id,
                    &cex_id,
                ))
            }
            Self::AllocationForAssetInCex {
                cex_id_value,
                asset_id_value,
            } => {
                let cex_id = cex_id_value.cex_id(c)?;
                let asset_id = asset_id_value.asset_id(c)?;
                Ok(c.allocation.allocation_for_asset_in_cex(&asset_id, &cex_id))
            }
            Self::AllocationForAssetTotal { asset_id_value } => {
                let asset_id = asset_id_value.asset_id(c)?;
                Ok(c.allocation.allocation_for_asset_total(&asset_id))
            }
            Self::AssetInAccount {
                account_id_value,
                asset_id_value,
            } => {
                let account_id = account_id_value.account_id(c)?;
                let asset_id = asset_id_value.asset_id(c)?;
                Ok(c.portfolio.asset_in_account(&asset_id, &account_id))
            }
            Self::AssetInAccountInCex {
                cex_id_value,
                account_id_value,
                asset_id_value,
            } => {
                let cex_id = cex_id_value.cex_id(c)?;
                let account_id = account_id_value.account_id(c)?;
                let asset_id = asset_id_value.asset_id(c)?;
                Ok(c.portfolio
                    .asset_in_account_in_cex(&asset_id, &account_id, &cex_id))
            }
            Self::AssetInCex {
                cex_id_value,
                asset_id_value,
            } => {
                let cex_id = cex_id_value.cex_id(c)?;
                let asset_id = asset_id_value.asset_id(c)?;
                Ok(c.portfolio.asset_in_cex(&asset_id, &cex_id))
            }
            Self::AssetTotal { asset_id_value } => {
                let asset_id = asset_id_value.asset_id(c)?;
                Ok(c.portfolio.asset_total(&asset_id))
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
