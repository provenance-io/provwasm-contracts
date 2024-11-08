use crate::core::cw721::{Approval, ApprovalResponse, Expiration};
use cosmwasm_std::{to_json_binary, Binary, Deps, Env, StdError};

use crate::core::error::ContractError;
use crate::storage::nft::TOKENS;

pub fn handle(
    deps: Deps,
    env: Env,
    token_id: String,
    spender: String,
    include_expired: bool,
) -> Result<Binary, ContractError> {
    let token = TOKENS.load(deps.storage, &token_id)?;

    // token owner has absolute approval
    if token.owner.as_str() == spender {
        let approval = Approval {
            spender: token.owner.to_string(),
            expires: Expiration::Never {},
        };
        return Ok(to_json_binary(&ApprovalResponse { approval })?);
    }

    let filtered: Vec<_> = token
        .approvals
        .into_iter()
        .filter(|t| t.spender.as_str() == spender)
        .filter(|t| include_expired || !t.is_expired(&env.block))
        .map(|a| Approval {
            spender: a.spender.into_string(),
            expires: a.expires,
        })
        .collect();

    if filtered.is_empty() {
        return Err(ContractError::from(StdError::not_found(
            "Approval not found",
        )));
    }
    // we expect only one item
    let approval = filtered[0].clone();

    Ok(to_json_binary(&ApprovalResponse { approval })?)
}
