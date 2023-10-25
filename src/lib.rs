use cosmwasm_std::entry_point;

use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use msg::{ExecMsg, InitMsg, QueryMsg};

mod contract;
pub mod msg;
mod state;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InitMsg,
) -> StdResult<Response> {
    use contract::instantiate;
    instantiate(deps, info, msg.counter);
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    use contract::query::value;
    use msg::QueryMsg::*;
    match _msg {
        Value {} => to_binary(&value(deps)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecMsg) -> StdResult<Response> {
    use contract::execute::donate_orai;
    use msg::ExecMsg::*;
    match msg {
        DonateOrai { receiver, amount } => donate_orai(deps, env, info, receiver, amount),
    }
}
