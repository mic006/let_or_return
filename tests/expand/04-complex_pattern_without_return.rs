#[macro_use]
extern crate let_or_return;

struct A {
    a: Option<u32>,
    b: u32,
    c: u32,
}

fn tst() {
    let in_x = A {
        a: Some(42),
        b: 27,
        c: 12,
    };
    let_or_return!(A{a: Some(a), b, c: _c} = in_x => (a, b));
}
