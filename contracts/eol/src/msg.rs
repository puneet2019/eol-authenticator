use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint64;
pub use osmosis_std::types::osmosis::poolmanager::v1beta1::SwapAmountInRoute;

use crate::eol::eol::EOL;

use crate::eol::EOLStore;
// re-export the structs from cw_authenticator
pub use cw_authenticator::AuthenticatorSudoMsg as SudoMsg;

#[cw_serde]
pub struct TrackedDenom {
    pub denom: String,
    pub swap_routes: Vec<SwapAmountInRoute>,
}

#[cw_serde]
pub enum DenomRemovalTarget {
    All,
    Partial(Vec<String>),
}

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: Option<String>,
}

#[cw_serde]
pub enum ExecuteMsg {
    TransferAdmin {
        address: String,
    },
    ClaimAdminTransfer {},
    RejectAdminTransfer {},
    CancelAdminTransfer {},
    RevokeAdmin {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(EOLResponse)]
    EOL {
        account: String,
        authenticator_id: String,
    },

    #[returns(EOLsByAccountResponse)]
    EOLs { account: String },

    #[returns(AdminResponse)]
    Admin {},

    #[returns(AdminCandidateResponse)]
    AdminCandidate {},
}

#[cw_serde]
pub struct EOLResponse {
    pub eol: EOL,
}

#[cw_serde]
pub struct EOLsByAccountResponse {
    pub eols: Vec<(String, EOL)>,
}

#[cw_serde]
pub struct AdminResponse {
    pub admin: Option<String>,
}

#[cw_serde]
pub struct AdminCandidateResponse {
    pub candidate: Option<String>,
}
