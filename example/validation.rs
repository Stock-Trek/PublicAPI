use crate::{
    data::trading_pair::TradingPair,
    errors::{StockTrekError, ValidationError},
};
use rust_decimal::Decimal;
use std::collections::HashSet;

pub fn validate_client_order_id(client_order_id: &str) -> Result<(), StockTrekError> {
    validate_string_is_not_empty("client_order_id", client_order_id)
}

pub fn validate_trading_pair(
    trading_pair: &TradingPair,
    currencies: &HashSet<String>,
) -> Result<(), StockTrekError> {
    validate_string_is_not_empty("base_currency", trading_pair.base_currency())?;
    validate_string_is_not_empty("quote_currency", trading_pair.quote_currency())?;
    validate_currency(trading_pair.base_currency(), currencies)?;
    validate_currency(trading_pair.quote_currency(), currencies)?;
    Ok(())
}

pub fn validate_price(price: Decimal, increment: Decimal) -> Result<(), StockTrekError> {
    let name = "price";
    validate_is_positive(name, price)?;
    validate_increment(name, price, increment)?;
    Ok(())
}

pub fn validate_quantity(quantity: Decimal, increment: Decimal) -> Result<(), StockTrekError> {
    let name = "quantity";
    validate_is_positive(name, quantity)?;
    validate_increment(name, quantity, increment)?;
    Ok(())
}

pub fn validate_remaining_quantity(remaining_quantity: Decimal) -> Result<(), StockTrekError> {
    validate_is_positive("remaining_quantity", remaining_quantity)
}

fn validate_currency(currency: &str, currencies: &HashSet<String>) -> Result<(), StockTrekError> {
    if !currencies.contains(currency) {
        return Err(StockTrekError::Validation(ValidationError::DoesNotExist {
            name: currency.into(),
            container: "currencies".into(),
        }));
    }
    Ok(())
}

fn validate_string_is_not_empty(name: &str, value: &str) -> Result<(), StockTrekError> {
    if value.is_empty() {
        return Err(StockTrekError::Validation(ValidationError::IsEmpty {
            name: name.to_string(),
        }));
    }
    Ok(())
}

fn validate_is_positive(name: &str, value: Decimal) -> Result<(), StockTrekError> {
    if value <= Decimal::ZERO {
        return Err(StockTrekError::Validation(ValidationError::IsNotPositive {
            name: name.to_string(),
            value,
        }));
    }
    Ok(())
}

fn validate_increment(
    name: &str,
    value: Decimal,
    increment: Decimal,
) -> Result<(), StockTrekError> {
    if value % increment != Decimal::ZERO {
        return Err(StockTrekError::Validation(
            ValidationError::InvalidIncrement {
                name: name.to_string(),
                value,
                increment,
            },
        ));
    }
    Ok(())
}
