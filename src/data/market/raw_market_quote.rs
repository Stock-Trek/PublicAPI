use rust_decimal::Decimal;

#[derive(Debug, Clone, Copy)]
pub struct RawMarketQuote {
    price: Decimal,
    quantity: Decimal,
}

impl RawMarketQuote {
    pub fn price(&self) -> Decimal {
        self.price
    }
    pub fn quantity(&self) -> Decimal {
        self.quantity
    }
}

impl PartialEq for RawMarketQuote {
    fn eq(&self, other: &Self) -> bool {
        self.price.eq(&other.price) && self.quantity.eq(&other.quantity)
    }
}

impl PartialOrd for RawMarketQuote {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.price.partial_cmp(&other.price) {
            Some(std::cmp::Ordering::Equal) => self.quantity.partial_cmp(&other.quantity),
            price_ordering => price_ordering,
        }
    }
}
