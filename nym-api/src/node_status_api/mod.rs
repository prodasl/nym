// Copyright 2021 - Nym Technologies SA <contact@nymtech.net>
// SPDX-License-Identifier: Apache-2.0

pub(crate) use cache::{NodeStatusCache, NodeStatusCacheRefresher};
use okapi::openapi3::OpenApi;
use rocket::Route;
use rocket_okapi::{openapi_get_routes_spec, settings::OpenApiSettings};
use std::time::Duration;
pub(crate) mod cache;
pub(crate) mod helpers;
pub(crate) mod local_guard;
pub(crate) mod models;
pub(crate) mod reward_estimate;
pub(crate) mod routes;
pub(crate) mod uptime_updater;
pub(crate) mod utils;

pub(crate) const FIFTEEN_MINUTES: Duration = Duration::from_secs(900);
pub(crate) const ONE_HOUR: Duration = Duration::from_secs(3600);
pub(crate) const ONE_DAY: Duration = Duration::from_secs(86400);

pub(crate) fn node_status_routes(
    settings: &OpenApiSettings,
    enabled: bool,
) -> (Vec<Route>, OpenApi) {
    if enabled {
        openapi_get_routes_spec![
            settings: routes::gateway_report,
            routes::gateway_uptime_history,
            routes::gateway_core_status_count,
            routes::mixnode_report,
            routes::mixnode_uptime_history,
            routes::mixnode_core_status_count,
            routes::get_mixnode_status,
            routes::get_mixnode_reward_estimation,
            routes::compute_mixnode_reward_estimation,
            routes::get_mixnode_stake_saturation,
            routes::get_mixnode_inclusion_probability,
            routes::get_mixnode_avg_uptime,
            routes::get_mixnode_inclusion_probabilities,
            routes::get_mixnodes_detailed,
            routes::get_rewarded_set_detailed,
            routes::get_active_set_detailed,
        ]
    } else {
        // in the minimal variant we would not have access to endpoints relying on existence
        // of the network monitor and the associated storage
        openapi_get_routes_spec![
            settings: routes::get_mixnode_status,
            routes::get_mixnode_stake_saturation,
            routes::get_mixnode_inclusion_probability,
            routes::get_mixnode_inclusion_probabilities,
            routes::get_mixnodes_detailed,
            routes::get_rewarded_set_detailed,
            routes::get_active_set_detailed,
        ]
    }
}