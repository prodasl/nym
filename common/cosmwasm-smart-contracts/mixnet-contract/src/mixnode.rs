// Copyright 2021-2023 - Nym Technologies SA <contact@nymtech.net>
// SPDX-License-Identifier: Apache-2.0

// due to code generated by JsonSchema
#![allow(clippy::field_reassign_with_default)]

use crate::constants::{TOKEN_SUPPLY, UNIT_DELEGATION_BASE};
use crate::error::MixnetContractError;
use crate::helpers::IntoBaseDecimal;
use crate::reward_params::{NodeRewardParams, RewardingParams};
use crate::rewarding::helpers::truncate_reward;
use crate::rewarding::RewardDistribution;
use crate::{Delegation, EpochEventId, EpochId, IdentityKey, MixId, Percent, SphinxKey};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Decimal, StdResult, Uint128};
use schemars::JsonSchema;
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Current state of given node in the rewarded set.
#[cfg_attr(feature = "generate-ts", derive(ts_rs::TS))]
#[cfg_attr(
    feature = "generate-ts",
    ts(export_to = "ts-packages/types/src/types/rust/RewardedSetNodeStatus.ts")
)]
#[cw_serde]
#[derive(Copy)]
pub enum RewardedSetNodeStatus {
    /// Node that is currently active, i.e. is expected to be used by clients for mixing packets.
    Active,

    /// Node that is currently in standby, i.e. it's present in the rewarded set but is not active.
    Standby,
}

impl RewardedSetNodeStatus {
    pub fn is_active(&self) -> bool {
        matches!(self, RewardedSetNodeStatus::Active)
    }
}

/// Full details associated with given mixnode.
#[cw_serde]
pub struct MixNodeDetails {
    /// Basic bond information of this mixnode, such as owner address, original pledge, etc.
    pub bond_information: MixNodeBond,

    /// Details used for computation of rewarding related data.
    pub rewarding_details: MixNodeRewarding,

    /// Adjustments to the mixnode that are ought to happen during future epoch transitions.
    #[serde(default)]
    pub pending_changes: PendingMixNodeChanges,
}

impl MixNodeDetails {
    pub fn new(
        bond_information: MixNodeBond,
        rewarding_details: MixNodeRewarding,
        pending_changes: PendingMixNodeChanges,
    ) -> Self {
        MixNodeDetails {
            bond_information,
            rewarding_details,
            pending_changes,
        }
    }

    pub fn mix_id(&self) -> MixId {
        self.bond_information.mix_id
    }

    pub fn layer(&self) -> Layer {
        self.bond_information.layer
    }

    pub fn is_unbonding(&self) -> bool {
        self.bond_information.is_unbonding
    }

    pub fn original_pledge(&self) -> &Coin {
        &self.bond_information.original_pledge
    }

    pub fn pending_operator_reward(&self) -> Coin {
        let pledge = self.original_pledge();
        self.rewarding_details.pending_operator_reward(pledge)
    }

    pub fn pending_detailed_operator_reward(&self) -> StdResult<Decimal> {
        let pledge = self.original_pledge();
        self.rewarding_details
            .pending_detailed_operator_reward(pledge)
    }

    pub fn total_stake(&self) -> Decimal {
        self.rewarding_details.node_bond()
    }

    pub fn pending_pledge_change(&self) -> Option<EpochEventId> {
        self.pending_changes.pledge_change
    }
}

#[cw_serde]
pub struct MixNodeRewarding {
    /// Information provided by the operator that influence the cost function.
    pub cost_params: MixNodeCostParams,

    /// Total pledge and compounded reward earned by the node operator.
    pub operator: Decimal,

    /// Total delegation and compounded reward earned by all node delegators.
    pub delegates: Decimal,

    /// Cumulative reward earned by the "unit delegation" since the block 0.
    pub total_unit_reward: Decimal,

    /// Value of the theoretical "unit delegation" that has delegated to this mixnode at block 0.
    pub unit_delegation: Decimal,

    /// Marks the epoch when this node was last rewarded so that we wouldn't accidentally attempt
    /// to reward it multiple times in the same epoch.
    pub last_rewarded_epoch: EpochId,

    // technically we don't need that field to determine reward magnitude or anything
    // but it saves on extra queries to determine if we're removing the final delegation
    // (so that we could zero the field correctly)
    pub unique_delegations: u32,
}

impl MixNodeRewarding {
    pub fn initialise_new(
        cost_params: MixNodeCostParams,
        initial_pledge: &Coin,
        current_epoch: EpochId,
    ) -> Result<Self, MixnetContractError> {
        assert!(
            initial_pledge.amount <= TOKEN_SUPPLY,
            "pledge cannot be larger than the token supply"
        );

        Ok(MixNodeRewarding {
            cost_params,
            operator: initial_pledge.amount.into_base_decimal()?,
            delegates: Decimal::zero(),
            total_unit_reward: Decimal::zero(),
            unit_delegation: UNIT_DELEGATION_BASE,
            last_rewarded_epoch: current_epoch,
            unique_delegations: 0,
        })
    }

    /// Determines whether this node is still bonded. This is performed via a simple check,
    /// if there are no tokens left associated with the operator, it means they have unbonded
    /// and those params only exist for the purposes of calculating rewards for delegators that
    /// have not yet removed their tokens.
    pub fn still_bonded(&self) -> bool {
        self.operator != Decimal::zero()
    }

    pub fn pending_operator_reward(&self, original_pledge: &Coin) -> Coin {
        let reward_with_pledge = truncate_reward(self.operator, &original_pledge.denom);
        Coin {
            denom: reward_with_pledge.denom,
            amount: reward_with_pledge.amount - original_pledge.amount,
        }
    }

    pub fn pending_detailed_operator_reward(&self, original_pledge: &Coin) -> StdResult<Decimal> {
        let initial_dec = original_pledge.amount.into_base_decimal()?;
        if initial_dec > self.operator {
            panic!(
                "seems slashing has occurred while it has not been implemented nor accounted for!"
            )
        }
        Ok(self.operator - initial_dec)
    }

    pub fn operator_pledge_with_reward(&self, denom: impl Into<String>) -> Coin {
        truncate_reward(self.operator, denom)
    }

    pub fn pending_delegator_reward(&self, delegation: &Delegation) -> StdResult<Coin> {
        let delegator_reward = self.determine_delegation_reward(delegation)?;
        Ok(truncate_reward(delegator_reward, &delegation.amount.denom))
    }

    pub fn withdraw_operator_reward(
        &mut self,
        original_pledge: &Coin,
    ) -> Result<Coin, MixnetContractError> {
        let initial_dec = original_pledge.amount.into_base_decimal()?;
        if initial_dec > self.operator {
            panic!(
                "seems slashing has occurred while it has not been implemented nor accounted for!"
            )
        }
        let diff = self.operator - initial_dec;
        self.operator = initial_dec;

        Ok(truncate_reward(diff, &original_pledge.denom))
    }

    pub fn withdraw_delegator_reward(
        &mut self,
        delegation: &mut Delegation,
    ) -> Result<Coin, MixnetContractError> {
        let reward = self.determine_delegation_reward(delegation)?;
        self.decrease_delegates_decimal(reward)?;

        delegation.cumulative_reward_ratio = self.full_reward_ratio();
        Ok(truncate_reward(reward, &delegation.amount.denom))
    }

    pub fn node_bond(&self) -> Decimal {
        self.operator + self.delegates
    }

    /// Saturation over the tokens pledged by the node operator.
    pub fn pledge_saturation(&self, reward_params: &RewardingParams) -> Decimal {
        // make sure our saturation is never greater than 1
        if self.operator > reward_params.interval.stake_saturation_point {
            Decimal::one()
        } else {
            self.operator / reward_params.interval.stake_saturation_point
        }
    }

    /// Saturation over all the tokens staked over this node.
    pub fn bond_saturation(&self, reward_params: &RewardingParams) -> Decimal {
        // make sure our saturation is never greater than 1
        if self.node_bond() > reward_params.interval.stake_saturation_point {
            Decimal::one()
        } else {
            self.node_bond() / reward_params.interval.stake_saturation_point
        }
    }

    pub fn uncapped_bond_saturation(&self, reward_params: &RewardingParams) -> Decimal {
        self.node_bond() / reward_params.interval.stake_saturation_point
    }

    pub fn node_reward(
        &self,
        reward_params: &RewardingParams,
        node_params: NodeRewardParams,
    ) -> Decimal {
        let work = if node_params.in_active_set {
            reward_params.active_node_work()
        } else {
            reward_params.standby_node_work()
        };

        let alpha = reward_params.interval.sybil_resistance;

        reward_params.interval.epoch_reward_budget
            * node_params.performance.value()
            * self.bond_saturation(reward_params)
            * (work
                + alpha.value() * self.pledge_saturation(reward_params)
                    / reward_params.dec_rewarded_set_size())
            / (Decimal::one() + alpha.value())
    }

    pub fn determine_reward_split(
        &self,
        node_reward: Decimal,
        node_performance: Percent,
        // I don't like this argument here, makes things look, idk, messy...
        epochs_in_interval: u32,
    ) -> RewardDistribution {
        let node_cost =
            self.cost_params.epoch_operating_cost(epochs_in_interval) * node_performance.value();

        // check if profit is positive
        if node_reward > node_cost {
            let profit = node_reward - node_cost;
            let profit_margin = self.cost_params.profit_margin_percent.value();
            let one = Decimal::one();

            let operator_share = self.operator / self.node_bond();

            let operator = profit * (profit_margin + (one - profit_margin) * operator_share);
            let delegates = profit - operator;

            debug_assert_eq!(operator + delegates + node_cost, node_reward);

            RewardDistribution {
                operator: operator + node_cost,
                delegates,
            }
        } else {
            RewardDistribution {
                operator: node_reward,
                delegates: Decimal::zero(),
            }
        }
    }

    pub fn calculate_epoch_reward(
        &self,
        reward_params: &RewardingParams,
        node_params: NodeRewardParams,
        epochs_in_interval: u32,
    ) -> RewardDistribution {
        let node_reward = self.node_reward(reward_params, node_params);
        self.determine_reward_split(node_reward, node_params.performance, epochs_in_interval)
    }

    pub fn distribute_rewards(
        &mut self,
        distribution: RewardDistribution,
        absolute_epoch_id: EpochId,
    ) {
        let unit_delegation_reward = distribution.delegates
            * self.delegator_share(self.unit_delegation + self.total_unit_reward);

        self.operator += distribution.operator;
        self.delegates += distribution.delegates;

        // self.current_period_reward += unit_delegation_reward;
        self.total_unit_reward += unit_delegation_reward;
        self.last_rewarded_epoch = absolute_epoch_id;
    }

    pub fn epoch_rewarding(
        &mut self,
        reward_params: &RewardingParams,
        node_params: NodeRewardParams,
        epochs_in_interval: u32,
        absolute_epoch_id: EpochId,
    ) {
        let reward_distribution =
            self.calculate_epoch_reward(reward_params, node_params, epochs_in_interval);
        self.distribute_rewards(reward_distribution, absolute_epoch_id)
    }

    pub fn determine_delegation_reward(&self, delegation: &Delegation) -> StdResult<Decimal> {
        let starting_ratio = delegation.cumulative_reward_ratio;
        let ending_ratio = self.full_reward_ratio();
        let adjust = starting_ratio + self.unit_delegation;

        Ok((ending_ratio - starting_ratio) * delegation.dec_amount()? / adjust)
    }

    // this updates `unique_delegations` field
    pub fn add_base_delegation(&mut self, amount: Uint128) -> Result<(), MixnetContractError> {
        self.increase_delegates_uint128(amount)?;
        self.unique_delegations += 1;
        Ok(())
    }

    pub fn increase_operator_uint128(
        &mut self,
        amount: Uint128,
    ) -> Result<(), MixnetContractError> {
        self.operator += amount.into_base_decimal()?;
        Ok(())
    }

    /// Decreases total pledge of operator by the specified amount.
    pub fn decrease_operator_uint128(
        &mut self,
        amount: Uint128,
    ) -> Result<(), MixnetContractError> {
        let amount_decimal = amount.into_base_decimal()?;
        if self.operator < amount_decimal {
            return Err(MixnetContractError::OverflowDecimalSubtraction {
                minuend: self.operator,
                subtrahend: amount_decimal,
            });
        }
        self.operator -= amount_decimal;
        Ok(())
    }

    pub fn increase_delegates_uint128(
        &mut self,
        amount: Uint128,
    ) -> Result<(), MixnetContractError> {
        self.delegates += amount.into_base_decimal()?;
        Ok(())
    }

    // this updates `unique_delegations` field
    // special care must be taken when calling this method as the caller has to ensure
    // the corresponding delegation has not accumulated any rewards
    pub fn remove_delegation_uint128(
        &mut self,
        amount: Uint128,
    ) -> Result<(), MixnetContractError> {
        self.decrease_delegates_uint128(amount)?;
        self.decrement_unique_delegations()
    }

    pub fn decrease_delegates_uint128(
        &mut self,
        amount: Uint128,
    ) -> Result<(), MixnetContractError> {
        let amount_dec = amount.into_base_decimal()?;
        self.decrease_delegates_decimal(amount_dec)
    }

    fn decrement_unique_delegations(&mut self) -> Result<(), MixnetContractError> {
        if self.unique_delegations == 0 {
            return Err(MixnetContractError::OverflowSubtraction {
                minuend: 0,
                subtrahend: 1,
            });
        }
        self.unique_delegations -= 1;
        Ok(())
    }

    // this updates `unique_delegations` field
    pub fn remove_delegation_decimal(
        &mut self,
        amount: Decimal,
    ) -> Result<(), MixnetContractError> {
        self.decrease_delegates_decimal(amount)?;
        self.decrement_unique_delegations()?;

        // if this was last delegation, move all leftover decimal tokens to the operator
        // (this is literally in the order of a millionth of a micronym)
        if self.unique_delegations == 0 {
            self.operator += self.delegates;
            self.delegates = Decimal::zero();
        }
        Ok(())
    }

    pub fn undelegate(&mut self, delegation: &Delegation) -> Result<Coin, MixnetContractError> {
        let reward = self.determine_delegation_reward(delegation)?;
        let full_amount = reward + delegation.dec_amount()?;
        self.remove_delegation_decimal(full_amount)?;
        Ok(truncate_reward(full_amount, &delegation.amount.denom))
    }

    pub fn decrease_delegates_decimal(
        &mut self,
        amount: Decimal,
    ) -> Result<(), MixnetContractError> {
        if self.delegates < amount {
            return Err(MixnetContractError::OverflowDecimalSubtraction {
                minuend: self.delegates,
                subtrahend: amount,
            });
        }

        self.delegates -= amount;
        Ok(())
    }

    pub fn decrease_operator_decimal(
        &mut self,
        amount: Decimal,
    ) -> Result<(), MixnetContractError> {
        if self.operator < amount {
            return Err(MixnetContractError::OverflowDecimalSubtraction {
                minuend: self.operator,
                subtrahend: amount,
            });
        }

        self.operator -= amount;
        Ok(())
    }

    pub fn full_reward_ratio(&self) -> Decimal {
        self.total_unit_reward //+ self.current_period_reward
    }

    pub fn delegator_share(&self, amount: Decimal) -> Decimal {
        if self.delegates.is_zero() {
            Decimal::zero()
        } else {
            amount / self.delegates
        }
    }
}

/// Basic mixnode information provided by the node operator.
#[cw_serde]
pub struct MixNodeBond {
    /// Unique id assigned to the bonded mixnode.
    pub mix_id: MixId,

    /// Address of the owner of this mixnode.
    pub owner: Addr,

    /// Original amount pledged by the operator of this node.
    pub original_pledge: Coin,

    /// Layer assigned to this mixnode.
    pub layer: Layer,

    /// Information provided by the operator for the purposes of bonding.
    pub mix_node: MixNode,

    /// Entity who bonded this mixnode on behalf of the owner.
    /// If exists, it's most likely the address of the vesting contract.
    pub proxy: Option<Addr>,

    /// Block height at which this mixnode has been bonded.
    pub bonding_height: u64,

    /// Flag to indicate whether this node is in the process of unbonding,
    /// that will conclude upon the epoch finishing.
    pub is_unbonding: bool,
}

impl MixNodeBond {
    pub fn new(
        mix_id: MixId,
        owner: Addr,
        original_pledge: Coin,
        layer: Layer,
        mix_node: MixNode,
        proxy: Option<Addr>,
        bonding_height: u64,
    ) -> Self {
        MixNodeBond {
            mix_id,
            owner,
            original_pledge,
            layer,
            mix_node,
            proxy,
            bonding_height,
            is_unbonding: false,
        }
    }

    pub fn identity(&self) -> &str {
        &self.mix_node.identity_key
    }

    pub fn original_pledge(&self) -> &Coin {
        &self.original_pledge
    }

    pub fn owner(&self) -> &Addr {
        &self.owner
    }

    pub fn mix_node(&self) -> &MixNode {
        &self.mix_node
    }
}

/// Information provided by the node operator during bonding that are used to allow other entities to use the services of this node.
#[cw_serde]
#[cfg_attr(feature = "generate-ts", derive(ts_rs::TS))]
#[cfg_attr(
    feature = "generate-ts",
    ts(export_to = "ts-packages/types/src/types/rust/Mixnode.ts")
)]
pub struct MixNode {
    /// Network address of this mixnode, for example 1.1.1.1 or foo.mixnode.com
    pub host: String,

    /// Port used by this mixnode for listening for mix packets.
    pub mix_port: u16,

    /// Port used by this mixnode for listening for verloc requests.
    pub verloc_port: u16,

    /// Port used by this mixnode for its http(s) API
    pub http_api_port: u16,

    /// Base58-encoded x25519 public key used for sphinx key derivation.
    pub sphinx_key: SphinxKey,

    /// Base58-encoded ed25519 EdDSA public key.
    pub identity_key: IdentityKey,

    /// The self-reported semver version of this mixnode.
    pub version: String,
}

/// The cost parameters, or the cost function, defined for the particular mixnode that influences
/// how the rewards should be split between the node operator and its delegators.
#[cw_serde]
pub struct MixNodeCostParams {
    /// The profit margin of the associated mixnode, i.e. the desired percent of the reward to be distributed to the operator.
    pub profit_margin_percent: Percent,

    /// Operating cost of the associated mixnode per the entire interval.
    pub interval_operating_cost: Coin,
}

impl MixNodeCostParams {
    pub fn to_inline_json(&self) -> String {
        serde_json_wasm::to_string(self).unwrap_or_else(|_| "serialisation failure".into())
    }
}

impl MixNodeCostParams {
    pub fn epoch_operating_cost(&self, epochs_in_interval: u32) -> Decimal {
        Decimal::from_ratio(self.interval_operating_cost.amount, epochs_in_interval)
    }
}

#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize_repr,
    Deserialize_repr,
    JsonSchema,
)]
#[repr(u8)]
pub enum Layer {
    One = 1,
    Two = 2,
    Three = 3,
}

impl From<Layer> for String {
    fn from(layer: Layer) -> Self {
        (layer as u8).to_string()
    }
}

impl TryFrom<u8> for Layer {
    type Error = MixnetContractError;

    fn try_from(i: u8) -> Result<Layer, MixnetContractError> {
        match i {
            1 => Ok(Layer::One),
            2 => Ok(Layer::Two),
            3 => Ok(Layer::Three),
            _ => Err(MixnetContractError::InvalidLayer(i)),
        }
    }
}

impl From<Layer> for u8 {
    fn from(layer: Layer) -> u8 {
        match layer {
            Layer::One => 1,
            Layer::Two => 2,
            Layer::Three => 3,
        }
    }
}

#[cfg_attr(feature = "generate-ts", derive(ts_rs::TS))]
#[cfg_attr(
    feature = "generate-ts",
    ts(export_to = "ts-packages/types/src/types/rust/PendingMixnodeChanges.ts")
)]
#[cw_serde]
#[derive(Default, Copy)]
pub struct PendingMixNodeChanges {
    pub pledge_change: Option<EpochEventId>,
    // pub cost_params_change: Option<IntervalEventId>,
}

impl PendingMixNodeChanges {
    pub fn new_empty() -> PendingMixNodeChanges {
        PendingMixNodeChanges {
            pledge_change: None,
        }
    }
}

/// Basic information of a node that used to be part of the mix network but has already unbonded.
#[cfg_attr(feature = "generate-ts", derive(ts_rs::TS))]
#[cfg_attr(
    feature = "generate-ts",
    ts(export_to = "ts-packages/types/src/types/rust/UnbondedMixnode.ts")
)]
#[cw_serde]
pub struct UnbondedMixnode {
    /// Base58-encoded ed25519 EdDSA public key.
    pub identity_key: IdentityKey,

    /// Address of the owner of this mixnode.
    #[cfg_attr(feature = "generate-ts", ts(type = "string"))]
    pub owner: Addr,

    /// Entity who bonded this mixnode on behalf of the owner.
    /// If exists, it's most likely the address of the vesting contract.
    #[cfg_attr(feature = "generate-ts", ts(type = "string | null"))]
    pub proxy: Option<Addr>,

    /// Block height at which this mixnode has unbonded.
    #[cfg_attr(feature = "generate-ts", ts(type = "number"))]
    pub unbonding_height: u64,
}

#[cfg_attr(feature = "generate-ts", derive(ts_rs::TS))]
#[cfg_attr(
    feature = "generate-ts",
    ts(export_to = "ts-packages/types/src/types/rust/MixNodeConfigUpdate.ts")
)]
#[cw_serde]
pub struct MixNodeConfigUpdate {
    pub host: String,
    pub mix_port: u16,
    pub verloc_port: u16,
    pub http_api_port: u16,
    pub version: String,
}

impl MixNodeConfigUpdate {
    pub fn to_inline_json(&self) -> String {
        serde_json_wasm::to_string(self).unwrap_or_else(|_| "serialisation failure".into())
    }
}

/// Response containing paged list of all mixnode bonds in the contract.
#[cw_serde]
pub struct PagedMixnodeBondsResponse {
    /// The mixnode bond information present in the contract.
    pub nodes: Vec<MixNodeBond>,

    /// Maximum number of entries that could be included in a response. `per_page <= nodes.len()`
    // this field is rather redundant and should be deprecated.
    pub per_page: usize,

    /// Field indicating paging information for the following queries if the caller wishes to get further entries.
    pub start_next_after: Option<MixId>,
}

impl PagedMixnodeBondsResponse {
    pub fn new(nodes: Vec<MixNodeBond>, per_page: usize, start_next_after: Option<MixId>) -> Self {
        PagedMixnodeBondsResponse {
            nodes,
            per_page,
            start_next_after,
        }
    }
}

/// Response containing paged list of all mixnode details in the contract.
#[cw_serde]
pub struct PagedMixnodesDetailsResponse {
    /// All mixnode details stored in the contract.
    /// Apart from the basic bond information it also contains details required for all future reward calculation
    /// as well as any pending changes requested by the operator.
    pub nodes: Vec<MixNodeDetails>,

    /// Maximum number of entries that could be included in a response. `per_page <= nodes.len()`
    // this field is rather redundant and should be deprecated.
    pub per_page: usize,

    /// Field indicating paging information for the following queries if the caller wishes to get further entries.
    pub start_next_after: Option<MixId>,
}

impl PagedMixnodesDetailsResponse {
    pub fn new(
        nodes: Vec<MixNodeDetails>,
        per_page: usize,
        start_next_after: Option<MixId>,
    ) -> Self {
        PagedMixnodesDetailsResponse {
            nodes,
            per_page,
            start_next_after,
        }
    }
}

/// Response containing paged list of all mixnodes that have ever unbonded.
#[cw_serde]
pub struct PagedUnbondedMixnodesResponse {
    /// The past ids of unbonded mixnodes alongside their basic information such as the owner or the identity key.
    pub nodes: Vec<(MixId, UnbondedMixnode)>,

    /// Maximum number of entries that could be included in a response. `per_page <= nodes.len()`
    // this field is rather redundant and should be deprecated.
    pub per_page: usize,

    /// Field indicating paging information for the following queries if the caller wishes to get further entries.
    pub start_next_after: Option<MixId>,
}

impl PagedUnbondedMixnodesResponse {
    pub fn new(
        nodes: Vec<(MixId, UnbondedMixnode)>,
        per_page: usize,
        start_next_after: Option<MixId>,
    ) -> Self {
        PagedUnbondedMixnodesResponse {
            nodes,
            per_page,
            start_next_after,
        }
    }
}

/// Response containing details of a mixnode belonging to the particular owner.
#[cw_serde]
pub struct MixOwnershipResponse {
    /// Validated address of the mixnode owner.
    pub address: Addr,

    /// If the provided address owns a mixnode, this field contains its detailed information.
    pub mixnode_details: Option<MixNodeDetails>,
}

/// Response containing details of a mixnode with the provided id.
#[cw_serde]
pub struct MixnodeDetailsResponse {
    /// Id of the requested mixnode.
    pub mix_id: MixId,

    /// If there exists a mixnode with the provided id, this field contains its detailed information.
    pub mixnode_details: Option<MixNodeDetails>,
}

/// Response containing details of a bonded mixnode with the provided identity key.
#[cw_serde]
pub struct MixnodeDetailsByIdentityResponse {
    /// The identity key (base58-encoded ed25519 public key) of the mixnode.
    pub identity_key: IdentityKey,

    /// If there exists a bonded mixnode with the provided identity key, this field contains its detailed information.
    pub mixnode_details: Option<MixNodeDetails>,
}

/// Response containing rewarding information of a mixnode with the provided id.
#[cw_serde]
pub struct MixnodeRewardingDetailsResponse {
    /// Id of the requested mixnode.
    pub mix_id: MixId,

    /// If there exists a mixnode with the provided id, this field contains its rewarding information.
    pub rewarding_details: Option<MixNodeRewarding>,
}

/// Response containing basic information of an unbonded mixnode with the provided id.
#[cw_serde]
pub struct UnbondedMixnodeResponse {
    /// Id of the requested mixnode.
    pub mix_id: MixId,

    /// If there existed a mixnode with the provided id, this field contains its basic information.
    pub unbonded_info: Option<UnbondedMixnode>,
}

/// Response containing the current state of the stake saturation of a mixnode with the provided id.
#[cw_serde]
pub struct StakeSaturationResponse {
    /// Id of the requested mixnode.
    pub mix_id: MixId,

    /// The current stake saturation of this node that is indirectly used in reward calculation formulas.
    /// Note that it can't be larger than 1.
    pub current_saturation: Option<Decimal>,

    /// The current, absolute, stake saturation of this node.
    /// Note that as the name suggests it can be larger than 1.
    /// However, anything beyond that value has no effect on the total node reward.
    pub uncapped_saturation: Option<Decimal>,
}
