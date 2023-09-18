//! Test usage of let_or_return macro

#[macro_use]
extern crate let_or_return;

/// Test let_or_return!(pattern, ret)
#[test]
pub fn simple_pattern_with_return() {
    fn tst(opt_x: Option<u32>, processed: &mut bool) -> u32 {
        let_or_return!(Some(x) = opt_x, 0);
        *processed = true;
        x
    }
    let mut processed = false;
    assert_eq!(tst(None, &mut processed), 0);
    assert!(!processed);
    assert_eq!(tst(Some(42), &mut processed), 42);
    assert!(processed);
}

/// Test let_or_return!(pattern)
#[test]
pub fn simple_pattern_without_return() {
    fn tst(opt_x: Option<u32>, processed: &mut bool) {
        let_or_return!(Some(_x) = opt_x);
        *processed = true;
    }
    let mut processed = false;
    tst(None, &mut processed);
    assert!(!processed);
    tst(Some(42), &mut processed);
    assert!(processed);
}

struct A {
    a: Option<u32>,
    b: u32,
    c: u32,
}

/// Test let_or_return!(pattern => var, ret)
#[test]
pub fn complex_pattern_with_return() {
    fn tst(in_x: &A, processed: &mut bool) -> u32 {
        let_or_return!(A{a: Some(a), b, c: _c} = in_x => (a, b), 0);
        *processed = true;
        a + b
    }
    let mut processed = false;
    assert_eq!(
        tst(
            &A {
                a: None,
                b: 1,
                c: 1
            },
            &mut processed
        ),
        0
    );
    assert!(!processed);
    assert_eq!(
        tst(
            &A {
                a: Some(42),
                b: 1,
                c: 27
            },
            &mut processed
        ),
        42 + 1
    );
    assert!(processed);
}

/// Test let_or_return!(pattern => var)
#[test]
pub fn complex_pattern_without_return() {
    fn tst(in_x: &A, processed: &mut bool) {
        let_or_return!(A{a: Some(a), b, c: _c} = in_x => (a, b));
        *processed = true;
        let _ = a + b;
    }
    let mut processed = false;
    tst(
        &A {
            a: None,
            b: 1,
            c: 1,
        },
        &mut processed,
    );
    assert!(!processed);
    tst(
        &A {
            a: Some(42),
            b: 1,
            c: 27,
        },
        &mut processed,
    );
    assert!(processed);
}
