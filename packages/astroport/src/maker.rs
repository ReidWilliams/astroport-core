use crate::asset::{Asset, AssetInfo};
use crate::factory::UpdateAddr;
use cosmwasm_std::{Addr, Decimal, Uint128, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// ## Description
/// This structure describes the basic settings for creating a contract.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    /// contract address that used for controls settings for maker
    pub owner: String,
    /// the ASTRO token contract address
    pub astro_token_contract: String,
    /// the factory contract address
    pub factory_contract: String,
    /// the staking contract address
    pub staking_contract: String,
    /// the governance contract address
    pub governance_contract: Option<String>,
    /// the governance percent
    pub governance_percent: Option<Uint64>,
    /// the maximum spread
    pub max_spread: Option<Decimal>,
}

/// ## Description
/// This structure describes the execute messages of the contract.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Collects astro tokens from the given assets
    Collect {
        /// the assets to collect
        assets: Vec<AssetWithLimit>,
    },
    /// Updates general settings that contains in the  [`Config`]
    UpdateConfig {
        /// the factory contract address
        factory_contract: Option<String>,
        /// the staking contract address
        staking_contract: Option<String>,
        /// the governance contract address
        governance_contract: Option<UpdateAddr>,
        /// the governance percent
        governance_percent: Option<Uint64>,
        /// the maximum spread
        max_spread: Option<Decimal>,
    },
    /// Add bridges
    UpdateBridges {
        add: Option<Vec<(AssetInfo, AssetInfo)>>,
        remove: Option<Vec<AssetInfo>>,
    },
    /// Swap rewards via bridge assets
    SwapBridgeAssets { assets: Vec<AssetInfo>, depth: u64 },
    /// Distribute rewards in ASTRO tokens
    DistributeAstro {},
    /// Creates a request to change ownership.
    ProposeNewOwner {
        /// a new owner
        owner: String,
        /// the validity period of the offer to change the owner
        expires_in: u64,
    },
    /// Removes a request to change ownership.
    DropOwnershipProposal {},
    /// Approves ownership.
    ClaimOwnership {},
    /// Enables rewards collecting
    EnableRewards { blocks: u64 },
}

/// ## Description
/// This structure describes the query messages of the contract.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Returns information about the maker configs that contains in the [`Config`]
    Config {},
    /// Returns the balance for each asset in the specified input parameters
    Balances {
        assets: Vec<AssetInfo>,
    },
    Bridges {},
}

/// ## Description
/// A custom struct for each query response that returns controls settings of contract.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    /// Contract address that used for controls settings for factory, pools and tokenomics contracts
    pub owner: Addr,
    /// the ASTRO token contract address
    pub astro_token_contract: Addr,
    /// the factory contract address
    pub factory_contract: Addr,
    /// the staking contract address
    pub staking_contract: Addr,
    /// the governance contract address
    pub governance_contract: Option<Addr>,
    /// the governance percent
    pub governance_percent: Uint64,
    /// the maximum spread
    pub max_spread: Decimal,
    /// the remainder of pre-upgrade ASTRO fee
    pub remainder_reward: Uint128,
    /// the amount of collected ASTRO fee before enabling rewards distribution
    pub pre_upgrade_astro_amount: Uint128,
}

/// ## Description
/// A custom struct for each query response that returns the balance of asset.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BalancesResponse {
    pub balances: Vec<Asset>,
}

/// ## Description
/// This structure describes a migration message.
/// We currently take no arguments for migrations.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

/// ## Description
/// This enum describes asset with limits to be collected by maker.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AssetWithLimit {
    /// the available type of asset from [`AssetInfo`]
    pub info: AssetInfo,
    /// the amount of an asset
    pub limit: Option<Uint128>,
}
