
use cosmwasm_std::{
    attr, ensure, ensure_eq, Addr, Deps, DepsMut, Env, Event, MessageInfo, Response, StdError,
    StdResult,
};
use cw_controllers::Admin;
use cw_storage_plus::Item;
use crate::ownership::ownership::{OwnerProposal, Ownership, query_owner, query_ownership_proposal, handle_claim_ownership, 
    handle_ownership_proposal, handle_ownership_proposal_rejection};
//use crate::errors::ContractError;
use crate::error::ContractError;

pub const MAX_DURATION: u64 = 604800u64;

pub const VAULT_OWNER: Admin = Admin::new("vault_owner");
//pub const ADAPTER_OWNER: Admin = Admin::new("adapter_owner");
//pub const STRATEGY_OWNER: Admin = Admin::new("strategy_owner");

pub const VAULT_PROPOSAL: Item<OwnerProposal> = Item::new("vault_proposal");
//pub const ADAPTER_PROPOSAL: Item<OwnerProposal> = Item::new("adapter_proposal");
//pub const STRATEGY_PROPOSAL: Item<OwnerProposal> = Item::new("strategy_proposal");


pub enum VaultState {
    Init,
    Running,
    Paused,
    Terminated,
}

pub struct Vault {
    state: VaultState,
    last_statechange_bh: u64, // last statechange block height
}

  

impl Vault {
    pub fn new() -> Self {
        Vault {
            state: VaultState::Init,
            last_statechange_bh: 0,
        }
    }

    pub fn update_state(&mut self, new_state: VaultState, block_height: u64) {
        self.state = new_state;
        self.last_statechange_bh = block_height;
        todo!() // storage save
    }
}
impl Ownership for Vault {
    fn handle_ownership_proposal(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        env: Env,
        proposed_owner: String,
        duration: u64,
        owner: &Admin,
        proposal: &Item<OwnerProposal>,
    ) -> Result<Response, ContractError> {
        handle_ownership_proposal(deps, info, env, proposed_owner, duration, owner, proposal)
    }

    fn handle_ownership_proposal_rejection(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        owner: &Admin,
        proposal: &Item<OwnerProposal>,
    ) -> Result<Response, ContractError> {
        handle_ownership_proposal_rejection(deps, info, owner, proposal)
    }

    fn handle_claim_ownership(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        env: Env,
        owner: &Admin,
        proposal: &Item<OwnerProposal>,
    ) -> Result<Response, ContractError> {
        handle_claim_ownership(deps, info, env, owner, proposal)
    }

    fn query_ownership_proposal(
        &self,
        deps: Deps,
        proposal: &Item<OwnerProposal>,
    ) -> StdResult<OwnerProposal> {
        query_ownership_proposal(deps, proposal)
    }

    fn query_owner(&self, deps: Deps, owner: &Admin) -> StdResult<Option<Addr>> {
        query_owner(deps, owner)
    }
}
