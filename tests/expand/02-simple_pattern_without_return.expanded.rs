#[macro_use]
extern crate let_or_return;
fn tst() {
    let opt_x = Some(42);
    let x = if let Some(x) = opt_x {
        x
    } else {
        return;
    };
}
