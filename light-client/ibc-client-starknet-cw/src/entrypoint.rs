use ibc_client_cw::context::Context;
use ibc_client_cw::types::{ContractError, InstantiateMsg, QueryMsg, SudoMsg};
use sylvia::cw_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response};

use crate::client_type::StarknetClient;

pub type StarknetContext<'a> = Context<'a, StarknetClient>;

#[cfg_attr(not(feature = "library"), sylvia::cw_std::entry_point)]
pub fn instantiate(
    deps: DepsMut<'_>,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let mut ctx = StarknetContext::new_mut(deps, env)?;

    let data = ctx.instantiate(msg)?;

    Ok(Response::default().set_data(data))
}

#[cfg_attr(not(feature = "library"), sylvia::cw_std::entry_point)]
pub fn sudo(deps: DepsMut<'_>, env: Env, msg: SudoMsg) -> Result<Response, ContractError> {
    let mut ctx = StarknetContext::new_mut(deps, env)?;

    let data = ctx.sudo(msg)?;

    Ok(Response::default().set_data(data))
}

#[cfg_attr(not(feature = "library"), sylvia::cw_std::entry_point)]
pub fn query(deps: Deps<'_>, env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    let ctx = StarknetContext::new_ref(deps, env)?;

    ctx.query(msg)
}
