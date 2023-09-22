/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.35.3.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

export type Decimal = string;
export type Percent = Decimal;

/**
 * This instantiates the contract.
 */
export interface InstantiateMsg {
  epoch_duration: Duration;
  epochs_in_interval: number;
  initial_rewarding_params: InitialRewardingParams;
  rewarding_denom: string;
  rewarding_validator_address: string;
  vesting_contract_address: string;
}
export interface Duration {
  nanos: number;
  secs: number;
  [k: string]: unknown;
}
export interface InitialRewardingParams {
  active_set_size: number;
  active_set_work_factor: Decimal;
  initial_reward_pool: Decimal;
  initial_staking_supply: Decimal;
  interval_pool_emission: Percent;
  rewarded_set_size: number;
  staking_supply_scale_factor: Percent;
  sybil_resistance: Percent;
}
export type ExecuteMsg = {
  assign_node_layer: {
    layer: Layer;
    mix_id: number;
  };
} | {
  create_family: {
    label: string;
  };
} | {
  join_family: {
    family_head: FamilyHead;
    join_permit: MessageSignature;
  };
} | {
  leave_family: {
    family_head: FamilyHead;
  };
} | {
  kick_family_member: {
    member: string;
  };
} | {
  create_family_on_behalf: {
    label: string;
    owner_address: string;
  };
} | {
  join_family_on_behalf: {
    family_head: FamilyHead;
    join_permit: MessageSignature;
    member_address: string;
  };
} | {
  leave_family_on_behalf: {
    family_head: FamilyHead;
    member_address: string;
  };
} | {
  kick_family_member_on_behalf: {
    head_address: string;
    member: string;
  };
} | {
  update_rewarding_validator_address: {
    address: string;
  };
} | {
  update_contract_state_params: {
    updated_parameters: ContractStateParams;
  };
} | {
  update_active_set_size: {
    active_set_size: number;
    force_immediately: boolean;
  };
} | {
  update_rewarding_params: {
    force_immediately: boolean;
    updated_params: IntervalRewardingParamsUpdate;
  };
} | {
  update_interval_config: {
    epoch_duration_secs: number;
    epochs_in_interval: number;
    force_immediately: boolean;
  };
} | {
  begin_epoch_transition: {};
} | {
  advance_current_epoch: {
    expected_active_set_size: number;
    new_rewarded_set: LayerAssignment[];
  };
} | {
  reconcile_epoch_events: {
    limit?: number | null;
  };
} | {
  bond_mixnode: {
    cost_params: MixNodeCostParams;
    mix_node: MixNode;
    owner_signature: MessageSignature;
  };
} | {
  bond_mixnode_on_behalf: {
    cost_params: MixNodeCostParams;
    mix_node: MixNode;
    owner: string;
    owner_signature: MessageSignature;
  };
} | {
  pledge_more: {};
} | {
  pledge_more_on_behalf: {
    owner: string;
  };
} | {
  decrease_pledge: {
    decrease_by: Coin;
  };
} | {
  decrease_pledge_on_behalf: {
    decrease_by: Coin;
    owner: string;
  };
} | {
  unbond_mixnode: {};
} | {
  unbond_mixnode_on_behalf: {
    owner: string;
  };
} | {
  update_mixnode_cost_params: {
    new_costs: MixNodeCostParams;
  };
} | {
  update_mixnode_cost_params_on_behalf: {
    new_costs: MixNodeCostParams;
    owner: string;
  };
} | {
  update_mixnode_config: {
    new_config: MixNodeConfigUpdate;
  };
} | {
  update_mixnode_config_on_behalf: {
    new_config: MixNodeConfigUpdate;
    owner: string;
  };
} | {
  bond_gateway: {
    gateway: Gateway;
    owner_signature: MessageSignature;
  };
} | {
  bond_gateway_on_behalf: {
    gateway: Gateway;
    owner: string;
    owner_signature: MessageSignature;
  };
} | {
  unbond_gateway: {};
} | {
  unbond_gateway_on_behalf: {
    owner: string;
  };
} | {
  update_gateway_config: {
    new_config: GatewayConfigUpdate;
  };
} | {
  update_gateway_config_on_behalf: {
    new_config: GatewayConfigUpdate;
    owner: string;
  };
} | {
  delegate_to_mixnode: {
    mix_id: number;
  };
} | {
  delegate_to_mixnode_on_behalf: {
    delegate: string;
    mix_id: number;
  };
} | {
  undelegate_from_mixnode: {
    mix_id: number;
  };
} | {
  undelegate_from_mixnode_on_behalf: {
    delegate: string;
    mix_id: number;
  };
} | {
  reward_mixnode: {
    mix_id: number;
    performance: Percent;
  };
} | {
  withdraw_operator_reward: {};
} | {
  withdraw_operator_reward_on_behalf: {
    owner: string;
  };
} | {
  withdraw_delegator_reward: {
    mix_id: number;
  };
} | {
  withdraw_delegator_reward_on_behalf: {
    mix_id: number;
    owner: string;
  };
};
export type Layer = "One" | "Two" | "Three";
export type FamilyHead = string;
export type MessageSignature = number[];
export type Uint128 = string;
export interface ContractStateParams {
  minimum_gateway_pledge: Coin;
  minimum_mixnode_delegation?: Coin | null;
  minimum_mixnode_pledge: Coin;
}
export interface Coin {
  amount: Uint128;
  denom: string;
  [k: string]: unknown;
}
export interface IntervalRewardingParamsUpdate {
  active_set_work_factor?: Decimal | null;
  interval_pool_emission?: Percent | null;
  reward_pool?: Decimal | null;
  rewarded_set_size?: number | null;
  staking_supply?: Decimal | null;
  staking_supply_scale_factor?: Percent | null;
  sybil_resistance_percent?: Percent | null;
}
export interface LayerAssignment {
  layer: Layer;
  mix_id: number;
}
export interface MixNodeCostParams {
  interval_operating_cost: Coin;
  profit_margin_percent: Percent;
}
export interface MixNode {
  host: string;
  http_api_port: number;
  identity_key: string;
  mix_port: number;
  sphinx_key: string;
  verloc_port: number;
  version: string;
}
export interface MixNodeConfigUpdate {
  host: string;
  http_api_port: number;
  mix_port: number;
  verloc_port: number;
  version: string;
}
export interface Gateway {
  clients_port: number;
  host: string;
  identity_key: string;
  location: string;
  mix_port: number;
  sphinx_key: string;
  version: string;
}
export interface GatewayConfigUpdate {
  clients_port: number;
  host: string;
  location: string;
  mix_port: number;
  version: string;
}
export type QueryMsg = {
  get_all_families_paged: {
    limit?: number | null;
    start_after?: string | null;
  };
} | {
  get_all_members_paged: {
    limit?: number | null;
    start_after?: string | null;
  };
} | {
  get_family_by_head: {
    head: string;
  };
} | {
  get_family_by_label: {
    label: string;
  };
} | {
  get_family_members_by_head: {
    head: string;
  };
} | {
  get_family_members_by_label: {
    label: string;
  };
} | {
  get_contract_version: {};
} | {
  get_cw2_contract_version: {};
} | {
  get_rewarding_validator_address: {};
} | {
  get_state_params: {};
} | {
  get_state: {};
} | {
  get_rewarding_params: {};
} | {
  get_epoch_status: {};
} | {
  get_current_interval_details: {};
} | {
  get_rewarded_set: {
    limit?: number | null;
    start_after?: number | null;
  };
} | {
  get_mix_node_bonds: {
    limit?: number | null;
    start_after?: number | null;
  };
} | {
  get_mix_nodes_detailed: {
    limit?: number | null;
    start_after?: number | null;
  };
} | {
  get_unbonded_mix_nodes: {
    limit?: number | null;
    start_after?: number | null;
  };
} | {
  get_unbonded_mix_nodes_by_owner: {
    limit?: number | null;
    owner: string;
    start_after?: number | null;
  };
} | {
  get_unbonded_mix_nodes_by_identity_key: {
    identity_key: string;
    limit?: number | null;
    start_after?: number | null;
  };
} | {
  get_owned_mixnode: {
    address: string;
  };
} | {
  get_mixnode_details: {
    mix_id: number;
  };
} | {
  get_mixnode_rewarding_details: {
    mix_id: number;
  };
} | {
  get_stake_saturation: {
    mix_id: number;
  };
} | {
  get_unbonded_mix_node_information: {
    mix_id: number;
  };
} | {
  get_bonded_mixnode_details_by_identity: {
    mix_identity: string;
  };
} | {
  get_layer_distribution: {};
} | {
  get_gateways: {
    limit?: number | null;
    start_after?: string | null;
  };
} | {
  get_gateway_bond: {
    identity: string;
  };
} | {
  get_owned_gateway: {
    address: string;
  };
} | {
  get_mixnode_delegations: {
    limit?: number | null;
    mix_id: number;
    start_after?: string | null;
  };
} | {
  get_delegator_delegations: {
    delegator: string;
    limit?: number | null;
    start_after?: [number, string] | null;
  };
} | {
  get_delegation_details: {
    delegator: string;
    mix_id: number;
    proxy?: string | null;
  };
} | {
  get_all_delegations: {
    limit?: number | null;
    start_after?: [number, string] | null;
  };
} | {
  get_pending_operator_reward: {
    address: string;
  };
} | {
  get_pending_mix_node_operator_reward: {
    mix_id: number;
  };
} | {
  get_pending_delegator_reward: {
    address: string;
    mix_id: number;
    proxy?: string | null;
  };
} | {
  get_estimated_current_epoch_operator_reward: {
    estimated_performance: Percent;
    mix_id: number;
  };
} | {
  get_estimated_current_epoch_delegator_reward: {
    address: string;
    estimated_performance: Percent;
    mix_id: number;
    proxy?: string | null;
  };
} | {
  get_pending_epoch_events: {
    limit?: number | null;
    start_after?: number | null;
  };
} | {
  get_pending_interval_events: {
    limit?: number | null;
    start_after?: number | null;
  };
} | {
  get_pending_epoch_event: {
    event_id: number;
  };
} | {
  get_pending_interval_event: {
    event_id: number;
  };
} | {
  get_number_of_pending_events: {};
} | {
  get_signing_nonce: {
    address: string;
  };
};
export interface MigrateMsg {
  vesting_contract_address?: string | null;
}
export type Addr = string;
export interface PagedAllDelegationsResponse {
  delegations: Delegation[];
  start_next_after?: [number, string] | null;
}
export interface Delegation {
  amount: Coin;
  cumulative_reward_ratio: Decimal;
  height: number;
  mix_id: number;
  owner: Addr;
  proxy?: Addr | null;
}
export interface PagedFamiliesResponse {
  families: Family[];
  start_next_after?: string | null;
}
export interface Family {
  head: FamilyHead;
  label: string;
  proxy?: string | null;
}
export interface PagedMembersResponse {
  members: [string, FamilyHead][];
  start_next_after?: string | null;
}
export interface MixnodeDetailsByIdentityResponse {
  identity_key: string;
  mixnode_details?: MixNodeDetails | null;
}
export interface MixNodeDetails {
  bond_information: MixNodeBond;
  pending_changes?: PendingMixNodeChanges;
  rewarding_details: MixNodeRewarding;
}
export interface MixNodeBond {
  bonding_height: number;
  is_unbonding: boolean;
  layer: Layer;
  mix_id: number;
  mix_node: MixNode;
  original_pledge: Coin;
  owner: Addr;
  proxy?: Addr | null;
}
export interface PendingMixNodeChanges {
  pledge_change?: number | null;
}
export interface MixNodeRewarding {
  cost_params: MixNodeCostParams;
  delegates: Decimal;
  last_rewarded_epoch: number;
  operator: Decimal;
  total_unit_reward: Decimal;
  unique_delegations: number;
  unit_delegation: Decimal;
}
export interface ContractVersion {
  contract: string;
  version: string;
}
export interface ContractBuildInformation {
  build_timestamp: string;
  build_version: string;
  commit_branch: string;
  commit_sha: string;
  commit_timestamp: string;
  rustc_version: string;
}
export interface CurrentIntervalResponse {
  current_blocktime: number;
  interval: Interval;
  is_current_epoch_over: boolean;
  is_current_interval_over: boolean;
}
export interface Interval {
  current_epoch_id: number;
  current_epoch_start: string;
  epoch_length: Duration;
  epochs_in_interval: number;
  id: number;
  total_elapsed_epochs: number;
  [k: string]: unknown;
}
export interface MixNodeDelegationResponse {
  delegation?: Delegation | null;
  mixnode_still_bonded: boolean;
}
export interface PagedDelegatorDelegationsResponse {
  delegations: Delegation[];
  start_next_after?: [number, string] | null;
}
export type EpochState = "in_progress" | {
  rewarding: {
    final_node_id: number;
    last_rewarded: number;
  };
} | "reconciling_events" | "advancing_epoch";
export interface EpochStatus {
  being_advanced_by: Addr;
  state: EpochState;
}
export interface EstimatedCurrentEpochRewardResponse {
  current_stake_value?: Coin | null;
  current_stake_value_detailed_amount?: Decimal | null;
  detailed_estimation_amount?: Decimal | null;
  estimation?: Coin | null;
  original_stake?: Coin | null;
}
export interface FamilyByHeadResponse {
  family?: Family | null;
  head: FamilyHead;
}
export interface FamilyByLabelResponse {
  family?: Family | null;
  label: string;
}
export interface FamilyMembersByHeadResponse {
  head: FamilyHead;
  members: string[];
}
export interface FamilyMembersByLabelResponse {
  label: string;
  members: string[];
}
export interface GatewayBondResponse {
  gateway?: GatewayBond | null;
  identity: string;
}
export interface GatewayBond {
  block_height: number;
  gateway: Gateway;
  owner: Addr;
  pledge_amount: Coin;
  proxy?: Addr | null;
}
export interface PagedGatewayResponse {
  nodes: GatewayBond[];
  per_page: number;
  start_next_after?: string | null;
}
export interface LayerDistribution {
  layer1: number;
  layer2: number;
  layer3: number;
}
export interface PagedMixnodeBondsResponse {
  nodes: MixNodeBond[];
  per_page: number;
  start_next_after?: number | null;
}
export interface PagedMixnodesDetailsResponse {
  nodes: MixNodeDetails[];
  per_page: number;
  start_next_after?: number | null;
}
export interface PagedMixNodeDelegationsResponse {
  delegations: Delegation[];
  start_next_after?: string | null;
}
export interface MixnodeDetailsResponse {
  mix_id: number;
  mixnode_details?: MixNodeDetails | null;
}
export interface MixnodeRewardingDetailsResponse {
  mix_id: number;
  rewarding_details?: MixNodeRewarding | null;
}
export interface NumberOfPendingEventsResponse {
  epoch_events: number;
  interval_events: number;
}
export interface GatewayOwnershipResponse {
  address: Addr;
  gateway?: GatewayBond | null;
}
export interface MixOwnershipResponse {
  address: Addr;
  mixnode_details?: MixNodeDetails | null;
}
export interface PendingRewardResponse {
  amount_earned?: Coin | null;
  amount_earned_detailed?: Decimal | null;
  amount_staked?: Coin | null;
  mixnode_still_fully_bonded: boolean;
}
export type PendingEpochEventKind = {
  delegate: {
    amount: Coin;
    mix_id: number;
    owner: Addr;
    proxy?: Addr | null;
  };
} | {
  undelegate: {
    mix_id: number;
    owner: Addr;
    proxy?: Addr | null;
  };
} | {
  pledge_more: {
    amount: Coin;
    mix_id: number;
  };
} | {
  decrease_pledge: {
    decrease_by: Coin;
    mix_id: number;
  };
} | {
  unbond_mixnode: {
    mix_id: number;
  };
} | {
  update_active_set_size: {
    new_size: number;
  };
};
export interface PendingEpochEventResponse {
  event?: PendingEpochEventData | null;
  event_id: number;
}
export interface PendingEpochEventData {
  created_at: number;
  kind: PendingEpochEventKind;
}
export interface PendingEpochEventsResponse {
  events: PendingEpochEvent[];
  seconds_until_executable: number;
  start_next_after?: number | null;
}
export interface PendingEpochEvent {
  event: PendingEpochEventData;
  id: number;
}
export type PendingIntervalEventKind = {
  change_mix_cost_params: {
    mix_id: number;
    new_costs: MixNodeCostParams;
  };
} | {
  update_rewarding_params: {
    update: IntervalRewardingParamsUpdate;
  };
} | {
  update_interval_config: {
    epoch_duration_secs: number;
    epochs_in_interval: number;
  };
};
export interface PendingIntervalEventResponse {
  event?: PendingIntervalEventData | null;
  event_id: number;
}
export interface PendingIntervalEventData {
  created_at: number;
  kind: PendingIntervalEventKind;
}
export interface PendingIntervalEventsResponse {
  events: PendingIntervalEvent[];
  seconds_until_executable: number;
  start_next_after?: number | null;
}
export interface PendingIntervalEvent {
  event: PendingIntervalEventData;
  id: number;
}
export type RewardedSetNodeStatus = "active" | "standby";
export interface PagedRewardedSetResponse {
  nodes: [number, RewardedSetNodeStatus][];
  start_next_after?: number | null;
}
export interface RewardingParams {
  active_set_size: number;
  interval: IntervalRewardParams;
  rewarded_set_size: number;
}
export interface IntervalRewardParams {
  active_set_work_factor: Decimal;
  epoch_reward_budget: Decimal;
  interval_pool_emission: Percent;
  reward_pool: Decimal;
  stake_saturation_point: Decimal;
  staking_supply: Decimal;
  staking_supply_scale_factor: Percent;
  sybil_resistance: Percent;
}
export type String = string;
export type Uint32 = number;
export interface StakeSaturationResponse {
  current_saturation?: Decimal | null;
  mix_id: number;
  uncapped_saturation?: Decimal | null;
}
export interface ContractState {
  owner: Addr;
  params: ContractStateParams;
  rewarding_denom: string;
  rewarding_validator_address: Addr;
  vesting_contract_address: Addr;
}
export interface UnbondedMixnodeResponse {
  mix_id: number;
  unbonded_info?: UnbondedMixnode | null;
}
export interface UnbondedMixnode {
  identity_key: string;
  owner: Addr;
  proxy?: Addr | null;
  unbonding_height: number;
}
export interface PagedUnbondedMixnodesResponse {
  nodes: [number, UnbondedMixnode][];
  per_page: number;
  start_next_after?: number | null;
}