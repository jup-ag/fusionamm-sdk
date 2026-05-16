use crate::{MaybeTick, SparseTickArray};
use fusionamm_core::{TickArrayFacade, TickFacade};

impl From<SparseTickArray> for TickArrayFacade {
    fn from(val: SparseTickArray) -> Self {
        TickArrayFacade {
            start_tick_index: val.start_tick_index,
            ticks: val.ticks.map(|tick| tick.into()),
        }
    }
}

impl From<MaybeTick> for TickFacade {
    fn from(val: MaybeTick) -> Self {
        match val {
            MaybeTick::Uninitialized => TickFacade {
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
            MaybeTick::Initialized(tick) => TickFacade {
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
        }
    }
}
