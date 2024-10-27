use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Timestamp, Uint128};


#[cw_serde]
pub struct EOLParams {
    pub inactivity_period: Timestamp,
}

