use cosmwasm_schema::cw_serde;
use cw_vault_standard::{VaultStandardExecuteMsg, VaultStandardQueryMsg};
use cosmwasm_std::Uint128;

#[cw_serde]
pub struct InstantiateMsg {
    pub base_denom: String,
}

#[cw_serde]
pub enum QueryMsg {}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub enum ProExtensionExecuteMsg {
    MyVariant1 {
        /// The amount of base tokens to deposit.
        amount: Uint128,
        /// The optional recipient of the vault token. If not set, the caller
        /// address will be used instead.
        recipient: Option<String>,
    },

    UpdateRunningState {
        // Placeholder for running state details
    },

    UpdateVaultOwner {
        // Placeholder for vault owner details
    },

    UpdateStrategyOwner {
        // Placeholder for strategy owner details
    },

    CreateStrategy {
        // Placeholder for creating strategy
        // Adding adaptors, configuring adaptors, adding Strategy Control Owner, Adaptor Control Owner
    },

    ExecStrategyActions { 
        action : StrategyAction,
    },
}

#[cw_serde]
pub enum StrategyAction {
    DistributeFundWithPresetAdaptorRatio, // Distributing funds across adaptors as per preset ratios
    DistributeFundWithCustomAdaptorRatios { custom_ratios: String }, // CustomAdaptorRatio (A1:R1, A2:R2, A3:R3)
    RemoveAdaptor { adaptor: String }, // Remove Adaptor Ai
    AddNewAdaptor { adaptor: String }, // Add a new adaptor of type Ai. Should fail if already one is present of type A1.
    UpdateStrategyParams {
        // Placeholder for updating strategy parameters
        // e.g., update ratio, remove adaptor, enable/disable strategy or adaptor
    },
    UpdateAdaptorRunningState { adaptor: String },
    UpdateStrategyRunningState,
}

#[cw_serde]
pub enum ExtensionExecuteMsg {
    ProExtension(ProExtensionExecuteMsg),
}

/// ExecuteMsg
pub type ExecuteMsg = VaultStandardExecuteMsg<ExtensionExecuteMsg>;
