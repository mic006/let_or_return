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
/// ```
/// use let_or_return::let_or_return;
///
/// fn process_x(opt_x: &Option<u32>) -> bool {
///     let_or_return!(Some(x) = opt_x, false);
///     // use x
///     true
/// }
/// ```
/// will expand to
/// ```
/// fn process_x(opt_x: &Option<u32>) -> bool {
///     let x = if let Some(x) = opt_x { x } else { return false };
///     // use x
///     true
/// }
/// ```
///
/// ## Complex pattern, with explicit variable parameter
/// ```
/// use let_or_return::let_or_return;
///
/// struct A {
///     a: Option<u32>,
///     b: u32,
/// }
///
/// fn process_x(in_x: &A) -> bool {
///     let_or_return!(A { a: Some(a), b } = in_x => (a, b), false);
///     // use a and b
///     true
/// }
/// ```
/// will expand to
/// ```
/// struct A {
///     a: Option<u32>,
///     b: u32,
/// }
///
/// fn process_x(in_x: &A) -> bool {
///     let (a, b) = if let A { a: Some(a), b } = in_x {
///         (a, b)
///     } else {
///         return false;
///     };
///     // use a and b
///     true
/// }
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
    ($pattern:pat = $input:expr => $var:tt $(, $ret:expr )?) => {
        let $var = if let $pattern = $input {
            $var
        } else {
            return $($ret)?;
        };
    };
}
