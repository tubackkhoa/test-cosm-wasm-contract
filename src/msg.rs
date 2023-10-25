use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InitMsg {
    #[serde(default)]
    pub counter: u64,
}

#[cw_serde]

pub struct ValueResp {
    pub value: u64,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ValueResp)]
    Value {},
}
#[cw_serde]
pub enum ExecMsg {
    DonateOrai { receiver: String },
}
