use crate::core::cw721::{AllNftInfoResponse, NftInfoResponse, OwnerOfResponse};
use cosmwasm_std::{to_json_binary, Binary, Deps, Env};

use crate::core::error::ContractError;
use crate::core::msg::{NftData, Scope};
use crate::query::nft_info::load_scope;
use crate::storage::nft::TOKENS;
use crate::util::permission::humanize_approvals;

pub fn handle(
    deps: Deps,
    env: Env,
    token_id: String,
    include_expired: bool,
) -> Result<Binary, ContractError> {
    let info = TOKENS.load(deps.storage, &token_id)?;
    Ok(to_json_binary(&AllNftInfoResponse {
        access: OwnerOfResponse {
            owner: info.owner.to_string(),
            approvals: humanize_approvals(&env.block, &info, include_expired),
        },
        info: NftInfoResponse {
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
        },
    })?)
}
