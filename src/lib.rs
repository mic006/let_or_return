//! Convenient macro to extract a value via `if let`, and `return` in the else case.

/// Extract a value via `if let`, and `return` in the else case.
///
/// The macro is a convenient and concise way to extract value(s)
/// using a pattern, and return if the pattern does not match.
///
/// It avoids the cumulated indentation and reduces the boiler plate
/// of `if let ... {...} else { return ... }` statements.
///
/// # Description
///
/// Signature: let_or_return!(pattern [=> var] [, ret])
/// - pattern is the pattern for variable(s) extraction
/// - var is the extracted variable(s) (optional for a simple pattern)
/// - ret is the return value when the pattern does not match (optional)
///
/// # Examples
/// ## Simple pattern
/// ```ignore
/// let opt_x = Some(42);
/// let_or_return!(Some(x) = opt_x, false);
/// ```
/// will expand to
/// ```ignore
/// let opt_x = Some(42);
/// let x = if let Some(x) = opt_x { x } else { return false };
/// ```
///
/// ## Complex pattern, with explicit variable parameter
/// ```ignore
/// struct A {
///     a: Option<u32>,
///     b: u32,
/// }
/// let in_x = A{a: Some(42), b:27};
/// let_or_return!(A{a: Some(a), b, c: _c} = in_x => (a, b), false);
/// ```
/// will expand to
/// ```ignore
/// let in_x = A { a: Some(42), b: 27, c: 12 };
/// let (a, b) = if let A { a: Some(a), b, c: _c } = in_x {
///     (a, b)
/// } else {
///     return false;
/// };
/// ```
#[macro_export]
macro_rules! let_or_return {
    // simple pattern: [enum::]EnumVariant(var) = input [, ret]
    ($enum:ident $(::$enum_path:ident)* ( $var:ident ) = $input:expr $(, $ret:expr )?) => {
        let $var = if let $enum $(::$enum_path)* ($var) = $input {
            $var
        } else {
            return $($ret)?;
        };
    };
    // complex pattern: pattern = input => var [, ret]
    ($pattern:pat = $input:expr => $var:expr $(, $ret:expr )?) => {
        let $var = if let $pattern = $input {
            $var
        } else {
            return $($ret)?;
        };
    };
}
