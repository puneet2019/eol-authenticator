use cosmwasm_std::{DepsMut, Env, Response};
use cw_authenticator::OnAuthenticatorRemovedRequest;

use crate::state::EOLS;
use crate::authenticator::AuthenticatorError;

pub fn on_authenticator_removed(
    deps: DepsMut,
    _env: Env,
    OnAuthenticatorRemovedRequest {
        account,
        authenticator_id,
        ..
    }: OnAuthenticatorRemovedRequest,
) -> Result<Response, AuthenticatorError> {
    // clean up the spending
    EOLS.remove(deps.storage, (&account, authenticator_id.as_str()));

    Ok(Response::new().add_attribute("action", "on_authenticator_removed"))
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::{mock_dependencies, mock_env}, to_json_binary, Addr, Timestamp};

    use crate::eol::{EOLParams, EOL};

    use super::*;

    #[test]
    fn test_on_authenticator_removed() {
        let mut deps = mock_dependencies();

        // remove the spending
        let key = (&Addr::unchecked("account"), "2");
        EOLS
            .save(deps.as_mut().storage, key, &EOL::default())
            .unwrap();
        assert!(SPENDINGS.has(deps.as_ref().storage, key));

        let msg = OnAuthenticatorRemovedRequest {
            authenticator_id: "2".to_string(),
            account: Addr::unchecked("account"),
            authenticator_params: Some(
                to_json_binary(&EOLParams {
                    inactivity_period: Timestamp::from_seconds(100)
                })
                    .unwrap(),
            ),
        };

        on_authenticator_removed(deps.as_mut(), mock_env(), msg).unwrap();
        assert!(!SPENDINGS.has(deps.as_ref().storage, key));
    }
}
