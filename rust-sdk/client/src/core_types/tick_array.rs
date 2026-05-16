use crate::{SparseTickArray, TickArray};
use fusionamm_core::TickArrayFacade;

impl From<TickArray> for TickArrayFacade {
    fn from(val: TickArray) -> Self {
        SparseTickArray::from(val).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{MaybeTick, TickData, SPARSE_TICK_ARRAY_DISCRIMINATOR};
    use solana_program::pubkey::Pubkey;

    #[test]
    fn test_sparse_tick_array_to_facade() {
        let mut ticks: [MaybeTick; 88] = std::array::from_fn(|_| MaybeTick::Uninitialized);

        ticks[1] = MaybeTick::Initialized(TickData {
            liquidity_net: 1,
            liquidity_gross: 2,
            fee_growth_outside_a: 3,
            fee_growth_outside_b: 4,
            age: 5,
            open_orders_input: 6,
            part_filled_orders_input: 7,
            part_filled_orders_remaining_input: 8,
            fulfilled_a_to_b_orders_input: 9,
            fulfilled_b_to_a_orders_input: 10,
        });

        let sparse_tick_array = SparseTickArray {
            discriminator: SPARSE_TICK_ARRAY_DISCRIMINATOR,
            start_tick_index: 176,
            fusion_pool: Pubkey::new_unique(),
            ticks,
        };

        let tick_array = TickArray::from(sparse_tick_array);

        let facade: TickArrayFacade = tick_array.into();

        assert_eq!(facade.start_tick_index, 176);
        assert!(facade.ticks[1].initialized);
        assert_eq!(facade.ticks[1].liquidity_net, 1);
        assert_eq!(facade.ticks[1].liquidity_gross, 2);
        assert_eq!(facade.ticks[1].fee_growth_outside_a, 3);
        assert_eq!(facade.ticks[1].fee_growth_outside_b, 4);
        assert_eq!(facade.ticks[1].age, 5);
        assert_eq!(facade.ticks[1].open_orders_input, 6);
        assert_eq!(facade.ticks[1].part_filled_orders_input, 7);
        assert_eq!(facade.ticks[1].part_filled_orders_remaining_input, 8);
        assert_eq!(facade.ticks[1].fulfilled_a_to_b_orders_input, 9);
        assert_eq!(facade.ticks[1].fulfilled_b_to_a_orders_input, 10);
    }
}
