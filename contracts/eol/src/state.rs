use cw_storage_plus::{Item, Map};

use crate::{
    admin::Admin,

};
use crate::eol::EOLStore;

pub const EOLS: EOLStore<'_> = Map::new("eols");

/// Admin address, Optional.
pub const ADMIN: Item<Admin> = Item::new("admin");
