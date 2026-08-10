use crate::{
    action::resolved_action::ResolvedAction,
    cex::capability::HasRequiredCapabilities,
    error::result::StockTrekResult,
    resolveable::Resolvable,
    resolved_context::ResolvedContext,
    value::value::{AccountIdValue, AssetIdValue, CexIdValue, NumberValue},
};
use serde::{Deserialize, Serialize};
use stock_trek_types::cex::{capability::CexCapability, order_request::OrderRequest};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    SendOrderRequest {
        cex_id_value: CexIdValue,
        order_request: Box<OrderRequest<AssetIdValue, NumberValue>>,
    },
    SendOrderRequestInAccount {
        cex_id_value: CexIdValue,
        account_id_value: AccountIdValue,
        order_request: Box<OrderRequest<AssetIdValue, NumberValue>>,
    },
    // CancelAllOrders,
    // CancelAllOrdersWithTag {
    //     tag: Tag,
    // },
    // CancelAllOrdersInCex {
    //     cex_id_value: CexIdValue,
    // },
    // CancelAllOrdersInCexWithTag {
    //     cex_id_value: CexIdValue,
    //     tag: Tag,
    // },
}

impl Resolvable<ResolvedAction> for Action {
    fn try_resolve(&self, c: &ResolvedContext) -> StockTrekResult<ResolvedAction> {
        match self {
            Action::SendOrderRequest {
                cex_id_value,
                order_request,
            } => Ok(ResolvedAction::PlaceOrder {
                cex_id: cex_id_value.cex_id(c)?,
                order_request: order_request.try_resolve(c)?,
            }),
            Action::SendOrderRequestInAccount {
                cex_id_value,
                account_id_value,
                order_request,
            } => Ok(ResolvedAction::PlaceOrderInAccount {
                cex_id: cex_id_value.cex_id(c)?,
                account_id: account_id_value.account_id(c)?,
                order_request: order_request.try_resolve(c)?,
            }),
            // Action::CancelAllOrders => Ok(ResolvedAction::CancelAllOrders),
            // Action::CancelAllOrdersWithTag { tag } => {
            //     Ok(ResolvedAction::CancelAllOrdersWithTag { tag: tag.clone() })
            // }
            // Action::CancelAllOrdersInCex { cex_id_value } => {
            //     Ok(ResolvedAction::CancelAllOrdersInCex {
            //         cex_id: cex_id_value.cex_id(c)?,
            //     })
            // }
            // Action::CancelAllOrdersInCexWithTag { cex_id_value, tag } => {
            //     Ok(ResolvedAction::CancelAllOrdersInCexWithTag {
            //         cex_id: cex_id_value.cex_id(c)?,
            //         tag: tag.clone(),
            //     })
            // }
        }
    }
}

impl HasRequiredCapabilities for Action {
    fn required_capabilities(&self) -> Vec<CexCapability> {
        match self {
            Action::SendOrderRequest { order_request, .. }
            | Action::SendOrderRequestInAccount { order_request, .. } => {
                order_request.required_capabilities()
            } // Action::CancelAllOrders
              // | Action::CancelAllOrdersWithTag { .. }
              // | Action::CancelAllOrdersInCex { .. }
              // | Action::CancelAllOrdersInCexWithTag { .. } => vec![],
        }
    }
}
