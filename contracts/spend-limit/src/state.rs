use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use osmosis_std::types::osmosis::poolmanager::v1beta1::SwapAmountInRoute;

use crate::{
    price::{PriceInfoStore, PriceResolutionConfig},
    spend_limit::{PreExecBalance, SpendingStore},
};

#[cw_serde]
pub struct TrackedDenom {
    pub denom: Denom,
    pub path: Path,
}

pub type Denom = String;
pub type Path = Vec<SwapAmountInRoute>;

pub const SPENDINGS: SpendingStore<'_> = Map::new("spendings");

/// [`PreExecBalance`] is a map of spending keys to the account balances.
/// It is used to track the balances of the accounts before the transaction is executed,
/// and compare it with the balances after the transaction is executed.
///
/// It's lifetime is only within one authenticator's lifecycle.
pub const PRE_EXEC_BALANCES: PreExecBalance<'_> = Map::new("pre_exec_balance");

/// Contract address of the price oracle used for determining the price of the assets.
pub const PRICE_ORACLE_CONTRACT_ADDR: Item<Addr> = Item::new("price_oracle_contract_addr");

/// Configuration for the price resolution.
pub const PRICE_RESOLUTION_CONFIG: Item<PriceResolutionConfig> =
    Item::new("price_resolution_config");

/// Store for the price info of the tracked denoms.
pub const PRICE_INFOS: PriceInfoStore<'_> = Map::new("price_infos");
