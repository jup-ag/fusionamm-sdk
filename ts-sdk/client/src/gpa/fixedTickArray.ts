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
  FIXED_TICK_ARRAY_DISCRIMINATOR,
  FixedTickArray,
  FUSIONAMM_PROGRAM_ADDRESS,
  getFixedTickArrayDecoder,
} from "../generated";

import { fetchDecodedProgramAccounts } from "./utils";

export type FixedTickArrayFilter = GetProgramAccountsMemcmpFilter & {
  readonly __kind: unique symbol;
};

export function fixedTickArrayStartTickIndexFilter(startTickIndex: number): FixedTickArrayFilter {
  return {
    memcmp: {
      offset: 8n,
      bytes: getBase58Decoder().decode(getI32Encoder().encode(startTickIndex)),
      encoding: "base58",
    },
  } as FixedTickArrayFilter;
}

export function fixedTickArrayFusionPoolFilter(address: Address): FixedTickArrayFilter {
  return {
    memcmp: {
      offset: 113n * 88n + 12n,
      bytes: getBase58Decoder().decode(getAddressEncoder().encode(address)),
      encoding: "base58",
    },
  } as FixedTickArrayFilter;
}

export async function fetchAllFixedTickArrayWithFilter(
  rpc: Rpc<GetProgramAccountsApi>,
  ...filters: FixedTickArrayFilter[]
): Promise<Account<FixedTickArray>[]> {
  const discriminator = getBase58Decoder().decode(FIXED_TICK_ARRAY_DISCRIMINATOR);
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
    getFixedTickArrayDecoder(),
  );
}
