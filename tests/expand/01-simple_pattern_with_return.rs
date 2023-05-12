#[macro_use]
extern crate let_or_return;

fn tst() -> bool {
    let opt_x = Some(42);
    let_or_return!(Some(x) = opt_x, false);
    true
}
