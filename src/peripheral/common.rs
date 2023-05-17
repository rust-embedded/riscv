//! Common definitions for all the peripheral registers.

use core::marker::PhantomData;

/// Read-only type state for `A` in [`Reg`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RO;

/// Write-only type state for `A` in [`Reg`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WO;

/// Read-write type state for `A` in [`Reg`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RW;

/// Write-any-read-legal type state for `A` in [`Reg`].
/// In contrast with [`RW`] registers, `WARL` registers usually have
/// additional methods that are specific to the behavior of the register.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WARL;

/// Generic trait for all the peripheral registers.
/// This trait is sealed and cannot be implemented by any external crate.
pub trait Access: sealed::Access + Copy {}
impl Access for RO {}
impl Access for WO {}
impl Access for RW {}
impl Access for WARL {}

/// Trait for readable registers.
pub trait Read: Access {}
impl Read for RO {}
impl Read for RW {}
impl Read for WARL {}

/// Trait for writable registers.
pub trait Write: Access {}
impl Write for WO {}
impl Write for RW {}
impl Write for WARL {}

/// Generic register structure. `T` refers to the data type of the register.
/// Alternatively, `A` corresponds to the access level (e.g., read-only, read-write...).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct Reg<T: Copy, A: Access> {
    pub ptr: *mut T,
    phantom: PhantomData<A>,
}

unsafe impl<T: Copy, A: Access> Send for Reg<T, A> {}
unsafe impl<T: Copy, A: Access> Sync for Reg<T, A> {}

impl<T: Copy, A: Access> Reg<T, A> {
    #[inline(always)]
    pub const fn new(ptr: *mut T) -> Self {
        Self {
            ptr,
            phantom: PhantomData,
        }
    }
}

impl<T: Copy, A: Read> Reg<T, A> {
    /// Performs a volatile read of the peripheral register.
    ///
    /// # Note
    ///
    /// Beware of what "volatile" means in Rust (see [`core::ptr::read_volatile`]).
    ///
    /// # Safety
    ///
    /// The address assigned must be valid and must be correctly aligned.
    #[inline(always)]
    pub unsafe fn read(self) -> T {
        self.ptr.read_volatile()
    }
}

impl<T: Copy, A: Write> Reg<T, A> {
    /// Performs a volatile write of the peripheral register.
    ///
    /// # Note
    ///
    /// Beware of what "volatile" means in Rust (see [`core::ptr::read_volatile`]).
    ///
    /// # Safety
    ///
    /// The address assigned must be valid and must be correctly aligned.
    #[inline(always)]
    pub unsafe fn write(self, val: T) {
        self.ptr.write_volatile(val)
    }
}

impl<T: Copy, A: Read + Write> Reg<T, A> {
    /// It modifies the value of the register according to a given function `f`.
    /// After writing the new value to the register, it returns the value returned by `f`.
    ///
    /// # Safety
    ///
    /// It performs a non-atomic read-modify-write operation, which may lead to undefined behavior.
    #[inline(always)]
    pub unsafe fn modify<R>(self, f: impl FnOnce(&mut T) -> R) -> R {
        let mut val = self.read();
        let res = f(&mut val);
        self.write(val);
        res
    }
}

/// Macro to provide bit-wise operations to integer number registers.
macro_rules! bitwise_reg {
    ($TYPE: ty) => {
        impl<A: Read> Reg<$TYPE, A> {
            /// Reads the nth bit of the register.
            #[inline(always)]
            pub unsafe fn read_bit(self, n: $TYPE) -> bool {
                let mask = 1 << n;
                let val = self.read();
                val & mask == mask
            }
        }

        impl<A: Read + Write> Reg<$TYPE, A> {
            /// Clears the nth bit of the register.
            #[inline(always)]
            pub unsafe fn clear_bit(self, n: $TYPE) {
                let mask = 1 << n;
                let val = self.read();
                self.write(val & !mask);
            }

            /// Sets the nth bit of the register.
            #[inline(always)]
            pub unsafe fn set_bit(self, n: $TYPE) {
                let mask = 1 << n;
                let val = self.read();
                self.write(val | mask);
            }
        }
    };
}
bitwise_reg!(u8);
bitwise_reg!(u16);
bitwise_reg!(u32);
bitwise_reg!(u64);
bitwise_reg!(u128);
bitwise_reg!(usize);
bitwise_reg!(i8);
bitwise_reg!(i16);
bitwise_reg!(i32);
bitwise_reg!(i64);
bitwise_reg!(i128);
bitwise_reg!(isize);

/// Macro to define the archetypal behavior of registers.
/// You must specify the register name, its data type, and its access level.
macro_rules! peripheral_reg {
    ($REGISTER: ident, $TYPE: ty, $ACCESS: ty) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        #[repr(transparent)]
        pub struct $REGISTER {
            pub register: $crate::peripheral::common::Reg<$TYPE, $ACCESS>,
        }

        impl $REGISTER {
            #[inline(always)]
            pub const fn new(address: usize) -> Self {
                Self::from_ptr(address as _)
            }

            #[inline(always)]
            pub const fn from_ptr(ptr: *mut $TYPE) -> Self {
                Self {
                    register: Reg::new(ptr),
                }
            }
        }

        impl core::ops::Deref for $REGISTER {
            type Target = Reg<$TYPE, $ACCESS>;

            fn deref(&self) -> &Self::Target {
                &self.register
            }
        }
    };
}
pub(crate) use peripheral_reg;

mod sealed {
    use super::*;
    pub trait Access {}
    impl Access for RO {}
    impl Access for WO {}
    impl Access for RW {}
    impl Access for WARL {}
}
