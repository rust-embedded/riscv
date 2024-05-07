/// Convenience macro to wrap the `csrrs` assembly instruction for reading a CSR register.
///
/// This macro should generally not be called directly.
///
/// Instead, use the [read_csr_as](crate::read_csr_as) or [read_csr_as_usize](crate::read_csr_as_usize) macros.
#[macro_export]
macro_rules! read_csr {
    ($csr_number:literal) => {
        /// Reads the CSR
        #[inline]
        unsafe fn _read() -> usize {
            match () {
                #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
                () => {
                    let r: usize;
                    core::arch::asm!(concat!("csrrs {0}, ", stringify!($csr_number), ", x0"), out(reg) r);
                    r
                }

                #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
                () => unimplemented!(),
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
        /// Reads the CSR
        #[inline]
        unsafe fn _read() -> usize {
            match () {
                #[cfg(target_arch = "riscv32")]
                () => {
                    let r: usize;
                    core::arch::asm!(concat!("csrrs {0}, ", stringify!($csr_number), ", x0"), out(reg) r);
                    r
                }

                #[cfg(not(target_arch = "riscv32"))]
                () => unimplemented!(),
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

        /// Reads the CSR
        #[inline]
        pub fn read() -> $register {
            $register {
                bits: unsafe { _read() },
            }
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

        /// Reads the CSR
        #[inline]
        pub fn read() -> $register {
            $register {
                bits: unsafe { _read() },
            }
        }
    };
}

/// Convenience macro to read a CSR register value as a [`usize`].
#[macro_export]
macro_rules! read_csr_as_usize {
    ($csr_number:literal) => {
        $crate::read_csr!($csr_number);

        /// Reads the CSR
        #[inline]
        pub fn read() -> usize {
            unsafe { _read() }
        }
    };
}

/// `RV32`: Convenience macro to read a CSR register value as a [`usize`].
#[macro_export]
macro_rules! read_csr_as_usize_rv32 {
    ($csr_number:literal) => {
        $crate::read_csr_rv32!($csr_number);

        /// Reads the CSR
        #[inline]
        pub fn read() -> usize {
            unsafe { _read() }
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
        /// Writes the CSR
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _write(bits: usize) {
            match () {
                #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
                () => core::arch::asm!(concat!("csrrw x0, ", stringify!($csr_number), ", {0}"), in(reg) bits),

                #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
                () => unimplemented!(),
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
        /// Writes the CSR
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _write(bits: usize) {
            match () {
                #[cfg(target_arch = "riscv32")]
                () => core::arch::asm!(concat!("csrrw x0, ", stringify!($csr_number), ", {0}"), in(reg) bits),

                #[cfg(not(target_arch = "riscv32"))]
                () => unimplemented!(),
            }
        }
    };
}

/// Convenience macro to write a value with `bits` to a CSR
#[macro_export]
macro_rules! write_csr_as {
    ($csr_type:ty, $csr_number:literal) => {
        $crate::write_csr!($csr_number);

        /// Writes the CSR
        #[inline]
        pub fn write(value: $csr_type) {
            unsafe { _write(value.bits) }
        }
    };
}

/// Convenience macro to write a value to a CSR register.
#[macro_export]
macro_rules! write_csr_as_rv32 {
    ($csr_type:ty, $csr_number:literal) => {
        $crate::write_csr_rv32!($csr_number);

        /// Writes the CSR
        #[inline]
        pub fn write(value: $csr_type) {
            unsafe { _write(value.bits) }
        }
    };
}

/// Convenience macro to write a [`usize`] value to a CSR register.
#[macro_export]
macro_rules! write_csr_as_usize {
    ($csr_number:literal) => {
        $crate::write_csr!($csr_number);

        /// Writes the CSR
        #[inline]
        pub fn write(bits: usize) {
            unsafe { _write(bits) }
        }
    };
}

/// `RV32`: Convenience macro to write a [`usize`] value to a CSR register.
#[macro_export]
macro_rules! write_csr_as_usize_rv32 {
    ($csr_number:literal) => {
        $crate::write_csr_rv32!($csr_number);

        /// Writes the CSR
        #[inline]
        pub fn write(bits: usize) {
            unsafe { _write(bits) }
        }
    };
}

/// Convenience macro around the `csrrs` assembly instruction to set the CSR register.
///
/// This macro is intended for use with the [set_csr](crate::set_csr) or [set_clear_csr](crate::set_clear_csr) macros.
#[macro_export]
macro_rules! set {
    ($csr_number:literal) => {
        /// Set the CSR
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _set(bits: usize) {
            match () {
                #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
                () => core::arch::asm!(concat!("csrrs x0, ", stringify!($csr_number), ", {0}"), in(reg) bits),

                #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
                () => unimplemented!(),
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
        /// Set the CSR
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _set(bits: usize) {
            match () {
                #[cfg(target_arch = "riscv32")]
                () => core::arch::asm!(concat!("csrrs x0, ", stringify!($csr_number), ", {0}"), in(reg) bits),

                #[cfg(not(target_arch = "riscv32"))]
                () => unimplemented!(),
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
        /// Clear the CSR
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _clear(bits: usize) {
            match () {
                #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
                () => core::arch::asm!(concat!("csrrc x0, ", stringify!($csr_number), ", {0}"), in(reg) bits),

                #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
                () => unimplemented!(),
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
        /// Clear the CSR
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _clear(bits: usize) {
            match () {
                #[cfg(target_arch = "riscv32")]
                () => core::arch::asm!(concat!("csrrc x0, ", stringify!($csr_number), ", {0}"), in(reg) bits),

                #[cfg(not(target_arch = "riscv32"))]
                () => unimplemented!(),
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
        /// Set the pmp configuration corresponding to the index
        #[inline]
        pub unsafe fn set_pmp(index: usize, range: Range, permission: Permission, locked: bool) {
            #[cfg(target_arch = "riscv32")]
            assert!(index < 4);

            #[cfg(target_arch = "riscv64")]
            assert!(index < 8);

            let mut value = _read();
            value &= !(0xFF << (8 * index)); // clear previous value
            let byte = (locked as usize) << 7 | (range as usize) << 3 | (permission as usize);
            value |= byte << (8 * index);
            _write(value);
        }
    };
}

macro_rules! clear_pmp {
    () => {
        /// Clear the pmp configuration corresponding to the index
        #[inline]
        pub unsafe fn clear_pmp(index: usize) {
            #[cfg(target_arch = "riscv32")]
            assert!(index < 4);

            #[cfg(target_arch = "riscv64")]
            assert!(index < 8);

            let mut value = _read();
            value &= !(0xFF << (8 * index)); // clear previous value
            _write(value);
        }
    };
}
