use crate::state::Entry;
use cosmwasm_std::Binary;
use cosmwasm_schema::{cw_serde, QueryResponses};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

//#[derive(Serialize, Deserialize)]
#[cw_serde]
pub struct InstantiateMsg {
    pub message: String,
}

//#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[cw_serde]
pub enum ExecuteMsg {
    UpdateMessage { message: String },
    NewEntry {
        description: String,
        owner: String,
    },

}

// We define a custom struct for each query response
#[cw_serde]
pub struct EntryResponse {
    pub id: u64,
    pub description: String,
    pub owner: String,
}
#[cw_serde]
pub struct ListResponse {
    pub entries: Vec<Entry>,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct QueryResponse {
    pub response: String,
}

impl QueryResponse {
    pub fn new(response: &str) -> Self {
        QueryResponse {
            response: response.to_string(),
        }
    }
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {
    GetMessage { user : String },
    GetState { user : String },
    //#[returns(ListResponse)]
    QueryUserList { 
        user: String, 
        start_after: Option<u64>, 
        limit: Option<u32> 
    }
}