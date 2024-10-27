use cosmwasm_std::{ensure, DepsMut, Env, Response};
use cw_authenticator::OnAuthenticatorAddedRequest;

use crate::state::EOLS;
use crate::{
    authenticator::{handler::validate_and_parse_params, AuthenticatorError},
    eol::eol::EOL
    ,
};

pub fn on_authenticator_added(
    deps: DepsMut,
    env: Env,
    OnAuthenticatorAddedRequest {
        authenticator_id,
        account,
        authenticator_params,
    }: OnAuthenticatorAddedRequest,
) -> Result<Response, AuthenticatorError> {
    let params = validate_and_parse_params(authenticator_params)?;

    // Make sure (account, authenticator_id) is not already present in the state
    let key = (&account, authenticator_id.as_str());
    ensure!(
        !EOLS.has(deps.storage, key),
        AuthenticatorError::authenticator_already_exists(account, authenticator_id.as_str())
    );

    // initialize the spending for this authenticator
    EOLS.save(deps.storage, key, &EOL::new(params.inactivity_period, env.block.time))?;

    Ok(Response::new().add_attribute("action", "on_authenticator_added"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eol::EOLParams;
    use cosmwasm_std::{testing::{mock_dependencies_with_balances, mock_env}, to_json_binary, Addr, Coin, StdError, Timestamp};

    const USDC: &str = "ibc/498A0751C798A0D9A389AA3691123DADA57DAA4FE165D5C75894505B876BA6E4";

    #[test]
    fn test_on_authenticator_added() {
        let mut deps = mock_dependencies_with_balances(&[("someoneelse", &[Coin::new(1, USDC)])]);

        // missing authenticator_params
        let request = OnAuthenticatorAddedRequest {
            authenticator_id: "2".to_string(),
            account: Addr::unchecked("addr"),
            authenticator_params: None,
        };
        assert_eq!(
            on_authenticator_added(deps.as_mut(), mock_env(), request).unwrap_err(),
            AuthenticatorError::MissingAuthenticatorParams
        );

        // invalid authenticator_params
        let request = OnAuthenticatorAddedRequest {
            authenticator_id: "2".to_string(),
            account: Addr::unchecked("addr"),
            authenticator_params: Some(to_json_binary(&"invalid").unwrap()),
        };

        assert_eq!(
            on_authenticator_added(deps.as_mut(), mock_env(), request).unwrap_err(),
            AuthenticatorError::invalid_authenticator_params(StdError::parse_err(
                std::any::type_name::<EOLParams>(),
                "Invalid type"
            ))
        );

        // valid
        let request = OnAuthenticatorAddedRequest {
            authenticator_id: "2".to_string(),
            account: Addr::unchecked("addr"),
            authenticator_params: Some(
                to_json_binary(&EOLParams {
                    inactivity_period: Timestamp::from_seconds(100),
                })
                    .unwrap(),
            ),
        };

        let res = on_authenticator_added(deps.as_mut(), mock_env(), request).unwrap();
        assert_eq!(
            res,
            Response::new().add_attribute("action", "on_authenticator_added")
        );

        // check the state
        let spending = EOLS
            .load(deps.as_ref().storage, (&Addr::unchecked("addr"), "2"))
            .unwrap();
        assert_eq!(spending, EOL::default());

        // Adding the authenticator with the same (account, authenticator_id) should fail
        let request = OnAuthenticatorAddedRequest {
            authenticator_id: "2".to_string(),
            account: Addr::unchecked("addr"),
            authenticator_params: Some(
                to_json_binary(&EOLParams {
                    inactivity_period: Timestamp::from_seconds(100),

                })
                    .unwrap(),
            ),
        };

        assert_eq!(
            on_authenticator_added(deps.as_mut(), mock_env(), request).unwrap_err(),
            AuthenticatorError::authenticator_already_exists(Addr::unchecked("addr"), "2")
        );
    }
}
