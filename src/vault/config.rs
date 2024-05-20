use crate::msg::InstantiateMsg;
use cosmwasm_schema::cw_serde;
use serde::{de::DeserializeOwned, Serialize};

//use vaultenator::config::Configure;
//use vaultenator::errors::ContractError;

use cosmwasm_std::{DepsMut, Uint128};

#[cw_serde]
pub struct Config {
    pub max_deposit_cap: Uint128, // in the amounts of deposit denom
    pub deposit_denom: String,
    pub share_denom: String,
 }



