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

/// Generic trait for all the peripheral registers.
/// This trait is sealed and cannot be implemented by any external crate.
pub trait Access: sealed::Access + Copy {}
impl Access for RO {}
impl Access for WO {}
impl Access for RW {}

/// Trait for readable registers.
pub trait Read: Access {}
impl Read for RO {}
impl Read for RW {}

/// Trait for writable registers.
pub trait Write: Access {}
impl Write for WO {}
impl Write for RW {}

/// Generic register structure. `T` refers to the data type of the register.
/// Alternatively, `A` corresponds to the access level (e.g., read-only, read-write...).
///
/// # Note
///
/// This structure assumes that it points to a valid peripheral register.
/// If so, it is safe to read from or write to the register.
/// However, keep in mind that read-modify-write operations may lead to **wrong** behavior.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct Reg<T: Copy, A: Access> {
    ptr: *mut T,
    phantom: PhantomData<A>,
}

unsafe impl<T: Copy, A: Access> Send for Reg<T, A> {}
unsafe impl<T: Copy, A: Access> Sync for Reg<T, A> {}

impl<T: Copy, A: Access> Reg<T, A> {
    /// Creates a new register from a pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be valid and must be correctly aligned.
    #[inline(always)]
    pub unsafe fn new(ptr: *mut T) -> Self {
        Self {
            ptr,
            phantom: PhantomData,
        }
    }

    /// Returns a pointer to the register.
    #[inline(always)]
    pub fn get_ptr(self) -> *mut T {
        self.ptr
    }
}

impl<T: Copy, A: Read> Reg<T, A> {
    /// Performs a volatile read of the peripheral register with no side effects.
    ///
    /// # Note
    ///
    /// Beware of what "volatile" means in Rust (see [`core::ptr::read_volatile`]).
    ///
    /// If you want to perform a read-modify-write operation, use [`Reg::modify`] instead.
    #[inline(always)]
    pub fn read(self) -> T {
        // SAFETY: valid address and register is readable
        unsafe { self.ptr.read_volatile() }
    }
}

impl<T: Copy, A: Write> Reg<T, A> {
    /// Performs a volatile write of the peripheral register.
    ///
    /// # Note
    ///
    /// Beware of what "volatile" means in Rust (see [`core::ptr::read_volatile`]).
    ///
    /// If you want to perform a read-modify-write operation, use [`Reg::modify`] instead.
    #[inline(always)]
    pub fn write(self, val: T) {
        // SAFETY: valid address and register is writable
        unsafe { self.ptr.write_volatile(val) }
    }
}

impl<T: Copy, A: Read + Write> Reg<T, A> {
    /// It modifies the value of the register according to a given function `f`.
    /// After writing the new value to the register, it returns the value returned by `f`.
    ///
    /// # Note
    ///
    /// It performs a non-atomic read-modify-write operation, which may lead to **wrong** behavior.
    #[inline(always)]
    pub fn modify<R>(self, f: impl FnOnce(&mut T) -> R) -> R {
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
            pub fn read_bit(self, n: $TYPE) -> bool {
                let mask = 1 << n;
                let val = self.read();
                val & mask == mask
            }
        }

        impl<A: Read + Write> Reg<$TYPE, A> {
            /// Clears the nth bit of the register.
            ///
            /// # Note
            ///
            /// It performs a non-atomic read-modify-write operation, which may lead to **wrong** behavior.
            #[inline(always)]
            pub fn clear_bit(self, n: $TYPE) {
                self.modify(|val| *val &= !(1 << n));
            }

            /// Sets the nth bit of the register.
            ///
            /// # Note
            ///
            /// It performs a non-atomic read-modify-write operation, which may lead to **wrong** behavior.
            #[inline(always)]
            pub unsafe fn set_bit(self, n: $TYPE) {
                self.modify(|val| *val |= 1 << n);
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
macro_rules! peripheral {
    ($REGISTER: ident, $TYPE: ty, $ACCESS: ty) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        #[repr(transparent)]
        pub struct $REGISTER {
            register: $crate::peripheral::common::Reg<$TYPE, $ACCESS>,
        }

        impl $REGISTER {
            /// Creates a new register from an address.
            ///
            /// # Safety
            ///
            /// The address assigned must be valid and must be correctly aligned.
            #[inline(always)]
            pub unsafe fn new(address: usize) -> Self {
                Self {
                    register: $crate::peripheral::common::Reg::new(address as _),
                }
            }
        }
    };
}

/// Macro to define the archetypal behavior of *safe* registers.
/// You must specify the register name, its data type, and its access level.
///
/// # Note
///
/// Safe peripheral registers implement [`core::ops::Deref`] to [`Reg`].
/// You can safely use the dereferenced [`Reg::read`], [`Reg::write`], and/or [`Reg::modify`] methods.
macro_rules! safe_peripheral {
    ($REGISTER: ident, $TYPE: ty, $ACCESS: ty) => {
        $crate::peripheral::common::peripheral!($REGISTER, $TYPE, $ACCESS);

        impl core::ops::Deref for $REGISTER {
            type Target = $crate::peripheral::common::Reg<$TYPE, $ACCESS>;

            fn deref(&self) -> &Self::Target {
                &self.register
            }
        }
    };
}

/// Macro to define the archetypal behavior of *unsafe* registers.
/// You must specify the register name, its data type, and its access level.
///
/// # Note
///
/// Unsafe peripheral registers need special care when reading and/or writing.
/// They usually provide additional methods to perform safe (or unsafe) operations.
/// Nevertheless, you can still access the underlying register using the `unsafe get_register(self)` method.
macro_rules! unsafe_peripheral {
    ($REGISTER: ident, $TYPE: ty, $ACCESS: ty) => {
        $crate::peripheral::common::peripheral!($REGISTER, $TYPE, $ACCESS);

        impl $REGISTER {
            #[inline(always)]
            pub fn get_ptr(self) -> *mut $TYPE {
                self.register.get_ptr()
            }

            /// Returns the underlying register.
            ///
            /// # Safety
            ///
            /// This register is not supposed to be used directly.
            /// Use the other provided methods instead. Otherwise, use this method at your own risk.
            #[inline(always)]
            pub unsafe fn get_register(self) -> $crate::peripheral::common::Reg<$TYPE, $ACCESS> {
                self.register
            }
        }
    };
}

pub(crate) use {peripheral, safe_peripheral, unsafe_peripheral};

mod sealed {
    use super::*;
    pub trait Access {}
    impl Access for RO {}
    impl Access for WO {}
    impl Access for RW {}
}
