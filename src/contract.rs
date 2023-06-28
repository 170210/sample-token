use cosmwasm_std::{
    dynamic_link, entry_point, from_slice, to_binary, to_vec, wasm_execute, Addr, Binary, Contract,
    Deps, DepsMut, Env, MessageInfo, Reply, Response, SubMsg,
};
 
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, CalleeExecuteMsg, CalleeQueryMsg, NumberResponse, QueryMsg};
 
const ADDRESS_KEY: &[u8] = b"counter-address";
 
#[derive(Contract)]
struct CounterContract {
    address: Addr,
}
 
#[dynamic_link(CounterContract)]
trait Counter: Contract {
    fn increment(&self, by: i32);
    fn counter(&self) -> i32;
}
 
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    deps.storage.set(ADDRESS_KEY, &to_vec(&msg.callee_addr)?);
    Ok(Response::default())
}
 
#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Add { value } => handle_add(deps.as_ref(), value),
        ExecuteMsg::SubmsgReplyIncrement { value } => handle_submsg_reply_increment(deps.as_ref(), value),
    }
}
 
fn handle_add(deps: Deps, by: i32) -> Result<Response, ContractError> {
    let address: Addr = from_slice(&deps.storage.get(ADDRESS_KEY).unwrap())?;
    let contract = CounterContract {
        address: address.clone(),
    };
    contract.increment(by);
    Ok(Response::default())
}
 
 
fn handle_submsg_reply_increment(deps: Deps, by: i32) -> Result<Response, ContractError> {
    let contract_addr: Addr = from_slice(&deps.storage.get(ADDRESS_KEY).unwrap())?;
    let execute_msg = SubMsg::reply_on_success(
        wasm_execute(contract_addr, &CalleeExecuteMsg::Increment { value: by }, vec![])?,
        0,
    );
    let response = Response::default().add_submessage(execute_msg);
    Ok(response)
}
 
#[entry_point]
pub fn reply(deps: DepsMut, _env: Env, _msg: Reply) -> Result<Response, ContractError> {
    let address: Addr = from_slice(&deps.storage.get(ADDRESS_KEY).unwrap())?;
    let contract = CounterContract {
        address: address.clone(),
    };
    let value_dyn = contract.counter();
    let res: NumberResponse = deps
        .querier
        .query_wasm_smart(address, &CalleeQueryMsg::Number {})?;
    let response = Response::default()
        .add_attribute("value_by_dynamic", value_dyn.to_string())
        .add_attribute("value_by_query", res.value.to_string());
 
    Ok(response)
}
 
#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::Counter {} => Ok(to_binary(&query_counter(deps)?)?),
        QueryMsg::CounterDyn {} => Ok(to_binary(&query_counter_dyn(deps)?)?),
    }
}
 
fn query_counter(deps: Deps) -> Result<NumberResponse, ContractError> {
    let address: Addr = from_slice(&deps.storage.get(ADDRESS_KEY).unwrap())?;
    let response: NumberResponse = deps
        .querier
        .query_wasm_smart(address, &CalleeQueryMsg::Number {})?;
    Ok(response)
}

fn query_counter_dyn(deps: Deps) -> Result<NumberResponse, ContractError> {
    let address: Addr = from_slice(&deps.storage.get(ADDRESS_KEY).unwrap())?;
    let contract = CounterContract { address };
    let value = contract.counter();
    Ok(NumberResponse { value })
}