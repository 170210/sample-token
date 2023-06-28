use cosmwasm_std::{Addr};
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub callee_addr: Addr,
}

#[cw_serde]
pub enum ExecuteMsg {
    Add { value: i32 },
    SubmsgReplyIncrement { value: i32 },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(NumberResponse)]
    Counter {},
    #[returns(NumberResponse)]
    CounterDyn {},
}

#[cw_serde]
pub struct NumberResponse {
    pub value: i32,
}

#[cw_serde]
pub enum CalleeExecuteMsg {
    Increment{ value: i32 },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum CalleeQueryMsg {
    #[returns(NumberResponse)]
    Number {},    
}