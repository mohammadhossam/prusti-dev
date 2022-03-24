use prusti_contracts::*;

trait A {}
trait B {}

#[trusted]
#[ghost_constraint(T: A, [ //~ ERROR: [Prusti: unsupported feature] Multiple ghost constraints with different bounds defined
    requires(true),
    ensures(true),
])]
#[ghost_constraint(T: B, [
    requires(true),
    ensures(true),
])]
fn foo<T>(_x: T) {
}

fn main() {
}
