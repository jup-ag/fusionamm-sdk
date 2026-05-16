use fusionamm_core::{TickArrayFacade, TickFacade};

use crate::{FixedTickArray, Tick};

impl From<FixedTickArray> for TickArrayFacade {
    fn from(val: FixedTickArray) -> Self {
        TickArrayFacade {
            start_tick_index: val.start_tick_index,
            ticks: val.ticks.map(|tick| tick.into()),
        }
    }
}

impl From<Tick> for TickFacade {
    fn from(tick: Tick) -> Self {
        TickFacade {
            liquidity_net: tick.liquidity_net,
            liquidity_gross: tick.liquidity_gross,
            initialized: tick.initialized,
            fee_growth_outside_a: tick.fee_growth_outside_a,
            fee_growth_outside_b: tick.fee_growth_outside_b,
            age: tick.age,
            open_orders_input: tick.open_orders_input,
            part_filled_orders_input: tick.part_filled_orders_input,
            part_filled_orders_remaining_input: tick.part_filled_orders_remaining_input,
            fulfilled_a_to_b_orders_input: tick.fulfilled_a_to_b_orders_input,
            fulfilled_b_to_a_orders_input: tick.fulfilled_b_to_a_orders_input,
        }
    }
}
