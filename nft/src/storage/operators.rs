use crate::core::constants::OPERATORS_KEY;
use cosmwasm_std::Addr;
use cw_storage_plus::Map;
use crate::core::cw721::Expiration;

pub const OPERATORS: Map<(&'static Addr, &'static Addr), Expiration> =
    Map::new(OPERATORS_KEY);
