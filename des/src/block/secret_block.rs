/// Macro to generate a masked, zeroizable integer wrapper type.
///
/// Usage:
/// ```
/// secret_block! {
///     /// docs...
///     pub struct Block48(u64, 48, 0x0000_FFFF_FFFF_FFFFu64);
/// }
/// ```
#[macro_export]
macro_rules! secret_block {
    (
    $(#[$meta:meta])*
    $vis:vis struct $name:ident ( $int:tt, $bits:expr, $mask:expr );
) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        $vis struct $name($int);

        impl $name {
            /// Mask to restrict the underlying integer to valid bits.
            pub const MASK: $int = $mask;

            /// Calculate the number of hex digits needed for the bit width
            const fn hex_width() -> usize {
                ($bits as usize).div_ceil(4)
            }

            /// Calculate the number of octal digits needed for the bit width
            const fn octal_width() -> usize {
                ($bits as usize).div_ceil(3)
            }

            #[inline]
            #[must_use]
            pub const fn new(value: $int) -> Self {
                Self(value & Self::MASK)
            }

            secret_block!(@conversions_as $int);
            secret_block!(@conversions_from $int $int);
        }

        impl From<$int> for $name {
            fn from(v: $int) -> Self {
                Self(v & Self::MASK)
            }
        }

        impl From<$name> for $int {
            fn from(value: $name) -> $int {
                value.0
            }
        }

        impl std::fmt::UpperHex for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:0width$X}", self.0, width = Self::hex_width())
            }
        }

        impl std::fmt::LowerHex for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:0width$x}", self.0, width = Self::hex_width())
            }
        }

        impl std::fmt::Octal for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:0width$o}", self.0, width = Self::octal_width())
            }
        }

        impl std::fmt::Binary for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:0width$b}", self.0, width = $bits)
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

        secret_block!(@conversions_as u16);
    };
    (@conversions_as u16) => {
        /// Return value as u16
        #[allow(dead_code)]
        #[inline]
        #[must_use]
        pub const fn as_u16(&self) -> u16 {
            self.0 as u16
        }

        secret_block!(@conversions_as u32);
    };
    (@conversions_as u32) => {
        /// Return value as u32
        #[allow(dead_code)]
        #[inline]
        #[must_use]
        pub const fn as_u32(&self) -> u32 {
            self.0 as u32
        }

        secret_block!(@conversions_as u64);
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
    // Helper: generate conversions_from based on type
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

        secret_block!(@conversions_from u8 $int);
    };
    (@conversions_from u32 $int:tt) => {
        /// Create value from u32
        #[allow(dead_code)]
        #[inline]
        #[must_use]
        pub const fn from_u32(key: u32) -> Self {
            Self(key as $int & Self::MASK)
        }

        secret_block!(@conversions_from u16 $int);
    };
    (@conversions_from u64 $int:tt) => {
        /// Create value from u64
        #[allow(dead_code)]
        #[inline]
        #[must_use]
        pub const fn from_u64(key: u64) -> Self {
            Self(key & Self::MASK)
        }

        secret_block!(@conversions_from u32 $int);
    }
}
