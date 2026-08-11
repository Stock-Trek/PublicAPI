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
pub struct PortfolioValuesFactory;
#[derive(Debug, Clone)]
pub struct CalculationValuesFactory;
#[derive(Debug, Clone)]
pub struct LiteralValuesFactory;
#[derive(Debug, Clone)]
pub struct SignalValuesFactory;

impl PortfolioValuesFactory {
    pub fn pending_orders(&self) -> NumberValue {
        NumberValue::PendingOrders
    }
    pub fn pending_orders_with_tag(&self, tag: Tag) -> NumberValue {
        NumberValue::PendingOrdersWithTag { tag }
    }
    pub fn pending_orders_inc_cex_account(
        &self,
        cex_id_value: CexIdValue,
        account_id_value: AccountIdValue,
    ) -> NumberValue {
        NumberValue::PendingOrdersInCexAccount {
            cex_id_value,
            account_id_value,
        }
    }
    pub fn pending_orders_inc_cex_account_with_tag(
        &self,
        cex_id_value: CexIdValue,
        account_id_value: AccountIdValue,
        tag: Tag,
    ) -> NumberValue {
        NumberValue::PendingOrdersInCexAccountWithTag {
            cex_id_value,
            account_id_value,
            tag,
        }
    }
    pub fn asset_total(&self, asset_id_value: AssetIdValue) -> NumberValue {
        NumberValue::AssetTotal { asset_id_value }
    }
    pub fn asset_total_in_cex_account(
        &self,
        asset_id_value: AssetIdValue,
        cex_id_value: CexIdValue,
        account_id_value: AccountIdValue,
    ) -> NumberValue {
        NumberValue::AssetTotalInCexAccount {
            asset_id_value,
            cex_id_value,
            account_id_value,
        }
    }
    pub fn asset_available(&self, asset_id_value: AssetIdValue) -> NumberValue {
        NumberValue::AssetAvailable { asset_id_value }
    }
    pub fn asset_available_in_cex_account(
        &self,
        asset_id_value: AssetIdValue,
        cex_id_value: CexIdValue,
        account_id_value: AccountIdValue,
    ) -> NumberValue {
        NumberValue::AssetAvailableInCexAccount {
            asset_id_value,
            cex_id_value,
            account_id_value,
        }
    }
    pub fn asset_reserved(&self, asset_id_value: AssetIdValue) -> NumberValue {
        NumberValue::AssetReserved { asset_id_value }
    }
    pub fn asset_reserved_in_cex_account(
        &self,
        asset_id_value: AssetIdValue,
        cex_id_value: CexIdValue,
        account_id_value: AccountIdValue,
    ) -> NumberValue {
        NumberValue::AssetReservedInCexAccount {
            asset_id_value,
            cex_id_value,
            account_id_value,
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
