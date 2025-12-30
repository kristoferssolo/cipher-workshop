/// Macro to generate a masked integer wrapper type for cipher blocks.
///
/// Generates a type with formatting traits, conversions, and security features.
///
/// # Example
/// ```ignore
/// secret_block! {
///     pub struct Block64(u64, 64, 0xFFFF_FFFF_FFFF_FFFF);
/// }
/// ```
#[macro_export]
macro_rules! secret_block {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident ( $int:tt, $bits:expr, $mask:expr );
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, ::zeroize::Zeroize)]
        $vis struct $name($int);

        impl $name {
            /// Mask to restrict the underlying integer to valid bits.
            pub const MASK: $int = $mask;

            const fn hex_width() -> usize {
                ($bits as usize).div_ceil(4)
            }

            const fn octal_width() -> usize {
                ($bits as usize).div_ceil(3)
            }

            #[inline]
            #[must_use]
            pub const fn new(value: $int) -> Self {
                Self(value & Self::MASK)
            }

            $crate::secret_block!(@conversions_as $int);
            $crate::secret_block!(@conversions_from $int $int);
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

        impl ::std::fmt::UpperHex for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{:0width$X}", self.0, width = Self::hex_width())
            }
        }

        impl ::std::fmt::LowerHex for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{:0width$x}", self.0, width = Self::hex_width())
            }
        }

        impl ::std::fmt::Octal for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{:0width$o}", self.0, width = Self::octal_width())
            }
        }

        impl ::std::fmt::Binary for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{:0width$b}", self.0, width = $bits)
            }
        }
    };

    // Conversion chain: as_u8 -> as_u16 -> as_u32 -> as_u64 -> as_u128
    (@conversions_as u8) => {
        #[allow(dead_code)]
        #[inline]
        #[must_use]
        pub const fn as_u8(&self) -> u8 { self.0 as u8 }
        $crate::secret_block!(@conversions_as u16);
    };
    (@conversions_as u16) => {
        #[allow(dead_code)]
        #[inline]
        #[must_use]
        pub const fn as_u16(&self) -> u16 { self.0 as u16 }
        $crate::secret_block!(@conversions_as u32);
    };
    (@conversions_as u32) => {
        #[allow(dead_code)]
        #[inline]
        #[must_use]
        pub const fn as_u32(&self) -> u32 { self.0 as u32 }
        $crate::secret_block!(@conversions_as u64);
    };
    (@conversions_as u64) => {
        #[allow(dead_code)]
        #[inline]
        #[must_use]
        pub const fn as_u64(&self) -> u64 { self.0 as u64 }
        $crate::secret_block!(@conversions_as u128);
    };
    (@conversions_as u128) => {
        #[allow(dead_code)]
        #[inline]
        #[must_use]
        pub const fn as_u128(&self) -> u128 { self.0 as u128 }
    };

    // Conversion chain: from_u128 -> from_u64 -> from_u32 -> from_u16 -> from_u8
    (@conversions_from u8 $int:tt) => {
        #[allow(dead_code)]
        #[inline]
        #[must_use]
        pub const fn from_u8(v: u8) -> Self { Self(v as $int & Self::MASK) }
    };
    (@conversions_from u16 $int:tt) => {
        #[allow(dead_code)]
        #[inline]
        #[must_use]
        pub const fn from_u16(v: u16) -> Self { Self(v as $int & Self::MASK) }
        $crate::secret_block!(@conversions_from u8 $int);
    };
    (@conversions_from u32 $int:tt) => {
        #[allow(dead_code)]
        #[inline]
        #[must_use]
        pub const fn from_u32(v: u32) -> Self { Self(v as $int & Self::MASK) }
        $crate::secret_block!(@conversions_from u16 $int);
    };
    (@conversions_from u64 $int:tt) => {
        #[allow(dead_code)]
        #[inline]
        #[must_use]
        pub const fn from_u64(v: u64) -> Self { Self(v as $int & Self::MASK) }
        $crate::secret_block!(@conversions_from u32 $int);
    };
    (@conversions_from u128 $int:tt) => {
        #[allow(dead_code)]
        #[inline]
        #[must_use]
        pub const fn from_u128(v: u128) -> Self { Self(v as $int & Self::MASK) }
        $crate::secret_block!(@conversions_from u64 $int);
    };
}

/// Macro to generate a masked integer wrapper type for cipher keys.
///
/// Keys have redacted Debug output and are zeroized on drop for security.
///
/// # Example
/// ```ignore
/// secret_key! {
///     pub struct Key(u64, 64, 0xFFFF_FFFF_FFFF_FFFF);
/// }
/// ```
#[macro_export]
macro_rules! secret_key {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident ( $int:tt, $bits:expr, $mask:expr );
    ) => {
        $(#[$meta])*
        #[derive(Default, Clone, Copy, ::zeroize::Zeroize)]
        $vis struct $name($int);

        impl $name {
            /// Mask to restrict the underlying integer to valid bits.
            pub const MASK: $int = $mask;

            $crate::secret_key!(@conversions_as $int);
            $crate::secret_key!(@conversions_from $int $int);
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

    // Conversion chain: as_u8 -> as_u16 -> as_u32 -> as_u64 -> as_u128
    (@conversions_as u8) => {
        #[allow(dead_code)]
        #[inline]
        #[must_use]
        pub const fn as_u8(&self) -> u8 { self.0 as u8 }
        $crate::secret_key!(@conversions_as u16);
    };
    (@conversions_as u16) => {
        #[allow(dead_code)]
        #[inline]
        #[must_use]
        pub const fn as_u16(&self) -> u16 { self.0 as u16 }
        $crate::secret_key!(@conversions_as u32);
    };
    (@conversions_as u32) => {
        #[allow(dead_code)]
        #[inline]
        #[must_use]
        pub const fn as_u32(&self) -> u32 { self.0 as u32 }
        $crate::secret_key!(@conversions_as u64);
    };
    (@conversions_as u64) => {
        #[allow(dead_code)]
        #[inline]
        #[must_use]
        pub const fn as_u64(&self) -> u64 { self.0 as u64 }
        $crate::secret_key!(@conversions_as u128);
    };
    (@conversions_as u128) => {
        #[allow(dead_code)]
        #[inline]
        #[must_use]
        pub const fn as_u128(&self) -> u128 { self.0 as u128 }
    };

    // Conversion chain: from_u128 -> from_u64 -> from_u32 -> from_u16 -> from_u8
    (@conversions_from u8 $int:tt) => {
        #[allow(dead_code)]
        #[inline]
        #[must_use]
        pub const fn from_u8(v: u8) -> Self { Self(v as $int & Self::MASK) }
    };
    (@conversions_from u16 $int:tt) => {
        #[allow(dead_code)]
        #[inline]
        #[must_use]
        pub const fn from_u16(v: u16) -> Self { Self(v as $int & Self::MASK) }
        $crate::secret_key!(@conversions_from u8 $int);
    };
    (@conversions_from u32 $int:tt) => {
        #[allow(dead_code)]
        #[inline]
        #[must_use]
        pub const fn from_u32(v: u32) -> Self { Self(v as $int & Self::MASK) }
        $crate::secret_key!(@conversions_from u16 $int);
    };
    (@conversions_from u64 $int:tt) => {
        #[allow(dead_code)]
        #[inline]
        #[must_use]
        pub const fn from_u64(v: u64) -> Self { Self(v as $int & Self::MASK) }
        $crate::secret_key!(@conversions_from u32 $int);
    };
    (@conversions_from u128 $int:tt) => {
        #[allow(dead_code)]
        #[inline]
        #[must_use]
        pub const fn from_u128(v: u128) -> Self { Self(v as $int & Self::MASK) }
        $crate::secret_key!(@conversions_from u64 $int);
    };
}
