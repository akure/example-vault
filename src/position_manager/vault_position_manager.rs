use cosmwasm_std::{Addr, Coin, DepsMut, Env, MessageInfo, Response, StdError, StdResult};
use cw_storage_plus::{Item, Map};
use crate::ContractError;

pub const VAULT_POSITIONS: Map<Addr, VaultPosition> = Map::new("vault_positions");

#[cw_serde]
pub struct VaultPosition {
    pub vault_address: Addr,
    pub balance: Vec<Coin>,
    pub last_updated: u64,
}

impl VaultPosition {
    pub fn new(vault_address: Addr, balance: Vec<Coin>, last_updated: u64) -> Self {
        VaultPosition {
            vault_address,
            balance,
            last_updated,
        }
    }
}

pub fn update_vault_position(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    vault_address: Addr,
    balance: Vec<Coin>,
) -> Result<Response, ContractError> {
    let position = VaultPosition::new(vault_address.clone(), balance, env.block.time.seconds());

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
