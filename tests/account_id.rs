use stock_trek::{
    ConditionFactory, InMemoryPortfolioBuilder, OrderFactory, Portfolio, ResolvedContext,
    actions::{ActionErrorResponse, ActionFactory, RecoveryPolicy, ResolvedAction},
    allocations::{Allocation, InMemoryAllocationBuilder},
    signals::{SignalKey, Signals},
    types::{AccountId, Activation, AssetId, CexId, Pricing, Quantity, Side, Tag},
    values::{AccountIdValue, AssetIdValue, CexIdValue, NumberValue},
};

fn context(portfolio: Portfolio, allocation: Allocation, signals: Signals) -> ResolvedContext {
    ResolvedContext {
        enqueue_action: Box::new(|_, _| Ok(())),
        allocation,
        portfolio,
        signals,
    }
}

#[test]
fn account_id_signal_key_reads_and_writes() {
    let key = SignalKey::<AccountId>::new_required("ACCOUNT");
    let mut signals = Signals::new();
    signals.write(&key, AccountId::new("spot-main"));
    assert_eq!(signals.read(&key).unwrap(), AccountId::new("spot-main"));

    // Reading with a key of a different type fails.
    let number_key = SignalKey::<f64>::new_required("ACCOUNT");
    assert!(signals.read(&number_key).is_err());
}

#[test]
fn account_id_value_literal_and_signal() {
    let c = context(Portfolio::Stub, Allocation::Stub, Signals::new());

    let literal = AccountIdValue::Literal {
        literal: AccountId::new("main"),
    };
    assert_eq!(literal.account_id(&c).unwrap(), AccountId::new("main"));

    let key = SignalKey::<AccountId>::new_required("ACCOUNT");
    let mut signals = Signals::new();
    signals.write(&key, AccountId::new("spot"));
    let c = context(Portfolio::Stub, Allocation::Stub, signals);

    let signal_value = AccountIdValue::Signal { signal: key };
    assert_eq!(signal_value.account_id(&c).unwrap(), AccountId::new("spot"));
}

#[test]
fn portfolio_account_queries() {
    let mut builder = InMemoryPortfolioBuilder::new();
    builder
        .assets_in_account(
            CexId::Binance,
            AccountId::new("main"),
            AssetId::Bitcoin,
            2.0,
        )
        .assets_in_account(CexId::Binance, AccountId::new("sub"), AssetId::Bitcoin, 3.0)
        .assets(CexId::Binance, AssetId::Ethereum, 5.0);
    let portfolio = builder.build();

    assert!(portfolio.has_account(&AccountId::new("main")));
    assert!(!portfolio.has_account(&AccountId::new("missing")));
    assert!(portfolio.has_account_in_cex(&CexId::Binance));

    assert!(portfolio.owns_asset_in_account(&AssetId::Bitcoin, &AccountId::new("main")));
    assert!(!portfolio.owns_asset_in_account(&AssetId::Ethereum, &AccountId::new("main")));
    assert!(portfolio.owns_asset_in_account_in_cex(
        &AssetId::Bitcoin,
        &AccountId::new("sub"),
        &CexId::Binance
    ));

    assert_eq!(
        portfolio.asset_in_account(&AssetId::Bitcoin, &AccountId::new("main")),
        2.0
    );
    assert_eq!(
        portfolio.asset_in_account_in_cex(
            &AssetId::Bitcoin,
            &AccountId::new("sub"),
            &CexId::Binance
        ),
        3.0
    );
    assert_eq!(
        portfolio.asset_in_cex(&AssetId::Bitcoin, &CexId::Binance),
        5.0
    );
    assert_eq!(portfolio.asset_total(&AssetId::Bitcoin), 5.0);
}

#[test]
fn allocation_account_queries() {
    let mut builder = InMemoryAllocationBuilder::new();
    builder
        .allocation_in_account(
            CexId::Binance,
            AccountId::new("main"),
            AssetId::Bitcoin,
            20.0,
        )
        .allocation_in_account(
            CexId::Binance,
            AccountId::new("sub"),
            AssetId::Bitcoin,
            30.0,
        );
    let allocation = builder.build();

    assert_eq!(
        allocation.allocation_for_asset_in_account(&AssetId::Bitcoin, &AccountId::new("main")),
        20.0
    );
    assert_eq!(
        allocation.allocation_for_asset_in_account_in_cex(
            &AssetId::Bitcoin,
            &AccountId::new("sub"),
            &CexId::Binance
        ),
        30.0
    );
    assert_eq!(
        allocation.allocation_for_asset_in_cex(&AssetId::Bitcoin, &CexId::Binance),
        50.0
    );
    assert_eq!(
        allocation.allocation_for_asset_total(&AssetId::Bitcoin),
        50.0
    );
}

#[test]
fn condition_account_queries() {
    let mut builder = InMemoryPortfolioBuilder::new();
    builder.assets_in_account(
        CexId::Binance,
        AccountId::new("main"),
        AssetId::Bitcoin,
        2.0,
    );
    let c = context(builder.build(), Allocation::Stub, Signals::new());
    let factory = ConditionFactory;

    assert!(
        factory
            .has_account(AccountId::new("main"))
            .test(&c)
            .unwrap()
    );
    assert!(
        !factory
            .has_account(AccountId::new("missing"))
            .test(&c)
            .unwrap()
    );
    assert!(
        factory
            .owns_asset_in_account(AssetId::Bitcoin, AccountId::new("main"))
            .test(&c)
            .unwrap()
    );
    assert!(
        factory
            .owns_asset_in_account_in_cex(AssetId::Bitcoin, AccountId::new("main"), CexId::Binance)
            .test(&c)
            .unwrap()
    );
    assert!(
        !factory
            .owns_asset_in_account(AssetId::Ethereum, AccountId::new("main"))
            .test(&c)
            .unwrap()
    );
}

#[test]
fn action_send_order_request_in_account_resolves() {
    let key_account = SignalKey::<AccountId>::new_required("ACCOUNT");
    let mut signals = Signals::new();
    signals.write(&key_account, AccountId::new("main"));

    let resolved: std::rc::Rc<std::cell::RefCell<Option<ResolvedAction>>> =
        std::rc::Rc::new(std::cell::RefCell::new(None));
    let resolved_capture = std::rc::Rc::clone(&resolved);
    let c = ResolvedContext {
        enqueue_action: Box::new(move |action, _| {
            *resolved_capture.borrow_mut() = Some(action.clone());
            Ok(())
        }),
        allocation: Allocation::Stub,
        portfolio: Portfolio::Stub,
        signals,
    };

    let order_request = OrderFactory.single(
        AssetIdValue::Literal {
            literal: AssetId::Bitcoin,
        },
        AssetIdValue::Literal {
            literal: AssetId::TetherUSD,
        },
        Side::Buy,
        Activation::Immediate,
        Pricing::Market,
        Quantity::OfQuote(NumberValue::Literal { literal: 10.0 }),
        Tag::new("account-test"),
    );

    let key_account = SignalKey::<AccountId>::new_required("ACCOUNT");
    let mut signals = Signals::new();
    signals.write(&key_account, AccountId::new("main"));

    ActionFactory
        .send_order_request_in_account(
            CexIdValue::Literal {
                literal: CexId::Binance,
            },
            AccountIdValue::Signal {
                signal: key_account,
            },
            order_request,
            RecoveryPolicy::with_default_response(ActionErrorResponse::Stop),
        )
        .enqueue(&c)
        .unwrap();

    let resolved = resolved.borrow();
    assert!(matches!(
        resolved.as_ref(),
        Some(ResolvedAction::PlaceOrderInAccount {
            cex_id: CexId::Binance,
            account_id,
            ..
        }) if *account_id == AccountId::new("main")
    ));
}
