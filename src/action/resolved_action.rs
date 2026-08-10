use serde::{Deserialize, Serialize};
use stock_trek_types::cex::{
    account_id::AccountId, asset_id::AssetId, cex_id::CexId, order_request::OrderRequest,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResolvedAction {
    // CancelAllOrders,
    // CancelAllOrdersWithTag {
    //     tag: Tag,
    // },
    // CancelAllOrdersInCexAccount {
    //     cex_id: CexId,
    //     account_id: AccountId,
    // },
    // CancelAllOrdersInCexAccountWithTag {
    //     cex_id: CexId,
    //     account_id: AccountId,
    //     tag: Tag,
    // },
    PlaceOrder {
        cex_id: CexId,
        account_id: AccountId,
        order_request: OrderRequest<AssetId, f64>,
    },
}
