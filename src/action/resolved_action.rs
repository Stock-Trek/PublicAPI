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
    // CancelAllOrdersInCex {
    //     cex_id: CexId,
    // },
    // CancelAllOrdersInCexWithTag {
    //     cex_id: CexId,
    //     tag: Tag,
    // },
    PlaceOrder {
        cex_id: CexId,
        order_request: OrderRequest<AssetId, f64>,
    },
    PlaceOrderInAccount {
        cex_id: CexId,
        account_id: AccountId,
        order_request: OrderRequest<AssetId, f64>,
    },
}
