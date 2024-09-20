/// Convenience macro to wrap the `csrrs` assembly instruction for reading a CSR register.
///
/// This macro should generally not be called directly.
///
/// Instead, use the [read_csr_as](crate::read_csr_as) or [read_csr_as_usize](crate::read_csr_as_usize) macros.
#[macro_export]
macro_rules! read_csr {
    ($csr_number:literal) => {
        /// Reads the CSR.
        ///
        /// **WARNING**: panics on non-`riscv` targets.
        #[inline]
        unsafe fn _read() -> usize {
            _try_read().unwrap()
        }

        /// Attempts to read the CSR.
        #[inline]
        unsafe fn _try_read() -> $crate::result::Result<usize> {
            match () {
                #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
                () => {
                    let r: usize;
                    core::arch::asm!(concat!("csrrs {0}, ", stringify!($csr_number), ", x0"), out(reg) r);
                    Ok(r)
                }

                #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
                () => Err($crate::result::Error::Unimplemented),
            }
        }
    };
}

/// `RV32`: Convenience macro to wrap the `csrrs` assembly instruction for reading a CSR register.
///
/// This macro should generally not be called directly.
///
/// Instead, use the [read_csr_as_rv32](crate::read_csr_as_rv32) or [read_csr_as_usize_rv32](crate::read_csr_as_usize_rv32) macros.
#[macro_export]
macro_rules! read_csr_rv32 {
    ($csr_number:literal) => {
        /// Reads the CSR.
        ///
        /// **WARNING**: panics on non-`riscv` targets.
        #[inline]
        unsafe fn _read() -> usize {
            _try_read().unwrap()
        }

        /// Attempts to read the CSR.
        #[inline]
        unsafe fn _try_read() -> $crate::result::Result<usize> {
            match () {
                #[cfg(target_arch = "riscv32")]
                () => {
                    let r: usize;
                    core::arch::asm!(concat!("csrrs {0}, ", stringify!($csr_number), ", x0"), out(reg) r);
                    Ok(r)
                }

                #[cfg(not(target_arch = "riscv32"))]
                () => Err($crate::result::Error::Unimplemented),
            }
        }
    };
}

/// Convenience macro to read a CSR register value as a `register` type.
///
/// The `register` type must be a defined type in scope of the macro call.
#[macro_export]
macro_rules! read_csr_as {
    ($register:ident, $csr_number:literal) => {
        $crate::read_csr!($csr_number);

        /// Reads the CSR.
        ///
        /// **WARNING**: panics on non-`riscv` targets.
        #[inline]
        pub fn read() -> $register {
            $register {
                bits: unsafe { _read() },
            }
        }

        /// Attempts to reads the CSR.
        #[inline]
        pub fn try_read() -> $crate::result::Result<$register> {
            Ok($register {
                bits: unsafe { _try_read()? },
            })
        }
    };
}

/// `RV32`: Convenience macro to read a CSR register value as a `register` type.
///
/// The `register` type must be a defined type in scope of the macro call.
#[macro_export]
macro_rules! read_csr_as_rv32 {
    ($register:ident, $csr_number:literal) => {
        $crate::read_csr_rv32!($csr_number);

        /// Reads the CSR.
        ///
        /// **WARNING**: panics on non-`riscv` targets.
        #[inline]
        pub fn read() -> $register {
            $register {
                bits: unsafe { _read() },
            }
        }

        /// Attempts to reads the CSR.
        #[inline]
        pub fn try_read() -> $crate::result::Result<$register> {
            Ok($register {
                bits: unsafe { _try_read()? },
            })
        }
    };
}

/// Convenience macro to read a CSR register value as a [`usize`].
#[macro_export]
macro_rules! read_csr_as_usize {
    ($csr_number:literal) => {
        $crate::read_csr!($csr_number);

        /// Reads the CSR.
        ///
        /// **WARNING**: panics on non-`riscv` targets.
        #[inline]
        pub fn read() -> usize {
            unsafe { _read() }
        }

        /// Attempts to read the CSR.
        #[inline]
        pub fn try_read() -> $crate::result::Result<usize> {
            unsafe { _try_read() }
        }
    };
}

/// `RV32`: Convenience macro to read a CSR register value as a [`usize`].
#[macro_export]
macro_rules! read_csr_as_usize_rv32 {
    ($csr_number:literal) => {
        $crate::read_csr_rv32!($csr_number);

        /// Reads the CSR.
        ///
        /// **WARNING**: panics on non-`riscv` targets.
        #[inline]
        pub fn read() -> usize {
            unsafe { _read() }
        }

        /// Attempts to reads the CSR.
        #[inline]
        pub fn try_read() -> $crate::result::Result<usize> {
            unsafe { _try_read() }
        }
    };
}

/// Convenience macro to wrap the `csrrw` assembly instruction for writing to CSR registers.
///
/// This macro should generally not be called directly.
///
/// Instead, use the [write_csr_as_usize](crate::write_csr_as_usize) macro.
#[macro_export]
macro_rules! write_csr {
    ($csr_number:literal) => {
        /// Writes the CSR.
        ///
        /// **WARNING**: panics on non-`riscv` targets.
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _write(bits: usize) {
            _try_write(bits).unwrap();
        }

        /// Attempts to write the CSR.
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _try_write(bits: usize) -> $crate::result::Result<()> {
            match () {
                #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
                () => {
                    core::arch::asm!(concat!("csrrw x0, ", stringify!($csr_number), ", {0}"), in(reg) bits);
                    Ok(())
                }

                #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
                () => Err($crate::result::Error::Unimplemented),
            }
        }
    };
}

/// `RV32`: Convenience macro to wrap the `csrrw` assembly instruction for writing to CSR registers.
///
/// This macro should generally not be called directly.
///
/// Instead, use the [write_csr_as_usize_rv32](crate::write_csr_as_usize_rv32) macro.
#[macro_export]
macro_rules! write_csr_rv32 {
    ($csr_number:literal) => {
        /// Writes the CSR.
        ///
        /// **WARNING**: panics on non-`riscv` targets.
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _write(bits: usize) {
            _try_write(bits).unwrap();
        }

        /// Attempts to write the CSR.
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _try_write(bits: usize) -> $crate::result::Result<()> {
            match () {
                #[cfg(target_arch = "riscv32")]
                () => {
                    core::arch::asm!(concat!("csrrw x0, ", stringify!($csr_number), ", {0}"), in(reg) bits);
                    Ok(())
                }

                #[cfg(not(target_arch = "riscv32"))]
                () => Err($crate::result::Error::Unimplemented),
            }
        }
    };
}

/// Convenience macro to write a value with `bits` to a CSR
#[macro_export]
macro_rules! write_csr_as {
    ($csr_type:ty, $csr_number:literal) => {
        $crate::write_csr!($csr_number);

        /// Writes the CSR.
        ///
        /// **WARNING**: panics on non-`riscv` targets.
        #[inline]
        pub fn write(value: $csr_type) {
            unsafe { _write(value.bits) }
        }

        /// Attempts to write the CSR.
        #[inline]
        pub fn try_write(value: $csr_type) -> $crate::result::Result<()> {
            unsafe { _try_write(value.bits) }
        }
    };
}

/// Convenience macro to write a value to a CSR register.
#[macro_export]
macro_rules! write_csr_as_rv32 {
    ($csr_type:ty, $csr_number:literal) => {
        $crate::write_csr_rv32!($csr_number);

        /// Writes the CSR.
        ///
        /// **WARNING**: panics on non-`riscv` targets.
        #[inline]
        pub fn write(value: $csr_type) {
            unsafe { _write(value.bits) }
        }

        /// Attempts to write the CSR.
        #[inline]
        pub fn try_write(value: $csr_type) -> $crate::result::Result<()> {
            unsafe { _try_write(value.bits) }
        }
    };
}

/// Convenience macro to write a [`usize`] value to a CSR register.
#[macro_export]
macro_rules! write_csr_as_usize {
    ($csr_number:literal) => {
        $crate::write_csr!($csr_number);

        /// Writes the CSR.
        ///
        /// **WARNING**: panics on non-`riscv` targets.
        #[inline]
        pub fn write(bits: usize) {
            unsafe { _write(bits) }
        }

        /// Attempts to write the CSR.
        #[inline]
        pub fn try_write(bits: usize) -> $crate::result::Result<()> {
            unsafe { _try_write(bits) }
        }
    };
}

/// `RV32`: Convenience macro to write a [`usize`] value to a CSR register.
#[macro_export]
macro_rules! write_csr_as_usize_rv32 {
    ($csr_number:literal) => {
        $crate::write_csr_rv32!($csr_number);

        /// Writes the CSR.
        ///
        /// **WARNING**: panics on non-`riscv` targets.
        #[inline]
        pub fn write(bits: usize) {
            unsafe { _write(bits) }
        }

        /// Attempts to write the CSR.
        #[inline]
        pub fn try_write(bits: usize) -> $crate::result::Result<()> {
            unsafe { _try_write(bits) }
        }
    };
}

/// Convenience macro around the `csrrs` assembly instruction to set the CSR register.
///
/// This macro is intended for use with the [set_csr](crate::set_csr) or [set_clear_csr](crate::set_clear_csr) macros.
#[macro_export]
macro_rules! set {
    ($csr_number:literal) => {
        /// Set the CSR.
        ///
        /// **WARNING**: panics on non-`riscv` targets.
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _set(bits: usize) {
            _try_set(bits).unwrap();
        }

        /// Attempts to set the CSR.
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _try_set(bits: usize) -> $crate::result::Result<()> {
            match () {
                #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
                () => {
                    core::arch::asm!(concat!("csrrs x0, ", stringify!($csr_number), ", {0}"), in(reg) bits);
                    Ok(())
                }

                #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
                () => Err($crate::result::Error::Unimplemented),
            }
        }
    };
}

/// `RV32`: Convenience macro around the `csrrs` assembly instruction to set the CSR register.
///
/// This macro is intended for use with the [set_csr](crate::set_csr) or [set_clear_csr](crate::set_clear_csr) macros.
#[macro_export]
macro_rules! set_rv32 {
    ($csr_number:literal) => {
        /// Set the CSR.
        ///
        /// **WARNING**: panics on non-`riscv` targets.
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _set(bits: usize) {
            _try_set(bits).unwrap();
        }

        /// Attempts to set the CSR.
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _try_set(bits: usize) -> $crate::result::Result<()> {
            match () {
                #[cfg(target_arch = "riscv32")]
                () => {
                    core::arch::asm!(concat!("csrrs x0, ", stringify!($csr_number), ", {0}"), in(reg) bits);
                    Ok(())
                }

                #[cfg(not(target_arch = "riscv32"))]
                () => Err($crate::result::Error::Unimplemented),
            }
        }
    };
}

/// Convenience macro around the `csrrc` assembly instruction to clear the CSR register.
///
/// This macro is intended for use with the [clear_csr](crate::clear_csr) or [set_clear_csr](crate::set_clear_csr) macros.
#[macro_export]
macro_rules! clear {
    ($csr_number:literal) => {
        /// Clear the CSR.
        ///
        /// **WARNING**: panics on non-`riscv` targets.
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _clear(bits: usize) {
            _try_clear(bits).unwrap();
        }

        /// Attempts to clear the CSR.
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _try_clear(bits: usize) -> $crate::result::Result<()> {
            match () {
                #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
                () => {
                    core::arch::asm!(concat!("csrrc x0, ", stringify!($csr_number), ", {0}"), in(reg) bits);
                    Ok(())
                }

                #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
                () => Err($crate::result::Error::Unimplemented),
            }
        }
    };
}

/// `RV32`: Convenience macro around the `csrrc` assembly instruction to clear the CSR register.
///
/// This macro is intended for use with the [clear_csr](crate::clear_csr) or [set_clear_csr](crate::set_clear_csr) macros.
#[macro_export]
macro_rules! clear_rv32 {
    ($csr_number:literal) => {
        /// Clear the CSR.
        ///
        /// **WARNING**: panics on non-`riscv` targets.
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _clear(bits: usize) {
            _try_clear(bits).unwrap();
        }

        /// Attempts to clear the CSR.
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _try_clear(bits: usize) -> $crate::result::Result<()> {
            match () {
                #[cfg(target_arch = "riscv32")]
                () => {
                    core::arch::asm!(concat!("csrrc x0, ", stringify!($csr_number), ", {0}"), in(reg) bits);
                    Ok(())
                }

                #[cfg(not(target_arch = "riscv32"))]
                () => Err($crate::result::Error::Unimplemented),
            }
        }
    };
}

/// Convenience macro to define field setter functions for a CSR type.
#[macro_export]
macro_rules! set_csr {
    ($(#[$attr:meta])*, $set_field:ident, $e:expr) => {
        $(#[$attr])*
        #[inline]
        pub unsafe fn $set_field() {
            _set($e);
        }
    };
}

/// Convenience macro to define field clear functions for a CSR type.
#[macro_export]
macro_rules! clear_csr {
    ($(#[$attr:meta])*, $clear_field:ident, $e:expr) => {
        $(#[$attr])*
        #[inline]
        pub unsafe fn $clear_field() {
            _clear($e);
        }
    };
}

/// Convenience macro to define field setter and clear functions for a CSR type.
#[macro_export]
macro_rules! set_clear_csr {
    ($(#[$attr:meta])*, $set_field:ident, $clear_field:ident, $e:expr) => {
        $crate::set_csr!($(#[$attr])*, $set_field, $e);
        $crate::clear_csr!($(#[$attr])*, $clear_field, $e);
    }
}

/// Convenience macro to read a composite value from a CSR register.
///
/// - `RV32`: reads 32-bits from `hi` and 32-bits from `lo` to create a 64-bit value
/// - `RV64`: reads a 64-bit value from `lo`
#[macro_export]
macro_rules! read_composite_csr {
    ($hi:expr, $lo:expr) => {
        /// Reads the CSR as a 64-bit value
        #[inline]
        pub fn read64() -> u64 {
            match () {
                #[cfg(target_arch = "riscv32")]
                () => loop {
                    let hi = $hi;
                    let lo = $lo;
                    if hi == $hi {
                        return ((hi as u64) << 32) | lo as u64;
                    }
                },

                #[cfg(not(target_arch = "riscv32"))]
                () => $lo as u64,
            }
        }
    };
}

macro_rules! set_pmp {
    () => {
        /// Set the pmp configuration corresponding to the index.
        ///
        /// **WARNING**: panics on non-`riscv` targets, and/or if `index` is out-of-bounds.
        #[inline]
        pub unsafe fn set_pmp(index: usize, range: Range, permission: Permission, locked: bool) {
            try_set_pmp(index, range, permission, locked).unwrap()
        }

        /// Attempts to set the pmp configuration corresponding to the index.
        ///
        /// Returns an error if the index is invalid.
        #[inline]
        pub unsafe fn try_set_pmp(
            index: usize,
            range: Range,
            permission: Permission,
            locked: bool,
        ) -> $crate::result::Result<()> {
            let max = match () {
                #[cfg(target_arch = "riscv32")]
                () => Ok(4usize),
                #[cfg(target_arch = "riscv64")]
                () => Ok(8usize),
                #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
                () => Err($crate::result::Error::Unimplemented),
            }?;

            if index < max {
                let mut value = _try_read()?;
                value &= !(0xFF << (8 * index)); // clear previous value
                let byte = (locked as usize) << 7 | (range as usize) << 3 | (permission as usize);
                value |= byte << (8 * index);
                _try_write(value)
            } else {
                Err($crate::result::Error::IndexOutOfBounds {
                    index,
                    min: 0,
                    max: max - 1,
                })
            }
        }
    };
}

macro_rules! clear_pmp {
    () => {
        /// Clear the pmp configuration corresponding to the index.
        ///
        /// **WARNING**: panics on non-`riscv` targets, and/or if `index` is out-of-bounds.
        #[inline]
        pub unsafe fn clear_pmp(index: usize) {
            try_clear_pmp(index).unwrap();
        }

        /// Attempts to clear the pmp configuration corresponding to the index.
        ///
        /// Returns an error if the index is invalid.
        #[inline]
        pub unsafe fn try_clear_pmp(index: usize) -> $crate::result::Result<()> {
            let max = match () {
                #[cfg(target_arch = "riscv32")]
                () => Ok(4usize),
                #[cfg(target_arch = "riscv64")]
                () => Ok(8usize),
                #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
                () => Err($crate::result::Error::Unimplemented),
            }?;

            if index < max {
                let mut value = _try_read()?;
                value &= !(0xFF << (8 * index)); // clear previous value
                _try_write(value)
            } else {
                Err($crate::result::Error::IndexOutOfBounds {
                    index,
                    min: 0,
                    max: max - 1,
                })
            }
        }
    };
}

/// Helper macro to define a CSR type.
///
/// This macro creates a type represents a CSR register, without defining any bitfields.
///
/// It is mainly used by [read_write_csr](crate::read_write_csr),
/// [read_only_csr](crate::read_only_csr), and [write_only_csr](crate::write_only_csr) macros.
#[macro_export]
macro_rules! csr {
    ($(#[$doc:meta])*
     $ty:ident,
     $mask:literal) => {
        #[repr(C)]
        $(#[$doc])*
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub struct $ty {
            bits: usize,
        }

        impl $ty {
            /// Bitmask for legal bitfields of the CSR type.
            pub const BITMASK: usize = $mask;

            /// Creates a new CSR type from raw bits value.
            ///
            /// Only bits in the [BITMASK](Self::BITMASK) will be set.
            pub const fn from_bits(bits: usize) -> Self {
                Self { bits: bits & $mask }
            }

            /// Gets the raw bits value.
            pub const fn bits(&self) -> usize {
                self.bits & $mask
            }

            /// Gets the bitmask for the CSR type's bitfields.
            pub const fn bitmask(&self) -> usize {
                Self::BITMASK
            }
        }
    };
}

#[macro_export]
macro_rules! csr_field_enum {
    ($(#[$field_ty_doc:meta])*
     $field_ty:ident {
         range: [$field_start:literal : $field_end:literal],
         default: $default_variant:ident,
         $($variant:ident = $value:expr$(,)?)+
     }$(,)?
    ) => {
         $(#[$field_ty_doc])*
         #[repr(usize)]
         #[derive(Clone, Copy, Debug, Eq, PartialEq)]
         pub enum $field_ty {
             $($variant = $value),+
         }

         impl $field_ty {
             /// Creates a new field variant.
             pub const fn new() -> Self {
                 Self::$default_variant
             }

             /// Attempts to convert a [`usize`] into a valid variant.
             pub const fn from_usize(val: usize) -> $crate::result::Result<Self> {
                 match val {
                     $($value => Ok(Self::$variant),)+
                     _ => Err($crate::result::Error::InvalidVariant(val)),
                 }
             }

             /// Converts the variant into a [`usize`].
             pub const fn into_usize(self) -> usize {
                 self as usize
             }
         }

         impl Default for $field_ty {
             fn default() -> Self {
                 Self::new()
             }
         }

         impl From<$field_ty> for usize {
             fn from(val: $field_ty) -> Self {
                 val.into_usize()
             }
         }
    };
}

/// Helper macro to create a read-write CSR type.
///
/// The type allows to read the CSR value into memory, and update the field values in-memory.
///
/// The user can then write the entire bitfield value back into the CSR with a single write.
#[macro_export]
macro_rules! read_write_csr {
    ($(#[$doc:meta])+
     $ty:ident: $csr:tt,
     mask: $mask:tt$(,)?
    ) => {
        $crate::csr!($(#[$doc])+ $ty, $mask);

        $crate::read_csr_as!($ty, $csr);
        $crate::write_csr_as!($ty, $csr);
    };
}

/// Helper macro to create a read-only CSR type.
///
/// The type allows to read the CSR value into memory.
#[macro_export]
macro_rules! read_only_csr {
    ($(#[$doc:meta])+
     $ty:ident: $csr:tt,
     mask: $mask:tt$(,)?
    ) => {
        $crate::csr! { $(#[$doc])+ $ty, $mask }

        $crate::read_csr_as!($ty, $csr);
    };
}

/// Helper macro to create a read-only CSR type.
///
/// The type allows to read the CSR value into memory.
#[macro_export]
macro_rules! write_only_csr {
    ($(#[$doc:meta])+
     $ty:ident: $csr:literal,
     mask: $mask:literal$(,)?
    ) => {
        $crate::csr! { $(#[$doc])+ $ty, $mask }

        $crate::write_csr_as!($ty, $csr);
    };
}

/// Defines field accesor functions for a read-write CSR type.
#[macro_export]
macro_rules! read_write_csr_field {
    ($ty:ident,
     $(#[$field_doc:meta])+
     $field:ident,
     $(#[$set_field_doc:meta])+
     $set_field:ident,
     bit: $bit:literal$(,)?
     ) => {
         $crate::read_only_csr_field!(
             $ty,
             $(#[$field_doc])+
             $field: $bit,
         );

         $crate::write_only_csr_field!(
             $ty,
             $(#[$set_field_doc])+
             $set_field: $bit,
         );
    };

    ($ty:ident,
     $(#[$field_doc:meta])+
     $field:ident,
     $(#[$try_field_doc:meta])+
     $try_field:ident,
     $(#[$set_field_doc:meta])+
     $set_field:ident,
     $(#[$try_set_field_doc:meta])+
     $try_set_field:ident,
     range: $bit_start:literal ..= $bit_end:literal$(,)?
    ) => {
         $crate::read_only_csr_field!(
             $ty,
             $(#[$field_doc])+
             $field,
             $(#[$try_field_doc])+
             $try_field,
             range: $bit_start ..= $bit_end,
         );

         $crate::write_only_csr_field!(
             $ty,
             $(#[$set_field_doc])+
             $set_field,
             $(#[$try_set_field_doc])+
             $try_set_field,
             range: $bit_start ..= $bit_end,
         );
    };

    ($ty:ident,
     $(#[$field_doc:meta])+
     $field:ident,
     $(#[$set_field_doc:meta])+
     $set_field:ident,
     range: [$bit_start:literal : $bit_end:literal]$(,)?
    ) => {
        $crate::read_only_csr_field!(
            $ty,
            $(#[$field_doc])+
            $field: [$bit_start : $bit_end],
        );

        $crate::write_only_csr_field!(
            $ty,
            $(#[$set_field_doc])+
            $set_field: [$bit_start : $bit_end],
        );
    };

    ($ty:ident,
     $(#[$field_doc:meta])+
     $field:ident,
     $(#[$try_field_doc:meta])+
     $try_field:ident,
     $(#[$set_field_doc:meta])+
     $set_field:ident,
     $(#[$field_ty_doc:meta])+
     $field_ty:ident {
         range: [$field_start:literal : $field_end:literal],
         default: $default_variant:ident,
         $($variant:ident = $value:expr$(,)?)+
     }$(,)?
    ) => {
        $crate::csr_field_enum!(
            $(#[$field_ty_doc])+
            $field_ty {
                range: [$field_start : $field_end],
                default: $default_variant,
                $($variant = $value,)+
            },
         );

         $crate::read_only_csr_field!(
             $ty,
             $(#[$field_doc])+
             $field,
             $(#[$try_field_doc])+
             $try_field,
             $field_ty,
             range: [$field_start : $field_end],
         );

         $crate::write_only_csr_field!(
             $ty,
             $(#[$set_field_doc])+
             $set_field,
             $field_ty,
             range: [$field_start : $field_end],
         );
    };
}

/// Defines field accesor functions for a read-only CSR type.
#[macro_export]
macro_rules! read_only_csr_field {
    ($ty:ident,
     $(#[$field_doc:meta])+
     $field:ident: $bit:literal$(,)?) => {
        const _: () = assert!($bit < usize::BITS);

        impl $ty {
            $(#[$field_doc])+
            #[inline]
            pub fn $field(&self) -> bool {
                $crate::bits::bf_extract(self.bits, $bit, 1) != 0
            }
        }
    };

    ($ty:ident,
     $(#[$field_doc:meta])+
     $field:ident,
     $(#[$try_field_doc:meta])+
     $try_field:ident,
     range: $bit_start:literal..=$bit_end:literal$(,)?) => {
        const _: () = assert!($bit_end < usize::BITS);
        const _: () = assert!($bit_start <= $bit_end);

        impl $ty {
            $(#[$field_doc])+
            #[inline]
            pub fn $field(&self, index: usize) -> bool {
                self.$try_field(index).unwrap()
            }

            $(#[$try_field_doc])+
            #[inline]
            pub fn $try_field(&self, index: usize) -> $crate::result::Result<bool> {
                if ($bit_start..=$bit_end).contains(&index) {
                    Ok($crate::bits::bf_extract(self.bits, index, 1) != 0)
                } else {
                    Err($crate::result::Error::IndexOutOfBounds {
                        index,
                        min: $bit_start,
                        max: $bit_end,
                    })
                }
            }
        }
    };

    ($ty:ident,
     $(#[$field_doc:meta])+
     $field:ident: [$bit_start:literal : $bit_end:literal]$(,)?) => {
        const _: () = assert!($bit_end < usize::BITS);
        const _: () = assert!($bit_start <= $bit_end);

        impl $ty {
            $(#[$field_doc])+
            #[inline]
            pub fn $field(&self) -> usize {
                $crate::bits::bf_extract(self.bits, $bit_start, $bit_end - $bit_start + 1)
            }
        }
    };

    ($ty:ident,
     $(#[$field_doc:meta])+
     $field:ident,
     $(#[$try_field_doc:meta])+
     $try_field:ident,
     $(#[$field_ty_doc:meta])+
     $field_ty:ident {
         range: [$field_start:literal : $field_end:literal],
         default: $default_variant:ident,
         $($variant:ident = $value:expr$(,)?)+
     }$(,)?
    ) => {
        $crate::csr_field_enum!(
            $(#[$field_ty_doc])+
            $field_ty {
                range: [$field_start : $field_end],
                default: $default_variant,
                $($variant = $value,)+
            },
        );

        $crate::read_only_csr_field!(
            $ty,
            $(#[$field_doc])*
            $field,
            $(#[$try_field_doc])*
            $try_field,
            $field_ty,
            range: [$field_start : $field_end],
        );
    };

    ($ty:ident,
     $(#[$field_doc:meta])+
     $field:ident,
     $(#[$try_field_doc:meta])+
     $try_field:ident,
     $field_ty:ident,
     range: [$field_start:literal : $field_end:literal]$(,)?
    ) => {
        const _: () = assert!($field_end < usize::BITS);
        const _: () = assert!($field_start <= $field_end);

        impl $ty {
            $(#[$field_doc])+
            #[inline]
            pub fn $field(&self) -> $field_ty {
                self.$try_field().unwrap()
            }

            $(#[$try_field_doc])+
            #[inline]
            pub fn $try_field(&self) -> $crate::result::Result<$field_ty> {
                let value = $crate::bits::bf_extract(
                    self.bits,
                    $field_start,
                    $field_end - $field_start + 1,
                );

                $field_ty::from_usize(value)
            }
        }
    };
}

/// Defines field accesor functions for a write-only CSR type.
#[macro_export]
macro_rules! write_only_csr_field {
    ($ty:ident,
     $(#[$field_doc:meta])+
     $field:ident: $bit:literal$(,)?) => {
        const _: () = assert!($bit < usize::BITS);

        impl $ty {
            $(#[$field_doc])+
            #[inline]
            pub fn $field(&mut self, $field: bool) {
                self.bits = $crate::bits::bf_insert(self.bits, $bit, 1, $field as usize);
            }
        }
    };

    ($ty:ident,
     $(#[$field_doc:meta])+
     $field:ident,
     $(#[$try_field_doc:meta])+
     $try_field:ident,
     range: $bit_start:literal..=$bit_end:literal$(,)?) => {
        const _: () = assert!($bit_end < usize::BITS);
        const _: () = assert!($bit_start <= $bit_end);

        impl $ty {
            $(#[$field_doc])+
            #[inline]
            pub fn $field(&mut self, index: usize, $field: bool) {
                self.$try_field(index, $field).unwrap();
            }

            $(#[$try_field_doc])+
            #[inline]
            pub fn $try_field(&mut self, index: usize, $field: bool) -> $crate::result::Result<()> {
                if ($bit_start..=$bit_end).contains(&index) {
                    self.bits = $crate::bits::bf_insert(self.bits, index, 1, $field as usize);
                    Ok(())
                } else {
                    Err($crate::result::Error::IndexOutOfBounds {
                        index,
                        min: $bit_start,
                        max: $bit_end,
                    })
                }
            }
        }
    };

    ($ty:ident,
     $(#[$field_doc:meta])+
     $field:ident: [$bit_start:literal : $bit_end:literal]$(,)?) => {
        const _: () = assert!($bit_end < usize::BITS);
        const _: () = assert!($bit_start <= $bit_end);

        impl $ty {
            $(#[$field_doc])+
            #[inline]
            pub fn $field(&mut self, $field: usize) {
                self.bits = $crate::bits::bf_insert(
                    self.bits,
                    $bit_start,
                    $bit_end - $bit_start + 1,
                    $field,
                );
            }
        }
    };

    ($ty:ident,
     $(#[$field_doc:meta])+
     $field:ident,
     $(#[$field_ty_doc:meta])+
     $field_ty:ident {
         range: [$field_start:literal : $field_end:literal],
         default: $default_variant:ident,
         $($variant:ident = $value:expr$(,)?)+
     }$(,)?
    ) => {
        $crate::csr_field_enum!(
            $(#[$field_ty_doc])+
            $field_ty {
                range: [$field_start : $field_end],
                default: $default_variant,
                $($variant = $value,)+
            },
         );

        $crate::write_only_csr_field!(
            $ty,
            $(#[$field_doc])+
            $field,
            $field_ty,
            range: [$field_start : $field_end],
        );
    };

    ($ty:ident,
     $(#[$field_doc:meta])+
     $field:ident,
     $field_ty:ident,
     range: [$field_start:literal : $field_end:literal]$(,)?
    ) => {
        const _: () = assert!($field_end < usize::BITS);
        const _: () = assert!($field_start <= $field_end);

        impl $ty {
            $(#[$field_doc])+
            #[inline]
            pub fn $field(&mut self, $field: $field_ty) {
                self.bits = $crate::bits::bf_insert(
                    self.bits,
                    $field_start,
                    $field_end - $field_start + 1,
                    $field.into(),
                );
            }
        }
    };
}
