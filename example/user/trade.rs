use rust_decimal::Decimal;

use crate::data::{flags::MarketSide, timestamp::TimestampMillis};

#[derive(Debug)]
pub struct Trade {
    client_order_id: String,
    timestamp_nanos: TimestampMillis,
    base_currency: String,
    side: MarketSide,
    price: Decimal,
    quantity: Decimal,
    quote_quantity: Decimal,
    fee: Decimal,
    fee_currency: String,
    is_maker: bool,
}

impl Trade {
    pub fn new(
        client_order_id: impl Into<String>,
        timestamp_nanos: TimestampMillis,
        base_currency: impl Into<String>,
        side: MarketSide,
        price: Decimal,
        quantity: Decimal,
        fee: Decimal,
        fee_currency: impl Into<String>,
        is_maker: bool,
    ) -> Self {
        let price = price;
        let quantity = quantity;
        let quote_quantity = price * quantity;
        Self {
            client_order_id: client_order_id.into(),
            timestamp_nanos,
            base_currency: base_currency.into(),
            side,
            price,
            quantity,
            quote_quantity,
            fee,
            fee_currency: fee_currency.into(),
            is_maker,
        }
    }
    pub fn client_order_id(&self) -> &str {
        &self.client_order_id
    }
    pub fn timestamp_nanos(&self) -> TimestampMillis {
        self.timestamp_nanos
    }
    pub fn base_currency(&self) -> &str {
        &self.base_currency
    }
    pub fn side(&self) -> MarketSide {
        self.side
    }
    pub fn price(&self) -> Decimal {
        self.price
    }
    pub fn quantity(&self) -> Decimal {
        self.quantity
    }
    pub fn quote_quantity(&self) -> Decimal {
        self.quote_quantity
    }
    pub fn fee(&self) -> Decimal {
        self.fee
    }
    pub fn fee_currency(&self) -> &str {
        &self.fee_currency
    }
    pub fn is_maker(&self) -> bool {
        self.is_maker
    }
}
