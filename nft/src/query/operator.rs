use cosmwasm_std::{to_json_binary, Binary, Deps, Env, StdError};
use crate::core::cw721::{Approval, OperatorResponse};
use crate::core::error::ContractError;
use crate::storage::operators::OPERATORS;

pub fn handle(
    deps: Deps,
    env: Env,
    owner: String,
    operator: String,
    include_expired: bool,
) -> Result<Binary, ContractError> {
    let owner_addr = deps.api.addr_validate(&owner)?;
    let operator_addr = deps.api.addr_validate(&operator)?;

    let info = OPERATORS.may_load(deps.storage, (&owner_addr, &operator_addr))?;

    if let Some(expires) = info {
        if !include_expired && expires.is_expired(&env.block) {
            return Err(ContractError::Std(StdError::not_found(
                "Approval not found",
            )));
        }
        return Ok(to_json_binary(&OperatorResponse {
            approval: Approval {
                spender: operator,
                expires,
            },
        })?);
    }

    Err(ContractError::Std(StdError::not_found(
        "Approval not found",
    )))
}
