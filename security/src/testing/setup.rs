use cosmwasm_std::{
    testing::{mock_env, MockApi, MockStorage},
    Addr, Coin, DepsMut, Empty, MessageInfo, OwnedDeps,
};
use provwasm_mocks::MockProvenanceQuerier;
use provwasm_std::{
    shim::Any,
    types::provenance::{
        marker::v1::{QueryMarkerRequest, QueryMarkerResponse},
        metadata::v1::{ScopeRequest, ScopeResponse, ScopeWrapper},
    },
};

use crate::contract;

use super::{
    constants::{CREATOR, TEST_AMOUNT, TEST_DENOM},
    msg::mock_instantiate_msg,
};

pub fn mock_info(funds: bool, sender: &str) -> MessageInfo {
    if funds {
        return MessageInfo {
            sender: Addr::unchecked(sender),
            funds: vec![Coin::new(TEST_AMOUNT, TEST_DENOM)],
        };
    }

    MessageInfo {
        sender: Addr::unchecked(sender),
        funds: vec![],
    }
}

pub fn mock_contract(deps: DepsMut) {
    let info = mock_info(false, CREATOR);
    let env = mock_env();
    let msg = mock_instantiate_msg();
    contract::instantiate(deps, env, info, msg).unwrap();
}

pub fn mock_scopes(deps: &mut OwnedDeps<MockStorage, MockApi, MockProvenanceQuerier, Empty>) {
    ScopeRequest::mock_response(
        &mut deps.querier,
        ScopeResponse {
            scope: Some(ScopeWrapper::default()),
            sessions: vec![],
            records: vec![],
            request: None,
        },
    )
}

pub fn mock_markers(deps: &mut OwnedDeps<MockStorage, MockApi, MockProvenanceQuerier, Empty>) {
    QueryMarkerRequest::mock_response(
        &mut deps.querier,
        QueryMarkerResponse {
            marker: Some(Any {
                type_url: "/provenance.marker.v1.MarkerAccount".to_string(),
                value: vec![],
            }),
        },
    );
}
