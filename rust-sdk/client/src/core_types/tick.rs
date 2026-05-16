use crate::{MaybeTick, Tick, TickData};

impl From<Tick> for MaybeTick {
    fn from(tick: Tick) -> Self {
        if tick.initialized {
            MaybeTick::Initialized(TickData {
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
            })
        } else {
            MaybeTick::Uninitialized
        }
    }
}
