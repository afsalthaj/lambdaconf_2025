use crate::bindings::exports::app::myapp::calculate::Guest;

use crate::bindings::docs::calculator::calculate::eval_expression;

pub mod bindings;

fn main() {
    println!("Executing from other components");
    println!("{}", eval_expression("1 + 2"));

}

struct Component;

impl Guest for Component {
    fn eval_expression(expr: String) -> u32 {
        // This is a placeholder implementation.
        // You would replace this with your actual logic.
        println!("Evaluating expression: {}", expr);
        eval_expression(&expr)
    }
}

bindings::export!(Component with_types_in bindings);
