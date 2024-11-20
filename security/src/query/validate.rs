use cosmwasm_std::{Coin, Deps};

use crate::{
    core::msg::QueryMsg,
    util::validate::{Validate, ValidateResult},
};

impl Validate for QueryMsg {
    /// Performs basic error checking on the QueryMsg
    ///
    /// # Arguments
    ///
    /// * `self` - A reference to the message implementing this trait.
    /// * `deps` - A non mutable version of the dependencies. The API, Querier, and storage can all be accessed from it.
    ///
    /// # Examples
    /// ```
    /// use cosmwasm_std::Addr;
    /// use cosmwasm_std::testing::message_info;
    /// use provwasm_mocks::mock_provenance_dependencies;
    /// use security::core::msg::{QueryMsg};
    /// use security::util::validate::Validate;
    ///
    /// let deps = mock_provenance_dependencies();
    /// let info = message_info(&Addr::unchecked("sender"), &[]);
    /// let msg = QueryMsg::QueryVersion {};
    /// let result = msg.validate(deps.as_ref());
    /// ```
    fn validate(&self, _deps: Deps) -> ValidateResult {
        Ok(())
    }

    /// Performs basic error checking on QueryMsg.
    ///
    /// # Arguments
    ///
    /// * `self` - A reference to the message implementing this trait.
    /// * `funds` - A slice representing the funds included with the message.
    ///
    /// # Examples
    /// ```
    /// use cosmwasm_std::Addr;
    /// use cosmwasm_std::testing::message_info;
    /// use provwasm_mocks::mock_provenance_dependencies;
    /// use security::core::msg::{QueryMsg};
    /// use security::util::validate::Validate;
    ///
    /// let deps = mock_provenance_dependencies();
    /// let info = message_info(&Addr::unchecked("sender"), &[]);
    /// let msg = QueryMsg::QueryVersion {};
    /// let result = msg.validate_funds(&info.funds);
    /// ```
    fn validate_funds(&self, _funds: &[Coin]) -> ValidateResult {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use provwasm_mocks::mock_provenance_dependencies;

    use crate::{core::msg::QueryMsg, util::validate::Validate};

    #[test]
    fn test_query_owner_validate_succeeds() {
        let query = QueryMsg::QueryOwner {};
        let deps = mock_provenance_dependencies();
        assert!(query.validate(deps.as_ref()).is_ok());
    }

    #[test]
    fn test_query_version_validate_succeeds() {
        let query = QueryMsg::QueryVersion {};
        let deps = mock_provenance_dependencies();
        assert!(query.validate(deps.as_ref()).is_ok());
    }

    #[test]
    fn test_query_funds_should_always_return_true() {
        let query1 = QueryMsg::QueryVersion {};
        let query2 = QueryMsg::QueryOwner {};
        assert!(query1.validate_funds(&[]).is_ok());
        assert!(query2.validate_funds(&[]).is_ok());
    }
}
