use crate::{
    data::{
        flags::MarketSide,
        market::market::Market,
        order_request::{OrderKind, OrderRequest, OrderTrigger},
        trading_pair::TradingPair,
        validation::{
            validate_client_order_id, validate_price, validate_quantity, validate_trading_pair,
        },
    },
    errors::StockTrekError,
};
use rust_decimal::Decimal;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Exchange {
    name: String,
    currencies: HashSet<String>,
    markets: HashMap<TradingPair, Market>,
}

impl Exchange {
    pub fn market(&self, trading_pair: &TradingPair) -> Result<&Market, StockTrekError> {
        self.markets.get(trading_pair).ok_or_else(|| {
            StockTrekError::Validation(crate::errors::ValidationError::DoesNotExist {
                name: trading_pair.to_string(),
                container: self.name.to_string(),
            })
        })
    }
    pub fn market_order(
        &self,
        client_order_id: &str,
        trading_pair: TradingPair,
        side: MarketSide,
        quantity: Decimal,
    ) -> Result<OrderRequest, StockTrekError> {
        let kind = OrderKind::Market;
        let trigger = OrderTrigger::None;
        self.validate_and_create_order(client_order_id, trading_pair, side, quantity, kind, trigger)
    }
    pub fn limit_order(
        &self,
        client_order_id: &str,
        trading_pair: TradingPair,
        side: MarketSide,
        quantity: Decimal,
        price: Decimal,
    ) -> Result<OrderRequest, StockTrekError> {
        let kind = OrderKind::Limit { price };
        let trigger = OrderTrigger::None;
        self.validate_and_create_order(client_order_id, trading_pair, side, quantity, kind, trigger)
    }
    pub fn stop_order(
        &self,
        client_order_id: &str,
        trading_pair: TradingPair,
        side: MarketSide,
        quantity: Decimal,
        stop_price: Decimal,
    ) -> Result<OrderRequest, StockTrekError> {
        let kind = OrderKind::Market;
        let trigger = OrderTrigger::Stop { stop_price };
        self.validate_and_create_order(client_order_id, trading_pair, side, quantity, kind, trigger)
    }
    pub fn stop_limit_order(
        &self,
        client_order_id: &str,
        trading_pair: TradingPair,
        side: MarketSide,
        quantity: Decimal,
        price: Decimal,
        stop_price: Decimal,
    ) -> Result<OrderRequest, StockTrekError> {
        let kind = OrderKind::Limit { price };
        let trigger = OrderTrigger::Stop { stop_price };
        self.validate_and_create_order(client_order_id, trading_pair, side, quantity, kind, trigger)
    }

    fn validate_and_create_order(
        &self,
        client_order_id: &str,
        trading_pair: TradingPair,
        side: MarketSide,
        quantity: Decimal,
        kind: OrderKind,
        trigger: OrderTrigger,
    ) -> Result<OrderRequest, StockTrekError> {
        validate_client_order_id(client_order_id)?;
        validate_trading_pair(&trading_pair, &self.currencies)?;
        let market = self.market(&trading_pair)?;
        let base_increment = market.base_increment();
        let quote_increment = market.quote_increment();
        validate_quantity(quantity, base_increment)?;
        match (&kind, &trigger) {
            (OrderKind::Market, OrderTrigger::None) => {}
            (OrderKind::Limit { price }, OrderTrigger::None) => {
                validate_price(*price, quote_increment)?;
            }
            (OrderKind::Market, OrderTrigger::Stop { stop_price }) => {
                validate_price(*stop_price, quote_increment)?;
            }
            (OrderKind::Limit { price }, OrderTrigger::Stop { stop_price }) => {
                validate_price(*price, quote_increment)?;
                validate_price(*stop_price, quote_increment)?;
            }
        }
        Ok(OrderRequest::new(
            client_order_id.into(),
            trading_pair,
            side,
            kind,
            trigger,
            quantity,
        ))
    }
}
