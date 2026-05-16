import {
  Account,
  Address,
  assertAccountExists,
  assertAccountsExist,
  EncodedAccount,
  Encoder,
  FetchAccountConfig,
  FetchAccountsConfig,
  fetchEncodedAccount,
  fetchEncodedAccounts,
  fixEncoderSize,
  getAddressEncoder,
  getArrayEncoder,
  getBytesEncoder,
  getI32Encoder,
  getStructEncoder,
  getU64Decoder,
  MaybeAccount,
  MaybeEncodedAccount,
  ReadonlyUint8Array,
  transformEncoder,
} from "@solana/kit";

import {
  decodeFixedTickArray,
  decodeSparseTickArray,
  FIXED_TICK_ARRAY_DISCRIMINATOR,
  FixedTickArray,
  getMaybeTickEncoder,
  MaybeTick,
  MaybeTickArgs,
  SPARSE_TICK_ARRAY_DISCRIMINATOR,
  SparseTickArray,
  Tick,
  type TickArgs,
} from "../generated";

export type TickArray = {
  discriminator: ReadonlyUint8Array;
  startTickIndex: number;
  fusionPool: Address;
  ticks: Array<Tick>;
};

export type TickArrayArgs = {
  startTickIndex: number;
  fusionPool: Address;
  ticks: Array<TickArgs>;
};

const FIXED_TICK_ARRAY_DISCRIMINATOR_NUMBER = getU64Decoder().decode(FIXED_TICK_ARRAY_DISCRIMINATOR);
const SPARSE_TICK_ARRAY_DISCRIMINATOR_NUMBER = getU64Decoder().decode(SPARSE_TICK_ARRAY_DISCRIMINATOR);

export function getTickArrayMinSize(): number {
  return 132; // 8+4+32+88
}

export function getTickArrayMaxSize(): number {
  return 9988; // 8+4+32+88*113
}

export function consolidateTick(tick: Tick | MaybeTick): Tick {
  if ("initialized" in tick) {
    return tick;
  }
  switch (tick.__kind) {
    case "Uninitialized":
      return {
        initialized: false,
        liquidityGross: 0n,
        liquidityNet: 0n,
        feeGrowthOutsideA: 0n,
        feeGrowthOutsideB: 0n,
        age: 0n,
        fulfilledAToBOrdersInput: 0n,
        fulfilledBToAOrdersInput: 0n,
        openOrdersInput: 0n,
        partFilledOrdersInput: 0n,
        partFilledOrdersRemainingInput: 0n,
      };
    case "Initialized":
      return {
        initialized: true,
        liquidityGross: tick.fields[0].liquidityGross,
        liquidityNet: tick.fields[0].liquidityNet,
        feeGrowthOutsideA: tick.fields[0].feeGrowthOutsideA,
        feeGrowthOutsideB: tick.fields[0].feeGrowthOutsideB,
        age: tick.fields[0].age,
        fulfilledAToBOrdersInput: tick.fields[0].fulfilledAToBOrdersInput,
        fulfilledBToAOrdersInput: tick.fields[0].fulfilledBToAOrdersInput,
        openOrdersInput: tick.fields[0].openOrdersInput,
        partFilledOrdersInput: tick.fields[0].partFilledOrdersInput,
        partFilledOrdersRemainingInput: tick.fields[0].partFilledOrdersRemainingInput,
      };
  }
}

export function getTickArrayEncoder(): Encoder<TickArrayArgs> {
  return transformEncoder(
    getStructEncoder([
      ["discriminator", fixEncoderSize(getBytesEncoder(), 8)],
      ["startTickIndex", getI32Encoder()],
      ["fusionPool", getAddressEncoder()],
      ["ticks", getArrayEncoder(getMaybeTickEncoder(), { size: 88 })],
    ]),
    value => ({
      discriminator: SPARSE_TICK_ARRAY_DISCRIMINATOR,
      startTickIndex: value.startTickIndex,
      fusionPool: value.fusionPool,
      ticks: value.ticks.map((t): MaybeTickArgs => {
        if (t.initialized) {
          return {
            __kind: "Initialized",
            fields: [
              {
                liquidityNet: t.liquidityNet,
                liquidityGross: t.liquidityGross,
                feeGrowthOutsideA: t.feeGrowthOutsideA,
                feeGrowthOutsideB: t.feeGrowthOutsideB,
                age: t.age,
                openOrdersInput: t.openOrdersInput,
                partFilledOrdersInput: t.partFilledOrdersInput,
                partFilledOrdersRemainingInput: t.partFilledOrdersRemainingInput,
                fulfilledAToBOrdersInput: t.fulfilledAToBOrdersInput,
                fulfilledBToAOrdersInput: t.fulfilledBToAOrdersInput,
              },
            ],
          };
        } else {
          return {
            __kind: "Uninitialized",
          };
        }
      }),
    }),
  );
}

export function consolidateTickArray<TAddress extends string = string>(
  tickArrayAccount: Account<FixedTickArray | SparseTickArray, TAddress>,
): Account<TickArray, TAddress>;
export function consolidateTickArray<TAddress extends string = string>(
  tickArrayAccount: MaybeAccount<FixedTickArray | SparseTickArray, TAddress>,
): MaybeAccount<TickArray, TAddress>;
export function consolidateTickArray<TAddress extends string = string>(
  tickArrayAccount:
    | Account<FixedTickArray | SparseTickArray, TAddress>
    | MaybeAccount<FixedTickArray | SparseTickArray, TAddress>,
): Account<TickArray, TAddress> | MaybeAccount<TickArray, TAddress> {
  if ("exists" in tickArrayAccount && !tickArrayAccount.exists) {
    return tickArrayAccount;
  }

  return {
    ...tickArrayAccount,
    data: {
      ...tickArrayAccount.data,
      ticks: tickArrayAccount.data.ticks.map(consolidateTick),
    },
  };
}

export function decodeTickArray<TAddress extends string = string>(
  encodedAccount: EncodedAccount<TAddress>,
): Account<TickArray, TAddress>;
export function decodeTickArray<TAddress extends string = string>(
  encodedAccount: MaybeEncodedAccount<TAddress>,
): MaybeAccount<TickArray, TAddress>;
export function decodeTickArray<TAddress extends string = string>(
  encodedAccount: EncodedAccount<TAddress> | MaybeEncodedAccount<TAddress>,
): Account<TickArray, TAddress> | MaybeAccount<TickArray, TAddress> {
  if ("exists" in encodedAccount && !encodedAccount.exists) {
    return encodedAccount;
  }
  const discriminator = getU64Decoder().decode(encodedAccount.data.subarray(0, 8));
  switch (discriminator) {
    case FIXED_TICK_ARRAY_DISCRIMINATOR_NUMBER:
      return consolidateTickArray(decodeFixedTickArray(encodedAccount));
    case SPARSE_TICK_ARRAY_DISCRIMINATOR_NUMBER:
      return consolidateTickArray(decodeSparseTickArray(encodedAccount));
    default:
      throw new Error(`Unknown discriminator: ${discriminator}`);
  }
}

export async function fetchTickArray<TAddress extends string = string>(
  rpc: Parameters<typeof fetchEncodedAccount>[0],
  address: Address<TAddress>,
  config?: FetchAccountConfig,
): Promise<Account<TickArray, TAddress>> {
  const maybeAccount = await fetchMaybeTickArray(rpc, address, config);
  assertAccountExists(maybeAccount);
  return maybeAccount;
}

export async function fetchMaybeTickArray<TAddress extends string = string>(
  rpc: Parameters<typeof fetchEncodedAccount>[0],
  address: Address<TAddress>,
  config?: FetchAccountConfig,
): Promise<MaybeAccount<TickArray, TAddress>> {
  const maybeAccount = await fetchEncodedAccount(rpc, address, config);
  return decodeTickArray(maybeAccount);
}

export async function fetchAllTickArray(
  rpc: Parameters<typeof fetchEncodedAccounts>[0],
  addresses: Array<Address>,
  config?: FetchAccountsConfig,
): Promise<Account<TickArray>[]> {
  const maybeAccounts = await fetchAllMaybeTickArray(rpc, addresses, config);
  assertAccountsExist(maybeAccounts);
  return maybeAccounts;
}

export async function fetchAllMaybeTickArray(
  rpc: Parameters<typeof fetchEncodedAccounts>[0],
  addresses: Array<Address>,
  config?: FetchAccountsConfig,
): Promise<MaybeAccount<TickArray>[]> {
  const maybeAccounts = await fetchEncodedAccounts(rpc, addresses, config);
  return maybeAccounts.map(maybeAccount => decodeTickArray(maybeAccount));
}
