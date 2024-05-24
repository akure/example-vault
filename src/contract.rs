//use crate::config::MyConfig;
use crate::msg::{InstantiateMsg, QueryMsg, MigrateMsg, ExecuteMsg, StrategyAction};
use crate::msg::ProExtensionExecuteMsg;
use crate::msg::ExtensionExecuteMsg;
use crate::strategy::strategy::{Strategy, StrategyKey, STRATEGY};
//use crate::state::MyState;
use crate::error::ContractError;
use crate::vault::provault::{VaultRunningState, VAULT_STATE, VAULT_OWNER,
    query_all_strategies, query_vault_running_state, Vault};
use crate::vault::config::{query_vault_config, VAULT_CONFIG, Config};

use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, 
    StdResult, StdError,to_json_binary,Coin, Uint128, BankMsg,CosmosMsg
};
// use osmosis_std::types::cosmwasm::wasm::v1::ExecuteContractProposal;
// use osmosis_std::types::osmosis::cosmwasmpool::v1beta1::ExitPoolExecuteMsgRequest;
 
// TODO - Locality of local variables to be strucured, that will reduce number of imports from
// other modules and will make the contract.rs clean to read and maintain.

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {

    match msg {
        ExecuteMsg::Deposit { amount, recipient } => { 
            try_deposit(deps, env, info, amount, recipient);},
        ExecuteMsg::Redeem { recipient, amount } => todo!(),
        ExecuteMsg::VaultExtension(extension_msg) => {
            match extension_msg {
                 ExtensionExecuteMsg::ProExtension(pro_msg) => {
                    match pro_msg {
                        // MyVariant1 is a test one.
                        ProExtensionExecuteMsg::MyVariant1{amount, recipient} => {
                            try_my_variant1(deps, amount, recipient);},
                        ProExtensionExecuteMsg::CreateStrategy{name,description} => {
                            try_create_strategy(deps, env, info, name, description); },
                        ProExtensionExecuteMsg::UpdateRunningState{new_state} => {
                            try_update_running_state(deps, env, info, new_state); },
                        ProExtensionExecuteMsg::UpdateStrategyOwner{} => { 
                            try_update_strategy_owner(deps); },
                        ProExtensionExecuteMsg::UpdateVaultOwner{} => { 
                            try_update_vault_owner(deps); },
                        ProExtensionExecuteMsg::ExecStrategyActions{action} => { 
                            try_exec_strategy_actions(deps, action); }
                    }
                }
                // Handle other possible ExtensionExecuteMsg variants here
                _ => todo!(),
            }
        }
    }
    Ok(Response::default())
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    VAULT_OWNER.set(deps.branch(), Some(info.sender.clone()))?;


    let config = Config {
        max_deposit_cap: msg.provault_config.max_deposit_cap,
        deposit_denom: msg.provault_config.deposit_denom,
        share_denom: msg.provault_config.share_denom,
        max_strategy_inst: msg.provault_config.max_strategy_inst,
        admin: msg.provault_config.admin,
    };

    VAULT_CONFIG.save(deps.storage, &config)?;


    // Initialize the vault state
    let mut vault = Vault::new();
    let response = vault.update_state(deps, env, info, VaultRunningState::Init)?;
    
    // TODO - PROPER EVENT HANDLING AND ERROR PROPAGATION.
    Ok(Response::new().add_attribute("method", "instantiate"))
    // Ok(Response::default())
 }
 

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {

    match msg {
        QueryMsg::GetAllStrategies {} => query_all_strategies(deps),
        QueryMsg::GetVaultConfig {} => query_vault_config(deps),
        QueryMsg::GetVaultRunningState {} => query_vault_running_state(deps),
        // Handle other queries ...
    }

    /* 
     // Create a default response message
     let response = "Default response message";

     // Convert the response message to a binary format
     let binary_response = to_json_binary(&response)?;
 
     // Return the binary response
     Ok(binary_response)
     */
}
/* 
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    // MyVault.query(deps, env, msg)
}
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
     // TODO - REPLY TO BE IMPLEMENTED
     Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    // MyVault.migrate(deps, env, msg)
    Ok(Response::default())
}


// Helpers methods for execute entry points
/* 
fn try_add_strategy_adapter(
    deps: DepsMut,
    adapter: String,
) -> Result<Response, ContractError> {
    let adapter_addr = deps.api.addr_validate(&adapter)?;
    let added = Strategy::add_adapter(deps.storage, adapter_addr)?;
    let status = if added { "adapter added" } else { "adapter already exists" };

    Ok(Response::new()
        .add_attribute("method", "try_add_strategy_adapter")
        .add_attribute("status", status))
}

fn try_distribute_strategy_funds(
    deps: DepsMut,
    total_funds: u128,
    ratios: Vec<u128>,
) -> Result<Response, ContractError> {
    let distributions = Strategy::distribute_funds(total_funds, ratios);
    Ok(Response::new()
        .add_attribute("method", "try_distribute_strategy_funds")
        .add_attribute("distributions", format!("{:?}", distributions)))
}
*/


fn try_my_variant1(
    deps: DepsMut,
    amount: cosmwasm_std::Uint128,
    recipient: Option<String>,
) -> Result<Response, ContractError> {
    // Implementation for MyVariant1
    Ok(Response::new()
        .add_attribute("method", "try_my_variant1"))
}

fn try_update_running_state(
    deps: DepsMut, env: Env, info: MessageInfo, new_state: VaultRunningState) 
    -> Result<Response, ContractError> {
    let mut vault = VAULT_STATE.load(deps.storage)?;
    vault.update_state(deps, env, info, new_state);

    Ok(Response::new()
        .add_attribute("method", "try_update_running_state"))
}

fn try_deposit(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
    recipient: Option<String>, // Used for future 
) -> Result<Response, ContractError> {
    
    // Check if the sent funds match the requested deposit amount
    if info.funds.len() != 1 || info.funds[0].amount != amount {
        return Err(ContractError::Std(StdError::generic_err("Incorrect deposit amount")));
    }

    // TODO - Whitelist denom verification. Whitelist should be added in initialise message too. 
    //        there should be a separate update whitelist exection as well.
    // Ensure the denom matches
    if info.funds[0].denom != "uosmo" { // TODO - Replace with the appropriate denom
        return Err(ContractError::Std(StdError::generic_err("Incorrect denom")));
    }

    // Construct the bank message to transfer the funds to the contract's address
    let bank_msg = BankMsg::Send {
        to_address: env.contract.address.to_string(),
        amount: vec![info.funds[0].clone()],
    };

    // TODO - share calculation and allocation to be done on reply handler on successful deposit. 

    Ok(Response::new()
        .add_message(CosmosMsg::Bank(bank_msg))
        .add_attribute("method", "try_deposit")
        .add_attribute("amount", amount.to_string())
        .add_attribute("sender", info.sender.to_string())
        .add_attribute("recipient", env.contract.address.to_string()))
}

fn try_update_vault_owner(
    deps: DepsMut,
) -> Result<Response, ContractError> {
    // Implementation for UpdateVaultOwner
    Ok(Response::new()
        .add_attribute("method", "try_update_vault_owner"))
}

fn try_update_strategy_owner(
    deps: DepsMut,
) -> Result<Response, ContractError> {
    // Implementation for UpdateStrategyOwner
    Ok(Response::new()
        .add_attribute("method", "try_update_strategy_owner"))
}


// Function to create a strategy with ID 1
pub fn try_create_strategy(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    name: String,
    description: String,
) -> StdResult<Response> {
    // TODO - Validation checks to be added.
    // Initially, for the simplicity only one instance of strategy should be supported 
    // within one provault contract. 
    // TODO - Other parameters to be added soon.
    let strategy = Strategy {
        id: 1,
        name,
        description,
    };

    STRATEGY.save(deps.storage, &StrategyKey::new(1), &strategy)?;

    Ok(Response::new()
        .add_attribute("action", "create_strategy")
        .add_attribute("strategy_id", "1"))
}

fn try_exec_strategy_actions(
    deps: DepsMut,
    action: StrategyAction,
) -> Result<Response, ContractError> {
    Strategy::execute_action(deps.storage, action)?;
    Ok(Response::new()
        .add_attribute("method", "try_exec_strategy_actions"))
}

