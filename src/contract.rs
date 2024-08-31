#[cfg(not(feature = "library"))]
use std::string;

use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Order, 
    StdError,to_binary
};

use cw2::set_contract_version;
use cw_storage_plus::Bound;
use std::ops::Add;

use crate::state::{self, State, STATE, Entry, ENTRY_SEQ, LIST};
use crate::msg::{InstantiateMsg, ExecuteMsg, QueryResponse, QueryMsg, ListResponse};
use crate::error::ContractError;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

// version info for migration
const CONTRACT_NAME: &str = "crates.io:cw-omly-fans-messages";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let state = State { message: msg.message };
    STATE.save(deps.storage, &state)?;
    set_contract_version(deps.storage, "omly_fans", "1.0.0")?;
    
    ENTRY_SEQ.save(deps.storage, &0u64)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateMessage { message } => try_update_message(deps, message),
        ExecuteMsg::NewEntry {
            description,
            owner,
        } => execute_create_new_entry(deps, _info, description, owner),
    }
}

pub fn execute_create_new_entry(
    deps: DepsMut,
    _info: MessageInfo,
    description: String,
    owner: String,
) -> Result<Response, ContractError> {
    let id = ENTRY_SEQ.update::<_, cosmwasm_std::StdError>(deps.storage, |id| Ok(id.add(1)))?;
    let new_entry = Entry {
        id,
        description,
        owner,
    };
    LIST.save(deps.storage, id, &new_entry)?;
    Ok(Response::new()
        .add_attribute("method", "execute_create_new_entry")
        .add_attribute("new_entry_id", id.to_string()))         
}


pub fn try_update_message(deps: DepsMut, message: String) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> StdResult<_> {
        state.message = message;
        Ok(state)
    })?;
    Ok(Response::default())
}


#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetMessage { user } => to_json_binary(&query_message(deps, user)?),
        QueryMsg::GetState { user } => to_json_binary(&query_state(deps, user)?),
        QueryMsg::QueryUserList { user, start_after, limit } => {
            to_json_binary(&query_user_list(deps, user, start_after, limit)?)
        }
    }
}

fn query_message(deps: Deps, user: String) -> StdResult<QueryResponse> {
    
    let result = QueryResponse {
        response: String::from("OMLY Fans 1"),
    };

    Ok(result)
}

fn query_state(deps: Deps, user: String) -> StdResult<QueryResponse> {
    
    let result = QueryResponse {
        response: String::from("OMLY Fans 2"),
    };

    Ok(result)
}

// Limits for pagination
const MAX_LIMIT: u32 = 30;
const DEFAULT_LIMIT: u32 = 10;

pub fn query_user_list(deps: Deps, user: String, start_after: Option<u64>, limit: Option<u32>) -> StdResult<ListResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(Bound::exclusive);
    
    let entries: StdResult<Vec<_>> = LIST
        .range(deps.storage, start, None, Order::Descending)
        .take(limit)
        .collect();

    let result = ListResponse {
        entries: entries?.into_iter().map(|(_, entry)| entry).collect(),
    };
    Ok(result)
}