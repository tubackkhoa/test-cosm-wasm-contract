use cosmwasm_std::entry_point;

use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use msg::{ExecMsg, InitMsg, QueryMsg};

mod contract;
pub mod msg;
mod state;

#[cfg(test)]
mod test;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InitMsg,
) -> StdResult<Response> {
    contract::instantiate(deps, info, msg.counter)?;
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    use msg::QueryMsg::*;
    match _msg {
        Value {} => to_binary(&contract::query::value(deps)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecMsg) -> StdResult<Response> {
    use msg::ExecMsg::*;
    match msg {
        DonateOrai { receiver } => contract::execute::donate_orai(deps, env, info, receiver),
    }
}
