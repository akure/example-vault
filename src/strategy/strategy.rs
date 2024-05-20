use cosmwasm_std::{DepsMut, StdResult, Storage, Addr};
use crate::msg::StrategyAction;
use serde::{Serialize,Deserialize};
use cw_storage_plus::{Item, Map};


const ADAPTERS: Map<&str, bool> = Map::new("adapters");
const PRESET_RATIOS: Map<&str, u128> = Map::new("preset_ratios"); // TODO
pub const STRATEGY: Map<&[u8], Strategy> = Map::new("strategy");

// Strategy Key 
pub struct StrategyKey;


impl StrategyKey {
    pub fn new(id: u64) -> Vec<u8> {
        id.to_be_bytes().to_vec()
    }
}

// Stratey here takes the control of the fund movement from the contract treasury balance to 
// the pro vault adaptors as per the instructions sent to strategy module in the contract.
// Fund distribution could be based on preset ratio or sent via external trigger, which depends on how
// an external strategiest proposal followed by decentralised vote and execution of instructions. 
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Strategy {
    pub id: u64,
    pub name: String,
    pub description: String,
    // Add more fields as necessary
}

impl Strategy {
    pub fn execute_action(storage: &mut dyn Storage, action: StrategyAction) -> StdResult<()> {
        match action {
            StrategyAction::DistributeFundWithPresetAdaptorRatio => {
                // Implementation for distributing funds with preset ratios
                Ok(())
            }
            StrategyAction::DistributeFundWithCustomAdaptorRatios { custom_ratios } => {
                Self::distribute_funds_with_custom_ratios(storage, custom_ratios)
            }
            StrategyAction::RemoveAdaptor { adaptor } => {
                ADAPTERS.remove(storage, adaptor.as_str());
                Ok(())
            }
            StrategyAction::AddNewAdaptor { adaptor } => {
                Self::add_adapter(storage, Addr::unchecked(adaptor))
            }
            StrategyAction::UpdateStrategyParams => {
                // Placeholder for updating strategy parameters
                Ok(())
            }
            StrategyAction::UpdateAdaptorRunningState { adaptor } => {
                // Placeholder for updating adaptor running state
                Ok(())
            }
            StrategyAction::UpdateStrategyRunningState => {
                // Placeholder for updating strategy running state
                Ok(())
            }
        }
    }

    // TODO - Adaptor object string and type check should be done here instead of Addr.
    // For simplification, there should be only one adaptor of one adaptor type. So maximum one
    // instance of CLVault, maximum one instance DebtMarket adaptor, and max one for the 
    // Swap Market. 
    pub fn add_adapter(storage: &mut dyn Storage, adapter: Addr) -> StdResult<()> {
        if ADAPTERS.has(storage, adapter.as_str()) {
            Err(cosmwasm_std::StdError::generic_err("Adapter already exists"))
        } else {
            ADAPTERS.save(storage, adapter.as_str(), &true)?;
            Ok(())
        }
    }

    pub fn distribute_funds_with_custom_ratios(storage: &mut dyn Storage, custom_ratios: String) -> StdResult<()> {
        // Parse custom_ratios and distribute funds accordingly
        Ok(())
    }

    pub fn distribute_funds(total_funds: u128, ratios: Vec<u128>) -> Vec<u128> {
        let sum_ratios: u128 = ratios.iter().sum();
        ratios.iter().map(|r| total_funds * r / sum_ratios).collect()
    }
}
