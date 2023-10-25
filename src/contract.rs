use cosmwasm_std::{DepsMut, MessageInfo, Response, StdResult};

use crate::state::COUNTER;

pub fn instantiate(deps: DepsMut, info: MessageInfo, counter: u64) -> StdResult<Response> {
    COUNTER.save(deps.storage, &counter)?;
    Ok(Response::new())
}

pub mod query {
    use cosmwasm_std::{Deps, StdResult};

    use crate::{msg::ValueResp, state::COUNTER};

    pub fn value(deps: Deps) -> StdResult<ValueResp> {
        let value = COUNTER.load(deps.storage)?;
        Ok(ValueResp { value })
    }
}

pub mod execute {
    use cosmwasm_std::{Addr, BankMsg, Coin, DepsMut, Env, MessageInfo, Response, StdResult};

    pub fn donate_orai(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        receiver: Addr,
        amount: Vec<Coin>,
    ) -> StdResult<Response> {
        let bank_msg = BankMsg::Send {
            to_address: receiver.to_string(),
            amount,
        };
        let resp = Response::new().add_message(bank_msg);
        Ok(resp)
    }
}
