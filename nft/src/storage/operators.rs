use crate::core::constants::OPERATORS_KEY;
use crate::core::cw721::Expiration;
use cosmwasm_std::Addr;
use cw_storage_plus::Map;

pub const OPERATORS: Map<(&'static Addr, &'static Addr), Expiration> = Map::new(OPERATORS_KEY);
