use crate::core::cw721::NftInfoResponse;
use crate::core::error::ContractError;
use crate::core::msg::{NftData, Scope};
use crate::storage::nft::TOKENS;
use cosmwasm_std::{to_json_binary, Binary, Deps, Env, QuerierWrapper, StdError};
use provwasm_std::types::provenance::metadata::v1::{MetadataQuerier, ScopeResponse};

pub fn handle(deps: Deps, _env: Env, token_id: String) -> Result<Binary, ContractError> {
    let info = TOKENS.load(deps.storage, &token_id)?;

    Ok(to_json_binary(&NftInfoResponse {
        token_uri: None,
        extension: &NftData {
            id: info.id,
            owner: info.owner,
            data: match load_scope(token_id, deps.querier)?.scope {
                Some(scope_wrapper) => match scope_wrapper.scope {
                    Some(scope) => Some(Scope {
                        scope_id: scope.scope_id,
                        specification_id: scope.specification_id,
                        value_owner_address: scope.value_owner_address,
                    }),
                    _ => None,
                },
                None => None,
            },
        },
    })?)
}

pub fn load_scope(scope_id: String, querier: QuerierWrapper) -> Result<ScopeResponse, StdError> {
    MetadataQuerier::new(&querier).scope(
        scope_id,
        "".to_string(),
        "".to_string(),
        true,
        true,
        false,
        false,
    )
}
