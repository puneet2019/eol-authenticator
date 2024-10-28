pub mod error;
pub mod params;
pub mod eol;

use cosmwasm_std::Addr;
use cw_storage_plus::Map;

pub type EOLStore<'a> = Map<'a, EOLKey<'a>, eol::EOL>;

/// SpendingKey is a key for the spending storage.
/// It is a tuple of (account, authenticator_id) which
/// allows multiple spend limits per account.
pub type EOLKey<'a> = (&'a Addr, &'a str);

