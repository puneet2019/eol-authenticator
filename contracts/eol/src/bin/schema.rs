#![cfg(not(tarpaulin_include))]

use cosmwasm_schema::write_api;
use eol::msg::{InstantiateMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        query: QueryMsg,
    }
}
