use crate::core::cw721::NumTokensResponse;
use cosmwasm_std::{to_json_binary, Binary, Deps};

use crate::core::error::ContractError;
use crate::storage::nft_count::nft_count;

pub fn handle(deps: Deps) -> Result<Binary, ContractError> {
    let count = nft_count(deps.storage)?;
    Ok(to_json_binary(&NumTokensResponse { count })?)
}
