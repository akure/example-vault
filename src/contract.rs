//use crate::config::MyConfig;
use crate::msg::{InstantiateMsg, QueryMsg, MigrateMsg, ExecuteMsg};
use crate::msg::ProExtensionExecuteMsg;
use crate::msg::ExtensionExecuteMsg;
//use crate::state::MyState;
use crate::error::ContractError;

use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, 
    StdResult, StdError,to_json_binary,
};
use osmosis_std::types::cosmwasm::wasm::v1::ExecuteContractProposal;
use osmosis_std::types::osmosis::cosmwasmpool::v1beta1::ExitPoolExecuteMsgRequest;
/*
use vaultenator::{
    admin::Administer,
    contract::Vaultenator,
    errors::ContractError,
    msg::{ExecuteMsg, MigrateMsg, QueryMsg},
    ownership::Own,
    query::Query,
    reply::ReplyHandler,
};
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {

    match msg {
        ExecuteMsg::Deposit { amount, recipient } => todo!(),
        ExecuteMsg::Redeem { recipient, amount } => todo!(),
        ExecuteMsg::VaultExtension(extension_msg) => {
            match extension_msg {
                // ExtensionExecuteMsg::ProExtensionExecuteMsg(pro_msg) => {
                ExtensionExecuteMsg::ProExtension(pro_msg) => {
                    match pro_msg {
                        ProExtensionExecuteMsg::MyVariant1{amount, recipient} => {

                        // ProExtensionExecuteMsg::MyVariant1 { amount, recipient } => {
                            // Implement MyVariant1 logic here
                            todo!()
                        },
                        ProExtensionExecuteMsg::CreateStrategy{} => {  todo!() },
                        ProExtensionExecuteMsg::UpdateRunningState{} =>{ todo!() },
                        ProExtensionExecuteMsg::UpdateStrategyOwner{} => { todo!() },
                        ProExtensionExecuteMsg::UpdateVaultOwner{} => { todo!() },
                        ProExtensionExecuteMsg::ExecStrategyActions{action} => {todo!() }

                    }
                }
                // Handle other possible ExtensionExecuteMsg variants here
                _ => todo!(),
            }
        }
    }
    Ok(Response::default())
}

// pub struct MyVault;

// # Custom trait implementations
//
// - Configure implemented in src/config.rs.
// - Describe implemented in src/describe.rs
// - ManageState implemented in src/state.rs.
// - Handle implemented in src/handle.rs.

// Default implementations taken from Vaultenator crate
/* 
impl Own for MyVault {}
impl Administer<MyState> for MyVault {}
impl Query<MyConfig, MyState> for MyVault {}
impl ReplyHandler<MyConfig> for MyVault {}
impl Vaultenator<MyConfig, MyState> for MyVault {}
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
    // MyVault.instantiate(deps, env, info, msg)
}
/* 
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
   msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
     // MyVault.execute(deps, env, info, msg)
}
*/


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {

     // Create a default response message
     let response = "Default response message";

     // Convert the response message to a binary format
     let binary_response = to_json_binary(&response)?;
 
     // Return the binary response
     Ok(binary_response)
}
/* 
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    // MyVault.query(deps, env, msg)
}
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
    // MyVault.reply(deps, env, msg)
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    // MyVault.migrate(deps, env, msg)
    Ok(Response::default())
}
