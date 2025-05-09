#[allow(warnings)]
mod bindings;

use bindings::exports::docs::calculator::calculate::Guest;
use crate::bindings::docs::adder::add::add;

struct Component;

impl Guest for Component {
    fn eval_expression(expr: String) -> u32 {
        add(1, 2)
    }
}

bindings::export!(Component with_types_in bindings);