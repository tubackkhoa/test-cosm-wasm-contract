use cosmwasm_std::{DepsMut, MessageInfo, Response, StdResult};

use crate::state::COUNTER;

pub fn instantiate(deps: DepsMut, _info: MessageInfo, counter: u64) -> StdResult<Response> {
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
    use cosmwasm_std::{BankMsg, DepsMut, Env, MessageInfo, Response, StdResult};

    pub fn donate_orai(
        _deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        receiver: String,
    ) -> StdResult<Response> {
        let amount = info
            .funds
            .into_iter()
            .filter(|fund| fund.denom == "orai")
            .collect();
        let bank_msg = BankMsg::Send {
            to_address: receiver,
            amount,
        };
        let resp = Response::new().add_message(bank_msg);
        Ok(resp)
    }
}
