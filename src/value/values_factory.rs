use crate::{
    signal::key::SignalKey,
    value::{
        binary_operator::BinaryOperator,
        unary_operator::UnaryOperator,
        value::{AccountIdValue, AssetIdValue, CexIdValue, FlagValue, NumberValue},
    },
};
use stock_trek_types::cex::{account_id::AccountId, asset_id::AssetId, cex_id::CexId, tag::Tag};

#[derive(Debug, Clone)]
pub struct AllocationValuesFactory;
#[derive(Debug, Clone)]
pub struct PortfolioValuesFactory;
#[derive(Debug, Clone)]
pub struct CalculationValuesFactory;
#[derive(Debug, Clone)]
pub struct LiteralValuesFactory;
#[derive(Debug, Clone)]
pub struct SignalValuesFactory;

impl AllocationValuesFactory {
    pub fn allocation_for_asset(&self, asset_id_value: AssetIdValue) -> NumberValue {
        NumberValue::AllocationForAsset { asset_id_value }
    }
    pub fn allocation_for_asset_in_cex_account(
        &self,
        cex_id_value: CexIdValue,
        account_id_value: AccountIdValue,
        asset_id_value: AssetIdValue,
    ) -> NumberValue {
        NumberValue::AllocationForAssetInCexAccount {
            cex_id_value,
            account_id_value,
            asset_id_value,
        }
    }
}

impl PortfolioValuesFactory {
    pub fn active_orders(&self) -> NumberValue {
        NumberValue::ActiveOrders
    }
    pub fn active_orders_with_tag(&self, tag: Tag) -> NumberValue {
        NumberValue::ActiveOrdersWithTag { tag }
    }
    pub fn active_orders_in_cex_account(
        &self,
        cex_id_value: CexIdValue,
        account_id_value: AccountIdValue,
    ) -> NumberValue {
        NumberValue::ActiveOrdersInCexAccount {
            cex_id_value,
            account_id_value,
        }
    }
    pub fn active_orders_in_cex_account_with_tag(
        &self,
        cex_id_value: CexIdValue,
        account_id_value: AccountIdValue,
        tag: Tag,
    ) -> NumberValue {
        NumberValue::ActiveOrdersInCexAccountWithTag {
            cex_id_value,
            account_id_value,
            tag,
        }
    }
    pub fn asset_total(&self, asset_id_value: AssetIdValue) -> NumberValue {
        NumberValue::AssetTotal { asset_id_value }
    }
    pub fn asset_in_cex_account(
        &self,
        cex_id_value: CexIdValue,
        account_id_value: AccountIdValue,
        asset_id_value: AssetIdValue,
    ) -> NumberValue {
        NumberValue::AssetInCexAccount {
            cex_id_value,
            account_id_value,
            asset_id_value,
        }
    }
}

impl CalculationValuesFactory {
    pub fn binary(
        &self,
        left: NumberValue,
        operator: BinaryOperator,
        right: NumberValue,
    ) -> NumberValue {
        NumberValue::BinaryCalculation {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
    pub fn unary(&self, operator: UnaryOperator, number: NumberValue) -> NumberValue {
        NumberValue::UnaryCalculation {
            number: Box::new(number),
            operator,
        }
    }
}

impl LiteralValuesFactory {
    pub fn cex_id(&self, literal: CexId) -> CexIdValue {
        CexIdValue::Literal { literal }
    }
    pub fn account_id(&self, literal: AccountId) -> AccountIdValue {
        AccountIdValue::Literal { literal }
    }
    pub fn asset_id(&self, literal: AssetId) -> AssetIdValue {
        AssetIdValue::Literal { literal }
    }
    pub fn flag(&self, literal: bool) -> FlagValue {
        FlagValue::Literal { literal }
    }
    pub fn number(&self, literal: f64) -> NumberValue {
        NumberValue::Literal { literal }
    }
}

impl SignalValuesFactory {
    pub fn cex_id(&self, key: &SignalKey<CexId>) -> CexIdValue {
        CexIdValue::Signal {
            signal: key.clone(),
        }
    }
    pub fn account_id(&self, key: &SignalKey<AccountId>) -> AccountIdValue {
        AccountIdValue::Signal {
            signal: key.clone(),
        }
    }
    pub fn asset_id(&self, key: &SignalKey<AssetId>) -> AssetIdValue {
        AssetIdValue::Signal {
            signal: key.clone(),
        }
    }
    pub fn flag(&self, key: &SignalKey<bool>) -> FlagValue {
        FlagValue::Signal {
            signal: key.clone(),
        }
    }
    pub fn number(&self, key: &SignalKey<f64>) -> NumberValue {
        NumberValue::Signal {
            signal: key.clone(),
        }
    }
}
