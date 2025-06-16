//! Common definitions for all the peripheral registers.

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
    phantom: core::marker::PhantomData<A>,
}

unsafe impl<T: Copy + Send, A: Access> Send for Reg<T, A> {}
unsafe impl<T: Copy + Sync, A: Access> Sync for Reg<T, A> {}

impl<T: Copy, A: Access> Reg<T, A> {
    /// Creates a new register from a pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be valid and must be correctly aligned.
    #[inline]
    pub const unsafe fn new(ptr: *mut T) -> Self {
        Self {
            ptr,
            phantom: core::marker::PhantomData,
        }
    }

    /// Returns a pointer to the register.
    #[inline]
    pub const fn get_ptr(self) -> *mut T {
        self.ptr
    }
}

impl<T: Copy, A: Read> Reg<T, A> {
    /// Performs a volatile read of the peripheral register with no side effects.
    ///
    /// # Note
    ///
    /// If you want to perform a read-modify-write operation, use [`Reg::modify`] instead.
    #[inline]
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
    /// If you want to perform a read-modify-write operation, use [`Reg::modify`] instead.
    #[inline]
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
    #[inline]
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
            /// Reads the `n`th bit of the register.
            #[inline]
            pub fn read_bit(self, n: usize) -> bool {
                let mask = 1 << n;
                let val = self.read();
                val & mask == mask
            }

            /// Reads a range of bits of the register specified by the `start` and `end` indexes, both included.
            #[inline]
            pub fn read_bits(self, start: usize, end: usize) -> $TYPE {
                let n_bits = end - start + 1;
                let mask = ((1 << n_bits) - 1) << start;
                let val = self.read();
                (val & mask) >> start
            }
        }

        impl<A: Read + Write> Reg<$TYPE, A> {
            /// Clears the `n`th bit of the register.
            ///
            /// # Note
            ///
            /// It performs a non-atomic read-modify-write operation, which may lead to **wrong** behavior.
            #[inline]
            pub fn clear_bit(self, n: usize) {
                self.modify(|val| *val &= !(1 << n));
            }

            /// Sets the nth bit of the register.
            ///
            /// # Note
            ///
            /// It performs a non-atomic read-modify-write operation, which may lead to **wrong** behavior.
            #[inline]
            pub fn set_bit(self, n: usize) {
                self.modify(|val| *val |= 1 << n);
            }

            /// Writes a range of bits of the register specified by the `start` and `end` indexes, both included.
            #[inline]
            pub fn write_bits(self, start: usize, end: usize, val: $TYPE) {
                let n_bits = end - start + 1;
                let mask = ((1 << n_bits) - 1) << start;
                self.modify(|v| *v = (*v & !mask) | ((val << start) & mask));
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

/// Macro to provide atomic bit-wise operations to integer number registers.
#[cfg(any(
    target_has_atomic = "8",
    target_has_atomic = "16",
    target_has_atomic = "32",
    target_has_atomic = "64",
    target_has_atomic = "ptr"
))]
macro_rules! bitwise_atomic_reg {
    ($TYPE: ty, $ATOMIC: ty) => {
        impl<A: Read + Write> Reg<$TYPE, A> {
            /// Creates a new atomic reference to the register.
            ///
            /// # Safety
            ///
            /// * Register must be properly aligned **for atomic operations**.
            /// * The register must not be accessed through non-atomic operations for the whole lifetime `'a`.
            pub unsafe fn as_atomic<'a>(&self) -> &'a $ATOMIC {
                // SAFETY: guaranteed by the caller
                unsafe { &*self.ptr.cast() }
            }

            /// Clears the `n`th bit of the register atomically.
            ///
            /// # Safety
            ///
            /// * Register must be properly aligned **for atomic operations**.
            /// * The register must not be accessed through non-atomic operations until this function returns.
            #[inline]
            pub unsafe fn atomic_clear_bit(&self, n: usize, order: core::sync::atomic::Ordering) {
                // SAFETY: guaranteed by the caller
                unsafe { self.as_atomic() }.fetch_and(!(1 << n), order);
            }

            /// Sets the `n`th bit of the register atomically.
            ///
            /// # Safety
            ///
            /// * Register must be properly aligned **for atomic operations**.
            /// * The register must not be accessed through non-atomic operations until this function returns.
            #[inline]
            pub unsafe fn atomic_set_bit(&self, n: usize, order: core::sync::atomic::Ordering) {
                // SAFETY: guaranteed by the caller
                unsafe { self.as_atomic() }.fetch_or(1 << n, order);
            }
        }
    };
}

#[cfg(target_has_atomic = "8")]
bitwise_atomic_reg!(u8, core::sync::atomic::AtomicU8);
#[cfg(target_has_atomic = "16")]
bitwise_atomic_reg!(u16, core::sync::atomic::AtomicU16);
#[cfg(target_has_atomic = "32")]
bitwise_atomic_reg!(u32, core::sync::atomic::AtomicU32);
#[cfg(target_has_atomic = "64")]
bitwise_atomic_reg!(u64, core::sync::atomic::AtomicU64);
#[cfg(target_has_atomic = "ptr")]
bitwise_atomic_reg!(usize, core::sync::atomic::AtomicUsize);
#[cfg(target_has_atomic = "8")]
bitwise_atomic_reg!(i8, core::sync::atomic::AtomicI8);
#[cfg(target_has_atomic = "16")]
bitwise_atomic_reg!(i16, core::sync::atomic::AtomicI16);
#[cfg(target_has_atomic = "32")]
bitwise_atomic_reg!(i32, core::sync::atomic::AtomicI32);
#[cfg(target_has_atomic = "64")]
bitwise_atomic_reg!(i64, core::sync::atomic::AtomicI64);
#[cfg(target_has_atomic = "ptr")]
bitwise_atomic_reg!(isize, core::sync::atomic::AtomicIsize);

/// Macro to define the archetypal behavior of registers.
macro_rules! peripheral {
    ($REGISTER: ident, $TYPE: ty, $ACCESS: ident) => {
        /// Peripheral register.
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        #[repr(transparent)]
        pub struct $REGISTER {
            register: $crate::common::Reg<$TYPE, $crate::common::$ACCESS>,
        }

        impl $REGISTER {
            /// Creates a new register from an address.
            ///
            /// # Safety
            ///
            /// The address assigned must be valid and must be correctly aligned.
            #[inline]
            pub const unsafe fn new(address: usize) -> Self {
                Self {
                    register: unsafe { $crate::common::Reg::new(address as _) },
                }
            }
        }
    };
    ($REGISTER: ident, $TYPE: ty, $ACCESS: ident, $GENERIC: ident) => {
        /// Peripheral register.
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        #[repr(transparent)]
        pub struct $REGISTER<$GENERIC> {
            register: $crate::common::Reg<$TYPE, $crate::common::$ACCESS>,
            _marker: core::marker::PhantomData<$GENERIC>,
        }

        impl<$GENERIC> $REGISTER<$GENERIC> {
            /// Creates a new register from an address.
            ///
            /// # Safety
            ///
            /// The address assigned must be valid and must be correctly aligned.
            #[inline]
            pub const unsafe fn new(address: usize) -> Self {
                Self {
                    register: $crate::common::Reg::new(address as _),
                    _marker: core::marker::PhantomData,
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
    ($REGISTER: ident, $TYPE: ty, $ACCESS: ident) => {
        $crate::common::peripheral!($REGISTER, $TYPE, $ACCESS);

        impl $REGISTER {
            /// Returns the underlying raw register.
            #[inline]
            pub const fn get_register(self) -> $crate::common::Reg<$TYPE, $crate::common::$ACCESS> {
                self.register
            }
        }

        impl core::ops::Deref for $REGISTER {
            type Target = $crate::common::Reg<$TYPE, $crate::common::$ACCESS>;

            fn deref(&self) -> &Self::Target {
                &self.register
            }
        }
    };
    ($REGISTER: ident, $TYPE: ty, $ACCESS: ident, $GENERIC: ident) => {
        $crate::common::peripheral!($REGISTER, $TYPE, $ACCESS, $GENERIC);

        impl $REGISTER {
            /// Returns the underlying raw register.
            #[inline]
            pub const fn get_register(self) -> $crate::common::Reg<$TYPE, $crate::common::$ACCESS> {
                self.register
            }
        }

        impl<$GENERIC> core::ops::Deref for $REGISTER<$GENERIC> {
            type Target = $crate::common::Reg<$TYPE, $crate::common::$ACCESS>;

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
    ($REGISTER: ident, $TYPE: ty, $ACCESS: ident) => {
        $crate::common::peripheral!($REGISTER, $TYPE, $ACCESS);

        impl $REGISTER {
            /// Returns a raw pointer to the register.
            #[inline]
            pub const fn get_ptr(self) -> *mut $TYPE {
                self.register.get_ptr()
            }

            /// Returns the underlying raw register.
            ///
            /// # Safety
            ///
            /// This register is not supposed to be used directly.
            /// Use the other provided methods instead. Otherwise, use this method at your own risk.
            #[inline]
            pub const unsafe fn get_register(
                self,
            ) -> $crate::common::Reg<$TYPE, $crate::common::$ACCESS> {
                self.register
            }
        }
    };
    ($REGISTER: ident, $TYPE: ty, $ACCESS: ident, $GENERIC: ident) => {
        $crate::common::peripheral!($REGISTER, $TYPE, $ACCESS, $GENERIC);

        impl<$GENERIC> $REGISTER<$GENERIC> {
            /// Returns a raw pointer to the register.
            #[inline]
            pub const fn get_ptr(self) -> *mut $TYPE {
                self.register.get_ptr()
            }

            /// Returns the underlying register.
            ///
            /// # Safety
            ///
            /// This register is not supposed to be used directly.
            /// Use the other provided methods instead. Otherwise, use this method at your own risk.
            #[inline]
            pub const unsafe fn get_register(
                self,
            ) -> $crate::common::Reg<$TYPE, $crate::common::$ACCESS> {
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
