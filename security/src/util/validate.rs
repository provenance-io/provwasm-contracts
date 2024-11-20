use cosmwasm_std::{Coin, Deps};

use crate::core::error::ContractError;

pub type ValidateResult = Result<(), ContractError>;

pub trait Validate {
    /// Performs basic error checking on the messages that implement this trait.
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
    /// use security::core::msg::{MigrateMsg};
    /// use security::util::validate::Validate;
    ///
    /// let deps = mock_provenance_dependencies();
    /// let info = message_info(&Addr::unchecked("sender"), &[]);
    /// let msg = MigrateMsg::Default {};
    /// let result = msg.validate(deps.as_ref());
    /// ```
    fn validate(&self, deps: Deps) -> ValidateResult;

    /// Performs basic error checking on the messages that implement this trait.
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
    /// use security::core::msg::{MigrateMsg};
    /// use security::util::validate::Validate;
    ///
    /// let deps = mock_provenance_dependencies();
    /// let info = message_info(&Addr::unchecked("sender"), &[]);
    /// let msg = MigrateMsg::Default {};
    /// let result = msg.validate_funds(&info.funds);
    /// ```
    fn validate_funds(&self, funds: &[Coin]) -> ValidateResult;
}
