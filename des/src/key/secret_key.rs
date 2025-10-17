/// Macro to generate a masked, zeroizable integer wrapper type.
///
/// Usage:
/// ```
/// secret_key! {
///     /// docs...
///     pub struct Subkey(u64, 48, 0x0000_FFFF_FFFF_FFFFu64);
/// }
/// ```
#[macro_export]
macro_rules! secret_key {
    (
    $(#[$meta:meta])*
    $vis:vis struct $name:ident ( $int:tt, $bits:expr, $mask:expr );
) => {
        $(#[$meta])*
        #[derive(::zeroize::ZeroizeOnDrop, Default)]
        #[zeroize(drop)]
        $vis struct $name($int);

        impl $name {
            /// Mask to restrict the underlying integer to valid bits.
            pub const MASK: $int = $mask;

            secret_key!(@conversions_as $int);
            secret_key!(@conversions_from $int $int);
        }

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str(concat!(stringify!($name), "[REDACTED]"))
            }
        }

        impl From<$int> for $name {
            fn from(v: $int) -> Self {
                Self(v & Self::MASK)
            }
        }
    };
    // Helper: generate conversions_as based on type
    (@conversions_as u8) => {
        /// Return value as u8
        #[allow(dead_code)]
        #[inline]
        #[must_use]
        pub const fn as_u8(&self) -> u8 {
            self.0 as u8
        }

        secret_key!(@conversions_as u16);
    };
    (@conversions_as u16) => {
        /// Return value as u16
        #[allow(dead_code)]
        #[inline]
        #[must_use]
        pub const fn as_u16(&self) -> u16 {
            self.0 as u16
        }

        secret_key!(@conversions_as u32);
    };
    (@conversions_as u32) => {
        /// Return value as u32
        #[allow(dead_code)]
        #[inline]
        #[must_use]
        pub const fn as_u32(&self) -> u32 {
            self.0 as u32
        }

        secret_key!(@conversions_as u64);
    };
    (@conversions_as u64) => {
        /// Return value as u64
        #[allow(dead_code)]
        #[inline]
        #[must_use]
        pub const fn as_u64(&self) -> u64 {
            self.0 as u64
        }
    };
    (@conversions_from u8 $int:tt) => {
        /// Create value from u8
        #[allow(dead_code)]
        #[inline]
        #[must_use]
        pub const fn from_u8(key: u8) -> Self {
            Self(key as $int & Self::MASK)
        }
    };
    (@conversions_from u16 $int:tt) => {
        /// Create value from u16
        #[allow(dead_code)]
        #[inline]
        #[must_use]
        pub const fn from_u16(key: u16) -> Self {
            Self(key as $int & Self::MASK)
        }

        secret_key!(@conversions_from u8 $int);
    };
    (@conversions_from u32 $int:tt) => {
        /// Create value from u32
        #[allow(dead_code)]
        #[inline]
        #[must_use]
        pub const fn from_u32(key: u32) -> Self {
            Self(key as $int & Self::MASK)
        }

        secret_key!(@conversions_from u16 $int);
    };
    (@conversions_from u64 $int:tt) => {
        /// Create value from u64
        #[allow(dead_code)]
        #[inline]
        #[must_use]
        pub const fn from_u64(key: u64) -> Self {
            Self(key & Self::MASK)
        }

        secret_key!(@conversions_from u32 $int);
    }
}
