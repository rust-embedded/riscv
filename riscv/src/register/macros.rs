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
