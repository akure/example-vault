use cosmwasm_std::{Addr, Coin, DepsMut, Env, MessageInfo, Response, StdError, StdResult};
use cw_storage_plus::{Item, Map};
use crate::error::ContractError;
use serde::{Serialize,Deserialize};
use schemars::JsonSchema;
use cosmwasm_std::{Uint128, Uint64};


pub const VAULT_POSITIONS: Map<Addr, VaultPosition> = Map::new("vault_positions");

//#[cw_serde]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VaultPosition {
    // pub vault_address: Addr, // Contract address
    pub vault_share_balance: Vec<Coin>,  //  
    pub last_updated: u64,
    pub total_unallocated :Uint128,
}

impl VaultPosition {
    pub fn new() -> Self {
        VaultPosition {
            vault_share_balance: Vec::new(),
            last_updated: 0,
            total_unallocated: Uint128::zero(),
        }
    }

    pub fn new2(vault_share_balance: Vec<Coin>, last_updated: u64, total_unallocated: Uint128) -> Self {
        VaultPosition {
            vault_share_balance,
            last_updated,
            total_unallocated,
        }
    }
}

pub fn update_vault_position(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    vault_address: Addr,
    vault_share_balance: Vec<Coin>,
    total_unallocated :Uint128,
) -> Result<Response, ContractError> {
    let position = VaultPosition::new2(vault_share_balance, 
        env.block.time.seconds(), 
        total_unallocated);

    VAULT_POSITIONS.save(deps.storage, vault_address.clone(), &position)?;

    Ok(Response::new()
        .add_attribute("action", "update_vault_position")
        .add_attribute("vault_address", vault_address.to_string())
        .add_attribute("sender", info.sender.to_string()))
}

pub fn get_vault_position(
    deps: DepsMut,
    vault_address: Addr,
) -> StdResult<VaultPosition> {
    VAULT_POSITIONS.load(deps.storage, vault_address)
}
