//
// Copyright (c) Cryptic Dot
//
// Modification based on Orca Whirlpools (https://github.com/orca-so/whirlpools),
// originally licensed under the Apache License, Version 2.0, prior to February 26, 2025.
//
// Modifications licensed under FusionAMM SDK Source-Available License v1.0
// See the LICENSE file in the project root for license information.
//

import type { Account, Address, GetProgramAccountsApi, GetProgramAccountsMemcmpFilter, Rpc } from "@solana/kit";
import { getAddressEncoder, getBase58Decoder, getI32Encoder } from "@solana/kit";

import {
  FUSIONAMM_PROGRAM_ADDRESS,
  getSparseTickArrayDecoder,
  SPARSE_TICK_ARRAY_DISCRIMINATOR,
  SparseTickArray,
} from "../generated";

import { fetchDecodedProgramAccounts } from "./utils";

export type SparseTickArrayFilter = GetProgramAccountsMemcmpFilter & {
  readonly __kind: unique symbol;
};

export function sparseTickArrayStartTickIndexFilter(startTickIndex: number): SparseTickArrayFilter {
  return {
    memcmp: {
      offset: 8n,
      bytes: getBase58Decoder().decode(getI32Encoder().encode(startTickIndex)),
      encoding: "base58",
    },
  } as SparseTickArrayFilter;
}

export function sparseTickArrayFusionPoolFilter(address: Address): SparseTickArrayFilter {
  return {
    memcmp: {
      offset: 12n,
      bytes: getBase58Decoder().decode(getAddressEncoder().encode(address)),
      encoding: "base58",
    },
  } as SparseTickArrayFilter;
}

export async function fetchAllSparseTickArrayWithFilter(
  rpc: Rpc<GetProgramAccountsApi>,
  ...filters: SparseTickArrayFilter[]
): Promise<Account<SparseTickArray>[]> {
  const discriminator = getBase58Decoder().decode(SPARSE_TICK_ARRAY_DISCRIMINATOR);
  const discriminatorFilter: GetProgramAccountsMemcmpFilter = {
    memcmp: {
      offset: 0n,
      bytes: discriminator,
      encoding: "base58",
    },
  };
  return fetchDecodedProgramAccounts(
    rpc,
    FUSIONAMM_PROGRAM_ADDRESS,
    [discriminatorFilter, ...filters],
    getSparseTickArrayDecoder(),
  );
}
