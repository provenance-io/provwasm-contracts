use uuid::Uuid;

use crate::core::error::ContractError;

pub mod action;
pub mod permission;
pub mod validate;

pub fn parse_uuid(id: &str) -> Result<Uuid, ContractError> {
    Uuid::parse_str(id).map_err(|error| ContractError::Uuid { error })
}
