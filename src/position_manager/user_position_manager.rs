use cosmwasm_std::{Addr, Coin, DepsMut, Env, MessageInfo, Response, StdError, StdResult};
use cw_storage_plus::{Item, Map};
use crate::error::ContractError;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

pub const USER_POSITIONS: Map<(Addr, Addr), UserPosition> = Map::new("user_positions");

// TODO - this file to be refined.
//#[cw_serde]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UserPosition {
    pub user_address: Addr,
    pub vault_address: Addr,
    pub balance: Vec<Coin>,
    pub last_updated: u64,
}

impl UserPosition {
    pub fn new(user_address: Addr, vault_address: Addr, balance: Vec<Coin>, last_updated: u64) -> Self {
        UserPosition {
            user_address,
            vault_address,
            balance,
            last_updated,
        }
    }
}

pub fn update_user_position(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    user_address: Addr,
    vault_address: Addr,
    balance: Vec<Coin>,
) -> Result<Response, ContractError> {
    let position = UserPosition::new(user_address.clone(), vault_address.clone(), balance, env.block.time.seconds());

    USER_POSITIONS.save(deps.storage, (user_address.clone(), vault_address.clone()), &position)?;

    Ok(Response::new()
        .add_attribute("action", "update_user_position")
        .add_attribute("user_address", user_address.to_string())
        .add_attribute("vault_address", vault_address.to_string())
        .add_attribute("sender", info.sender.to_string()))
}

pub fn get_user_position(
    deps: DepsMut,
    user_address: Addr,
    vault_address: Addr,
) -> StdResult<UserPosition> {
    USER_POSITIONS.load(deps.storage, (user_address, vault_address))
}
