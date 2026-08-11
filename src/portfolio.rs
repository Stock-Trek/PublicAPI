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
pub struct CexAccountPortfolio {
    asset_counts: HashMap<AssetId, Decimal>,
    pending_orders: HashMap<Tag, Vec<PendingOrder>>,
}

#[derive(Debug, Clone)]
pub struct PendingOrder {
    order_request: OrderRequest<AssetId, Decimal>,
    filled_quantity: Decimal,
}

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

#[derive(Debug, Clone)]
pub struct PortfolioBuilder {
    market_prices: HashMap<CexId, HashMap<TradingPair, Decimal>>,
    portfolios: HashMap<CexId, HashMap<AccountId, CexAccountPortfolio>>,
}

impl PortfolioBuilder {
    pub fn new() -> Self {
        Self {
            market_prices: HashMap::new(),
            portfolios: HashMap::new(),
        }
    }
}

impl Default for PortfolioBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl PortfolioBuilder {
    pub fn market_price(
        &mut self,
        cex_id: CexId,
        trading_pair: TradingPair,
        price: Decimal,
    ) -> &mut Self {
        self.market_prices
            .entry(cex_id)
            .or_default()
            .insert(trading_pair, price);
        self
    }

    pub fn asset_count(
        &mut self,
        cex_id: CexId,
        account_id: AccountId,
        asset_id: AssetId,
        count: Decimal,
    ) -> &mut Self {
        self.portfolios
            .entry(cex_id)
            .or_default()
            .entry(account_id)
            .or_insert_with(|| CexAccountPortfolio {
                asset_counts: HashMap::new(),
                pending_orders: HashMap::new(),
            })
            .asset_counts
            .insert(asset_id, count);
        self
    }

    pub fn pending_order(
        &mut self,
        cex_id: CexId,
        account_id: AccountId,
        tag: Tag,
        order_request: OrderRequest<AssetId, Decimal>,
        filled_quantity: Decimal,
    ) -> &mut Self {
        self.portfolios
            .entry(cex_id)
            .or_default()
            .entry(account_id)
            .or_insert_with(|| CexAccountPortfolio {
                asset_counts: HashMap::new(),
                pending_orders: HashMap::new(),
            })
            .pending_orders
            .entry(tag)
            .or_default()
            .push(PendingOrder {
                order_request,
                filled_quantity,
            });
        self
    }

    pub fn build(&self) -> Portfolio {
        Portfolio {
            market_prices: self.market_prices.clone(),
            portfolios: self.portfolios.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stock_trek_types::cex::{
        activation::Activation, orders::single_order::SingleOrder, pricing::Pricing,
        quantity::Quantity, side::Side, time_in_force::TimeInForce,
    };

    const CEX: CexId = CexId::Binance;
    const BTC: AssetId = AssetId::Bitcoin;
    const USDC: AssetId = AssetId::USDCoin;

    fn account() -> AccountId {
        AccountId::new("account-1")
    }

    fn account_2() -> AccountId {
        AccountId::new("account-2")
    }

    fn single_order(
        side: Side,
        quantity: Quantity<Decimal>,
        pricing: Pricing<Decimal>,
    ) -> OrderRequest<AssetId, Decimal> {
        OrderRequest::Single(SingleOrder {
            base: BTC,
            quote: USDC,
            activation: Activation::Immediate,
            pricing,
            side,
            quantity,
            tag: Tag::new("test"),
        })
    }

    fn limit_order(
        side: Side,
        quantity: Quantity<Decimal>,
        price: Decimal,
    ) -> OrderRequest<AssetId, Decimal> {
        single_order(
            side,
            quantity,
            Pricing::Limit {
                price,
                time_in_force: TimeInForce::GoodTillCancelled,
            },
        )
    }

    fn market_order(side: Side, quantity: Quantity<Decimal>) -> OrderRequest<AssetId, Decimal> {
        single_order(side, quantity, Pricing::Market)
    }

    #[test]
    fn empty_portfolio_defaults() {
        let portfolio = PortfolioBuilder::new().build();

        assert!(!portfolio.has_cex_account(&CEX, &account()));
        assert_eq!(portfolio.pending_orders(), 0.0);
        assert_eq!(portfolio.asset_total(&BTC), Decimal::ZERO);
        assert_eq!(portfolio.asset_available(&BTC), Decimal::ZERO);
        assert_eq!(portfolio.asset_reserved(&BTC), Decimal::ZERO);
    }

    #[test]
    fn asset_counts_are_queryable() {
        let mut builder = PortfolioBuilder::new();
        builder
            .asset_count(CEX, account(), BTC, Decimal::new(2, 0))
            .asset_count(CEX, account(), USDC, Decimal::new(500, 0))
            .asset_count(CEX, account_2(), BTC, Decimal::new(3, 0));
        let portfolio = builder.build();

        assert!(portfolio.has_cex_account(&CEX, &account()));
        assert!(portfolio.has_cex_account(&CEX, &account_2()));
        assert!(!portfolio.has_cex_account(&CEX, &AccountId::new("missing")));
        assert_eq!(portfolio.asset_total(&BTC), Decimal::new(5, 0));
        assert_eq!(
            portfolio.asset_total_in_cex_account(&BTC, &CEX, &account()),
            Decimal::new(2, 0)
        );
        assert_eq!(
            portfolio.asset_total_in_cex_account(&BTC, &CEX, &account_2()),
            Decimal::new(3, 0)
        );
        assert_eq!(portfolio.asset_total(&AssetId::Ethereum), Decimal::ZERO);
        assert_eq!(portfolio.asset_available(&BTC), Decimal::new(5, 0));
        assert_eq!(
            portfolio.asset_available_in_cex_account(&BTC, &CEX, &account()),
            Decimal::new(2, 0)
        );
    }

    #[test]
    fn pending_orders_are_queryable() {
        let order = limit_order(
            Side::Sell,
            Quantity::OfBase(Decimal::new(1, 0)),
            Decimal::new(100, 0),
        );
        let mut builder = PortfolioBuilder::new();
        builder
            .pending_order(CEX, account(), Tag::new("t1"), order.clone(), Decimal::ZERO)
            .pending_order(CEX, account(), Tag::new("t1"), order.clone(), Decimal::ZERO)
            .pending_order(CEX, account_2(), Tag::new("t2"), order, Decimal::ZERO);
        let portfolio = builder.build();

        assert_eq!(portfolio.pending_orders(), 3.0);
        assert_eq!(portfolio.pending_orders_with_tag(&Tag::new("t1")), 2.0);
        assert_eq!(portfolio.pending_orders_with_tag(&Tag::new("missing")), 0.0);
        assert_eq!(
            portfolio.pending_orders_in_cex_account(&CEX, &account()),
            2.0
        );
        assert_eq!(
            portfolio.pending_orders_in_cex_account(&CEX, &account_2()),
            1.0
        );
        assert_eq!(
            portfolio.pending_orders_in_cex_account_with_tag(&CEX, &account(), &Tag::new("t1")),
            2.0
        );
        assert_eq!(
            portfolio.pending_orders_in_cex_account_with_tag(&CEX, &account_2(), &Tag::new("t1")),
            0.0
        );
    }

    #[test]
    fn sell_of_base_reserves_base() {
        let mut builder = PortfolioBuilder::new();
        builder.pending_order(
            CEX,
            account(),
            Tag::new("t"),
            limit_order(
                Side::Sell,
                Quantity::OfBase(Decimal::new(2, 0)),
                Decimal::new(100, 0),
            ),
            Decimal::ZERO,
        );
        let portfolio = builder.build();

        assert_eq!(portfolio.asset_reserved(&BTC), Decimal::new(2, 0));
        assert_eq!(
            portfolio.asset_reserved_in_cex_account(&BTC, &CEX, &account()),
            Decimal::new(2, 0)
        );
        // A sell does not reserve the quote asset.
        assert_eq!(portfolio.asset_reserved(&USDC), Decimal::ZERO);
    }

    #[test]
    fn buy_of_quote_reserves_quote() {
        let mut builder = PortfolioBuilder::new();
        builder.pending_order(
            CEX,
            account(),
            Tag::new("t"),
            limit_order(
                Side::Buy,
                Quantity::OfQuote(Decimal::new(500, 0)),
                Decimal::new(100, 0),
            ),
            Decimal::ZERO,
        );
        let portfolio = builder.build();

        assert_eq!(portfolio.asset_reserved(&USDC), Decimal::new(500, 0));
        assert_eq!(portfolio.asset_reserved(&BTC), Decimal::ZERO);
    }

    #[test]
    fn buy_of_base_reserves_quote_at_limit_price() {
        let mut builder = PortfolioBuilder::new();
        builder.pending_order(
            CEX,
            account(),
            Tag::new("t"),
            limit_order(
                Side::Buy,
                Quantity::OfBase(Decimal::new(2, 0)),
                Decimal::new(100, 0),
            ),
            Decimal::ZERO,
        );
        let portfolio = builder.build();

        assert_eq!(portfolio.asset_reserved(&USDC), Decimal::new(200, 0));
        assert_eq!(portfolio.asset_reserved(&BTC), Decimal::ZERO);
    }

    #[test]
    fn sell_of_quote_reserves_base_at_market_price() {
        let mut builder = PortfolioBuilder::new();
        builder
            .market_price(CEX, TradingPair::new(BTC, USDC), Decimal::new(100, 0))
            .pending_order(
                CEX,
                account(),
                Tag::new("t"),
                market_order(Side::Sell, Quantity::OfQuote(Decimal::new(500, 0))),
                Decimal::ZERO,
            );
        let portfolio = builder.build();

        assert_eq!(portfolio.asset_reserved(&BTC), Decimal::new(5, 0));
    }

    #[test]
    fn market_order_without_price_reserves_nothing() {
        let mut builder = PortfolioBuilder::new();
        builder.pending_order(
            CEX,
            account(),
            Tag::new("t"),
            market_order(Side::Sell, Quantity::OfQuote(Decimal::new(500, 0))),
            Decimal::ZERO,
        );
        let portfolio = builder.build();

        assert_eq!(portfolio.asset_reserved(&BTC), Decimal::ZERO);
    }

    #[test]
    fn limit_order_uses_limit_price_even_when_market_price_is_set() {
        let mut builder = PortfolioBuilder::new();
        builder
            .market_price(CEX, TradingPair::new(BTC, USDC), Decimal::new(100, 0))
            .pending_order(
                CEX,
                account(),
                Tag::new("t"),
                limit_order(
                    Side::Buy,
                    Quantity::OfBase(Decimal::new(2, 0)),
                    Decimal::new(50, 0),
                ),
                Decimal::ZERO,
            );
        let portfolio = builder.build();

        assert_eq!(portfolio.asset_reserved(&USDC), Decimal::new(100, 0));
    }

    #[test]
    fn filled_quantity_reduces_reserved() {
        let mut builder = PortfolioBuilder::new();
        builder.pending_order(
            CEX,
            account(),
            Tag::new("t"),
            limit_order(
                Side::Sell,
                Quantity::OfBase(Decimal::new(2, 0)),
                Decimal::new(100, 0),
            ),
            Decimal::new(15, 1), // 1.5 filled
        );
        let portfolio = builder.build();

        assert_eq!(portfolio.asset_reserved(&BTC), Decimal::new(5, 1));
    }

    #[test]
    fn fully_filled_order_reserves_nothing() {
        let mut builder = PortfolioBuilder::new();
        builder.pending_order(
            CEX,
            account(),
            Tag::new("t"),
            limit_order(
                Side::Sell,
                Quantity::OfBase(Decimal::new(2, 0)),
                Decimal::new(100, 0),
            ),
            Decimal::new(2, 0),
        );
        let portfolio = builder.build();

        assert_eq!(portfolio.asset_reserved(&BTC), Decimal::ZERO);
    }
}
