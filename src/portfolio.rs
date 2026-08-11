use hashbrown::HashMap;
use rust_decimal::Decimal;
use stock_trek_types::cex::{
    account_id::AccountId, asset_id::AssetId, cex_id::CexId, order_request::OrderRequest,
    pricing::Pricing, quantity::Quantity, side::Side, tag::Tag, trading_pair::TradingPair,
};

#[derive(Debug, Clone)]
pub struct Portfolio {
    market_prices: HashMap<CexId, HashMap<TradingPair, Decimal>>,
    portfolios: HashMap<CexId, HashMap<AccountId, CexAccountPortfolio>>,
}

#[derive(Debug, Clone)]
struct CexAccountPortfolio {
    asset_counts: HashMap<AssetId, Decimal>,
    pending_orders: HashMap<Tag, Vec<PendingOrder>>,
}

#[derive(Debug, Clone)]
struct PendingOrder {
    order_request: OrderRequest<AssetId, Decimal>,
    filled_quantity: Decimal,
}

// impl Portfolio {
//     pub fn pending_orders(&self) -> f64 {}
//     pub fn pending_orders_with_tag(&self, tag: &Tag) -> f64 {}
//     pub fn pending_orders_in_cex_account(&self, cex_id: &CexId, account_id: &AccountId) -> f64 {}
//     pub fn pending_orders_in_cex_account_with_tag(
//         &self,
//         cex_id: &CexId,
//         account_id: &AccountId,
//         tag: &Tag,
//     ) -> f64 {
//     }
//     pub fn reserved(&self, asset_id: &AssetId) -> f64 {}
//     pub fn reserved_in_cex_account(
//         &self,
//         asset_id: &AssetId,
//         cex_id: &CexId,
//         account_id: &AccountId,
//     ) -> f64 {
//     }
//     pub fn available(&self, asset_id: &AssetId) -> f64 {}
//     pub fn available_in_cex_account(
//         &self,
//         asset_id: &AssetId,
//         cex_id: &CexId,
//         account_id: &AccountId,
//     ) -> f64 {
//     }
// }

impl Portfolio {
    /// Whether an account exists
    pub fn has_cex_account(&self, cex_id: &CexId, account_id: &AccountId) -> bool {
        self.portfolios
            .get(cex_id)
            .map(|map| map.contains_key(account_id))
            .unwrap_or_default()
    }

    /// Total number of pending orders across all CEXes and accounts.
    pub fn pending_orders(&self) -> f64 {
        self.portfolios
            .values()
            .flat_map(|account_map| account_map.values())
            .flat_map(|portfolio| portfolio.pending_orders.values())
            .map(|orders| orders.len())
            .sum::<usize>() as f64
    }
    /// Number of pending orders with the given `tag` across all accounts.
    pub fn pending_orders_with_tag(&self, tag: &Tag) -> f64 {
        self.portfolios
            .values()
            .flat_map(|account_map| account_map.values())
            .filter_map(|portfolio| portfolio.pending_orders.get(tag))
            .map(|orders| orders.len())
            .sum::<usize>() as f64
    }
    /// Number of pending orders in a specific CEX account.
    pub fn pending_orders_in_cex_account(&self, cex_id: &CexId, account_id: &AccountId) -> f64 {
        self.portfolios
            .get(cex_id)
            .and_then(|accounts| accounts.get(account_id))
            .map(|portfolio| {
                portfolio
                    .pending_orders
                    .values()
                    .map(|orders| orders.len())
                    .sum::<usize>()
            })
            .unwrap_or(0) as f64
    }
    /// Number of pending orders with the given `tag` in a specific CEX account.
    pub fn pending_orders_in_cex_account_with_tag(
        &self,
        cex_id: &CexId,
        account_id: &AccountId,
        tag: &Tag,
    ) -> f64 {
        self.portfolios
            .get(cex_id)
            .and_then(|accounts| accounts.get(account_id))
            .and_then(|portfolio| portfolio.pending_orders.get(tag))
            .map(|orders| orders.len())
            .unwrap_or(0) as f64
    }

    /// Total `asset_id` held across **all** accounts (counts available and reserved).
    pub fn asset_total(&self, asset_id: &AssetId) -> Decimal {
        self.portfolios
            .values()
            .flat_map(|accounts| accounts.values())
            .flat_map(|account| account.asset_counts.get(asset_id))
            .copied()
            .sum()
    }
    /// `asset_id` held in a **specific** CEX account (counts available and reserved).
    pub fn asset_total_in_cex_account(
        &self,
        asset_id: &AssetId,
        cex_id: &CexId,
        account_id: &AccountId,
    ) -> Decimal {
        self.portfolios
            .get(cex_id)
            .and_then(|accounts| accounts.get(account_id))
            .and_then(|account| account.asset_counts.get(asset_id))
            .copied()
            .unwrap_or(Decimal::ZERO)
    }
    /// Total available (free) amount of `asset_id` across all accounts (total - reserved).
    pub fn asset_available(&self, asset_id: &AssetId) -> Decimal {
        self.asset_total(asset_id) - self.asset_reserved(asset_id)
    }
    /// Available (free) amount of `asset_id` in a specific CEX account (total - reserved).
    pub fn asset_available_in_cex_account(
        &self,
        asset_id: &AssetId,
        cex_id: &CexId,
        account_id: &AccountId,
    ) -> Decimal {
        self.asset_total_in_cex_account(asset_id, cex_id, account_id)
            - self.asset_reserved_in_cex_account(asset_id, cex_id, account_id)
    }
    /// Total amount of `asset_id` reserved (locked in unfilled pending orders) across all accounts.
    pub fn asset_reserved(&self, asset_id: &AssetId) -> Decimal {
        let mut total = Decimal::ZERO;
        for (cex_id, accounts) in &self.portfolios {
            for account in accounts.values() {
                for pending_orders in account.pending_orders.values() {
                    for order in pending_orders {
                        total += self.reserved_quantity_for_pending_order(cex_id, order, asset_id);
                    }
                }
            }
        }
        total
    }
    /// Amount of `asset_id` reserved (locked in unfilled pending orders) in a specific CEX account.
    pub fn asset_reserved_in_cex_account(
        &self,
        asset_id: &AssetId,
        cex_id: &CexId,
        account_id: &AccountId,
    ) -> Decimal {
        self.portfolios
            .get(cex_id)
            .and_then(|accounts| accounts.get(account_id))
            .map(|portfolio| {
                portfolio
                    .pending_orders
                    .values()
                    .flatten()
                    .map(|order| self.reserved_quantity_for_pending_order(cex_id, order, asset_id))
                    .sum::<Decimal>()
            })
            .unwrap_or(Decimal::ZERO)
    }

    fn reserved_quantity_for_pending_order(
        &self,
        cex_id: &CexId,
        pending_order: &PendingOrder,
        asset_id: &AssetId,
    ) -> Decimal {
        match &pending_order.order_request {
            OrderRequest::Single(single_order) => {
                // quantity multiple (total - filled)
                let quantity_multiple = match &single_order.quantity {
                    Quantity::OfBase(amount) | Quantity::OfQuote(amount) => {
                        if *amount <= pending_order.filled_quantity {
                            Decimal::ZERO
                        } else {
                            *amount - pending_order.filled_quantity
                        }
                    }
                };
                if quantity_multiple <= Decimal::ZERO {
                    return Decimal::ZERO;
                }

                // price multiple to convert remaining quantity to the asset we're reserving
                let trading_pair = TradingPair::new(single_order.base, single_order.quote);
                let price_multiple = match single_order.side {
                    Side::Sell if single_order.base == *asset_id => {
                        // Reserving base, convert if order quantity is in quote
                        match &single_order.quantity {
                            Quantity::OfBase(..) => Decimal::ONE,
                            Quantity::OfQuote(..) => {
                                let price =
                                    self.price(cex_id, &trading_pair, &single_order.pricing);
                                if price <= Decimal::ZERO {
                                    Decimal::ZERO
                                } else {
                                    Decimal::ONE / price
                                }
                            }
                        }
                    }
                    Side::Buy if single_order.quote == *asset_id => {
                        // Reserving quote, convert if order quantity is in base
                        match &single_order.quantity {
                            Quantity::OfQuote(..) => Decimal::ONE,
                            Quantity::OfBase(..) => {
                                self.price(cex_id, &trading_pair, &single_order.pricing)
                            }
                        }
                    }
                    _ => Decimal::ZERO, // asset not involved
                };

                if price_multiple <= Decimal::ZERO {
                    Decimal::ZERO
                } else {
                    price_multiple * quantity_multiple
                }
            }
        }
    }
    fn price(
        &self,
        cex_id: &CexId,
        trading_pair: &TradingPair,
        pricing: &Pricing<Decimal>,
    ) -> Decimal {
        match pricing {
            Pricing::Limit { price, .. } => *price,
            Pricing::Market => self
                .market_prices
                .get(cex_id)
                .map(|market| market.get(trading_pair).copied().unwrap_or_default())
                .unwrap_or_default(),
        }
    }
}
