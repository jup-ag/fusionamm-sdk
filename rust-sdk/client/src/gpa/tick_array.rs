//
// Copyright (c) Cryptic Dot
//
// Modification based on Orca Whirlpools (https://github.com/orca-so/whirlpools),
// originally licensed under the Apache License, Version 2.0, prior to February 26, 2025.
//
// Modifications licensed under FusionAMM SDK Source-Available License v1.0
// See the LICENSE file in the project root for license information.
//

use std::error::Error;

use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_pubkey::Pubkey;

use crate::{DecodedAccount, TickArray};

use super::{fetch_all_fixed_tick_array_with_filter, FixedTickArrayFilter};
use super::{fetch_all_sparse_tick_array_with_filter, SparseTickArrayFilter};

#[derive(Debug, Clone)]
pub enum TickArrayFilter {
    FusionPool(Pubkey),
    StartTickIndex(i32),
}

impl From<TickArrayFilter> for FixedTickArrayFilter {
    fn from(val: TickArrayFilter) -> Self {
        match val {
            TickArrayFilter::FusionPool(address) => FixedTickArrayFilter::FusionPool(address),
            TickArrayFilter::StartTickIndex(tick_index) => FixedTickArrayFilter::StartTickIndex(tick_index),
        }
    }
}

impl From<TickArrayFilter> for SparseTickArrayFilter {
    fn from(val: TickArrayFilter) -> Self {
        match val {
            TickArrayFilter::FusionPool(address) => SparseTickArrayFilter::FusionPool(address),
            TickArrayFilter::StartTickIndex(tick_index) => SparseTickArrayFilter::StartTickIndex(tick_index),
        }
    }
}

pub async fn fetch_all_tick_array_with_filter(
    rpc: &RpcClient,
    filters: Vec<TickArrayFilter>,
) -> Result<Vec<DecodedAccount<TickArray>>, Box<dyn Error>> {
    let fixed_filters = filters.clone().into_iter().map(|filter| filter.into()).collect();
    let fixed_tick_arrays = fetch_all_fixed_tick_array_with_filter(rpc, fixed_filters).await?;

    let sparse_filters = filters.clone().into_iter().map(|filter| filter.into()).collect();
    let sparse_tick_arrays = fetch_all_sparse_tick_array_with_filter(rpc, sparse_filters).await?;

    let mut tick_arrays: Vec<DecodedAccount<TickArray>> = Vec::new();

    for fixed_tick_array in fixed_tick_arrays {
        tick_arrays.push(DecodedAccount {
            address: fixed_tick_array.address,
            account: fixed_tick_array.account,
            data: TickArray::from(fixed_tick_array.data),
        });
    }

    for sparse_tick_array in sparse_tick_arrays {
        tick_arrays.push(DecodedAccount {
            address: sparse_tick_array.address,
            account: sparse_tick_array.account,
            data: TickArray::from(sparse_tick_array.data),
        });
    }

    Ok(tick_arrays)
}
