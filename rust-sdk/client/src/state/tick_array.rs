use crate::{FixedTickArray, MaybeTick, SparseTickArray, Tick, FIXED_TICK_ARRAY_DISCRIMINATOR, SPARSE_TICK_ARRAY_DISCRIMINATOR};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_account_info::AccountInfo;
use solana_pubkey::Pubkey;

pub const TICK_ARRAY_DISCRIMINATOR: [u8; 8] = SPARSE_TICK_ARRAY_DISCRIMINATOR;

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TickArray {
    pub discriminator: [u8; 8],
    pub start_tick_index: i32,
    #[cfg_attr(feature = "serde", serde(with = "serde_with::As::<serde_with::DisplayFromStr>"))]
    pub fusion_pool: Pubkey,
    #[cfg_attr(feature = "serde", serde(with = "serde_big_array::BigArray"))]
    pub ticks: [Tick; 88],
}

impl TickArray {
    pub const MIN_LEN: usize = 132; // 8+4+32+88
    pub const MAX_LEN: usize = 9988; // 8+4+32+88*113

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, std::io::Error> {
        if bytes.len() < 8 {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid account data length"));
        }
        let discriminator = &bytes[0..8];
        if discriminator == FIXED_TICK_ARRAY_DISCRIMINATOR {
            let tick_array = FixedTickArray::from_bytes(bytes)?;
            Ok(tick_array.into())
        } else if discriminator == SPARSE_TICK_ARRAY_DISCRIMINATOR {
            let tick_array = SparseTickArray::from_bytes(bytes)?;
            Ok(tick_array.into())
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid account discriminator"))
        }
    }
}

impl From<FixedTickArray> for TickArray {
    fn from(val: FixedTickArray) -> Self {
        TickArray {
            discriminator: TICK_ARRAY_DISCRIMINATOR,
            start_tick_index: val.start_tick_index,
            fusion_pool: val.fusion_pool,
            ticks: val.ticks,
        }
    }
}

impl From<SparseTickArray> for TickArray {
    fn from(val: SparseTickArray) -> Self {
        TickArray {
            discriminator: TICK_ARRAY_DISCRIMINATOR,
            start_tick_index: val.start_tick_index,
            fusion_pool: val.fusion_pool,
            ticks: val.ticks.map(|tick| tick.into()),
        }
    }
}

impl<'a> TryFrom<&AccountInfo<'a>> for TickArray {
    type Error = std::io::Error;

    fn try_from(account_info: &AccountInfo<'a>) -> Result<Self, Self::Error> {
        let data: &[u8] = &(*account_info.data).borrow();
        Self::from_bytes(data)
    }
}

impl From<TickArray> for SparseTickArray {
    fn from(val: TickArray) -> Self {
        SparseTickArray {
            discriminator: SPARSE_TICK_ARRAY_DISCRIMINATOR,
            start_tick_index: val.start_tick_index,
            fusion_pool: val.fusion_pool,
            ticks: val.ticks.map(|tick| tick.into()),
        }
    }
}

impl From<MaybeTick> for Tick {
    fn from(val: MaybeTick) -> Self {
        match val {
            MaybeTick::Initialized(tick) => Tick {
                initialized: true,
                liquidity_net: tick.liquidity_net,
                liquidity_gross: tick.liquidity_gross,
                fee_growth_outside_a: tick.fee_growth_outside_a,
                fee_growth_outside_b: tick.fee_growth_outside_b,
                age: tick.age,
                open_orders_input: tick.open_orders_input,
                part_filled_orders_input: tick.part_filled_orders_input,
                part_filled_orders_remaining_input: tick.part_filled_orders_remaining_input,
                fulfilled_a_to_b_orders_input: tick.fulfilled_a_to_b_orders_input,
                fulfilled_b_to_a_orders_input: tick.fulfilled_b_to_a_orders_input,
            },
            MaybeTick::Uninitialized => Tick {
                initialized: false,
                liquidity_net: 0,
                liquidity_gross: 0,
                fee_growth_outside_a: 0,
                fee_growth_outside_b: 0,
                age: 0,
                open_orders_input: 0,
                part_filled_orders_input: 0,
                part_filled_orders_remaining_input: 0,
                fulfilled_a_to_b_orders_input: 0,
                fulfilled_b_to_a_orders_input: 0,
            },
        }
    }
}

impl BorshSerialize for TickArray {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        SparseTickArray::from(self.clone()).serialize(writer)
    }
}

impl BorshDeserialize for TickArray {
    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Self::from_bytes(&buf)
    }
}

#[cfg(feature = "anchor")]
impl anchor_lang::AccountDeserialize for TickArray {
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        Ok(Self::from_bytes(buf)?)
    }
}

#[cfg(feature = "anchor")]
impl anchor_lang::AccountSerialize for TickArray {}

#[cfg(feature = "anchor")]
impl anchor_lang::Owner for TickArray {
    fn owner() -> Pubkey {
        crate::FUSIONAMM_ID
    }
}

#[cfg(feature = "anchor-idl-build")]
impl anchor_lang::IdlBuild for TickArray {}

#[cfg(feature = "anchor-idl-build")]
impl anchor_lang::Discriminator for TickArray {
    const DISCRIMINATOR: &[u8] = &[0; 8];
}

#[cfg(feature = "fetch")]
pub fn fetch_tick_array(
    rpc: &solana_client::rpc_client::RpcClient,
    address: &solana_program::pubkey::Pubkey,
) -> Result<crate::DecodedAccount<TickArray>, std::io::Error> {
    let accounts = fetch_all_tick_array(rpc, &[*address])?;
    Ok(accounts[0].clone())
}

#[cfg(feature = "fetch")]
pub fn fetch_all_tick_array(
    rpc: &solana_client::rpc_client::RpcClient,
    addresses: &[solana_program::pubkey::Pubkey],
) -> Result<Vec<crate::shared::DecodedAccount<TickArray>>, std::io::Error> {
    let accounts = rpc
        .get_multiple_accounts(addresses)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    let mut decoded_accounts: Vec<crate::shared::DecodedAccount<TickArray>> = Vec::new();
    for i in 0..addresses.len() {
        let address = addresses[i];
        let account = accounts[i]
            .as_ref()
            .ok_or(std::io::Error::new(std::io::ErrorKind::Other, format!("Account not found: {}", address)))?;
        let data = TickArray::from_bytes(&account.data)?;
        decoded_accounts.push(crate::shared::DecodedAccount {
            address,
            account: account.clone(),
            data,
        });
    }
    Ok(decoded_accounts)
}

#[cfg(feature = "fetch")]
pub fn fetch_maybe_tick_array(
    rpc: &solana_client::rpc_client::RpcClient,
    address: &solana_program::pubkey::Pubkey,
) -> Result<crate::shared::MaybeAccount<TickArray>, std::io::Error> {
    let accounts = fetch_all_maybe_tick_array(rpc, &[*address])?;
    Ok(accounts[0].clone())
}

#[cfg(feature = "fetch")]
pub fn fetch_all_maybe_tick_array(
    rpc: &solana_client::rpc_client::RpcClient,
    addresses: &[solana_program::pubkey::Pubkey],
) -> Result<Vec<crate::shared::MaybeAccount<TickArray>>, std::io::Error> {
    let accounts = rpc
        .get_multiple_accounts(addresses)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    let mut decoded_accounts: Vec<crate::shared::MaybeAccount<TickArray>> = Vec::new();
    for i in 0..addresses.len() {
        let address = addresses[i];
        if let Some(account) = accounts[i].as_ref() {
            let data = TickArray::from_bytes(&account.data)?;
            decoded_accounts.push(crate::shared::MaybeAccount::Exists(crate::shared::DecodedAccount {
                address,
                account: account.clone(),
                data,
            }));
        } else {
            decoded_accounts.push(crate::shared::MaybeAccount::NotFound(address));
        }
    }
    Ok(decoded_accounts)
}
