// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::call;
use jsonrpc_v2::Error;
use rpc_api::state_api::*;

pub async fn state_get_actor(params: StateGetActorParams) -> Result<StateGetActorResult, Error> {
    call(STATE_GET_ACTOR, params).await
}

pub async fn state_miner_power(
    params: StateMinerPowerParams,
) -> Result<StateMinerPowerResult, Error> {
    call(STATE_MINER_POWER, params).await
}
