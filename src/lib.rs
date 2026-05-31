#![cfg_attr(not(any(feature = "std", test)), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![deny(clippy::pedantic)]
#![deny(clippy::unwrap_used)]
#![forbid(missing_docs)]
#![forbid(unsafe_code)]

//!
//! This `crate` provides the ability to define constants and their corresponding macros using literals.
//!
//! ## Examples
//!
//! ```
//! use your_literals::c_style;
//!
//! c_style!(pub(crate) constants => {
//!     #define FOO: &'static str => "foo",
//!     #define BAR: &[u8; 3] => b"bar",
//!     #define BAZ: char => 'b',
//!     #define QUX: u8 => b'q',
//!     #define QUUX: i8 => -128,
//! });
//! ```

#[doc(no_inline)]
pub use paste;

/// Define your literals.
///
/// ```
/// use your_literals::c_style;
///
/// c_style!(constants => {
///     #define FOO: &str => "foo",
///     #define BAR: &str => "bar",
///     #define FOO_BAR: &str => "let foo_bar = 99;",
///     #define CORGE: f32 => 500.0,
///     #define GRAULT: f64 => 501.0,
/// });
///
/// assert_eq!(FOO, foo!());
/// assert_eq!(BAR, bar!());
/// assert_eq!(FOO_BAR, concat!("let ", concat!(foo!(), "_", bar!()), ' ', stringify!(= 99;)));
///
/// assert_eq!(CORGE, corge!());
/// assert_eq!(GRAULT, grault!());
/// ```
#[cfg_attr(any(doc, docsrs), clean_macro_docs::clean_docs)]
#[macro_export]
macro_rules! c_style {
    ($vis:vis constants => { $($rest:tt)+ }) => {
        $crate::c_style!(@inner $vis, $($rest)+);
    };
    (export $vis:vis constants => { $($rest:tt)+ }) => {
        $crate::c_style!(@inner_export $vis, $($rest)+);
    };
    (@inner $vis:vis, $(#define $name:ident: $type:ty => $value:literal),+ $(,)?) => {
        $(
            $crate::c_style!(@inner_impl $vis, $name, $type, $value);
        )+
    };
    (@inner_export $vis:vis, $(#define $name:ident: $type:ty => $value:literal),+ $(,)?) => {
        $(
            $crate::c_style!(@inner_export_impl $vis, $name, $type, $value);
        )+
    };
    (@inner_impl $vis:vis, $name:ident, $type:ty, $value:literal) => {
        $crate::c_style!(@impl_const $vis, $name, $type, $value);
        $crate::c_style!(@impl_macro $name, $value);
    };
    (@inner_export_impl $vis:vis, $name:ident, $type:ty, $value:literal) => {
        $crate::c_style!(@impl_const $vis, $name, $type, $value);
        $crate::c_style!(@impl_macro #[macro_export] $name, $value);
    };
    (@impl_const $vis:vis, $name:ident, $type:ty, $value:literal) => {
        $vis const $name: $type = $value;
    };
    (@impl_macro $(#[$attr:meta])* $name:ident, $value:literal) => {
        $crate::paste::paste! {
            $(#[$attr])*
            macro_rules! [<$name:lower>] {
                () => {
                    $value
                };
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        c_style!(constants => { #define FOO: i32 => 123, #define BAR: u64 => 456 });

        assert_eq!(foo!(), FOO);
        assert_eq!(bar!(), BAR);
    }
}
