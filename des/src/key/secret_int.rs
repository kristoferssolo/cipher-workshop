/// Macro to generate a masked, zeroizable integer wrapper type.
///
/// Usage:
/// ```
/// secret_int! {
///     /// docs...
///     pub struct Subkey(u64, 48, 0x0000_FFFF_FFFF_FFFFu64);
/// }
/// ```
///
/// Optional `clone` token enables an explicit Clone impl:
/// ```
/// secret_int! { pub struct Foo(u32, 28, 0x0FFF_FFFFu32, clone); }
/// ```
#[macro_export]
macro_rules! secret_int {
    (
    $(#[$meta:meta])*
    $vis:vis struct $name:ident ( $int:ty, $bits:expr, $mask:expr $(, $opt:ident )? );
) => {
        $(#[$meta])*
        #[derive(::zeroize::ZeroizeOnDrop, Default, Eq)]
        #[zeroize(drop)]
        $vis struct $name($int);

        impl $name {
            /// Number of meaningful bits.
            pub const BITS: usize = $bits;
            /// Mask to restrict the underlying integer to valid bits.
            pub const MASK: $int = $mask;

            /// Create from the given integer.
            #[inline]
            #[must_use]
            pub const fn from_int(key: $int) -> Self {
                Self(key & Self::MASK)
            }

            /// Return as masked integer value;
            #[inline]
            #[must_use]
            pub const fn as_int(&self) -> $int {
                self.0 & Self::MASK
            }

            /// Zero value.
            pub const fn zero() -> Self {
                Self(0)
            }
        }

        // Optionally add Clone if requested explicitly (discouraged for secrets)
        secret_int!(@maybe_add_clone $( $opt )? ; $name, $int);

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str(concat!(stringify!($name), "[REDACTED]"))
            }
        }

        impl From<$int> for $name {
            fn from(v: $int) -> Self {
                Self::from_int(v)
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.as_int() == other.as_int()
            }
        }
    };

    // helper arm: create Clone impl only when "clone" token present
    (@maybe_add_clone clone ; $name:ident, $int:ty) => {
        impl Clone for $name {
            fn clone(&self) -> Self {
                // explicit clone - intentionally duplicating secret
                Self::from_int(self.as_int())
            }
        }
    };
    (@maybe_add_clone ; $name:ident, $int:ty) => {};
}
