use rust_decimal::Decimal;

#[derive(Debug)]
pub struct Balance {
    currency: String,
    free: Decimal,
    locked: Decimal,
    total: Decimal,
}

impl Balance {
    pub fn new(currency: impl Into<String>, free: Decimal, locked: Decimal) -> Self {
        let total = free + locked;
        Self {
            currency: currency.into(),
            free,
            locked,
            total,
        }
    }
    pub fn currency(&self) -> &str {
        &self.currency
    }
    pub fn free(&self) -> Decimal {
        self.free
    }
    pub fn locked(&self) -> Decimal {
        self.locked
    }
    pub fn total(&self) -> Decimal {
        self.total
    }
}
