#[macro_use]
extern crate let_or_return;
fn tst() -> bool {
    let opt_x = Some(42);
    let x = if let std::Some(x) = opt_x {
        x
    } else {
        return false;
    };
    true
}
