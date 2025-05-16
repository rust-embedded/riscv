//! Utility macros for generating standard peripherals-related code in RISC-V PACs.

pub use paste::paste;

/// Macro to create interfaces to CLINT peripherals in PACs.
/// The resulting struct will be named `CLINT`, and will provide safe access to the CLINT registers.
///
/// This macro expects 3 different argument types:
///
/// - Base address (**MANDATORY**): base address of the CLINT peripheral of the target.
/// - MTIME Frequency (**MANDATORY**): clock frequency (in Hz) of the `MTIME` register.
/// - HART map (**OPTIONAL**): a list of HART IDs and their corresponding numbers.
///
/// Check the examples below for more details about the usage and syntax of this macro.
///
/// # Example
///
/// ## Mandatory fields only
///
/// ```
/// riscv_peripheral::clint_codegen!(base 0x0200_0000, mtime_freq 32_768,); // do not forget the ending comma!
///
/// let clint = CLINT::new(); // Create a new CLINT peripheral
/// let mswi = clint.mswi();     // MSWI peripheral
/// let mtimer = clint.mtimer(); // MTIMER peripheral
/// ```
///
/// ## Base address and per-HART mtimecmp registers
///
/// ```
/// use riscv_pac::result::{Error, Result};
///
/// /// HART IDs for the target CLINT peripheral
/// #[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// pub enum HartId { H0 = 0, H1 = 1, H2 = 2 }
///
/// // Implement `HartIdNumber` for `HartId`
/// unsafe impl riscv_peripheral::aclint::HartIdNumber for HartId {
///   const MAX_HART_ID_NUMBER: usize = Self::H2 as usize;
///   fn number(self) -> usize { self as _ }
///   fn from_number(number: usize) -> Result<Self> {
///     match number {
///      0 => Ok(HartId::H0),
///      1 => Ok(HartId::H1),
///      2 => Ok(HartId::H2),
///      _ => Err(Error::InvalidVariant(number)),
///     }
///   }
/// }
///
/// riscv_peripheral::clint_codegen!(
///     base 0x0200_0000,
///     mtime_freq 32_768,
///     harts [HartId::H0 => 0, HartId::H1 => 1, HartId::H2 => 2], // do not forget the ending comma!
/// );
///
/// let clint = CLINT::new(); // Create a new CLINT peripheral
/// let mswi = clint.mswi(); // MSWI peripheral
/// let mtimer = clint.mtimer(); // MTIMER peripheral
///
/// let mtimecmp0 = clint.mtimecmp0(); // mtimecmp register for HART 0
/// let mtimecmp1 = clint.mtimecmp1(); // mtimecmp register for HART 1
/// let mtimecmp2 = clint.mtimecmp2(); // mtimecmp register for HART 2
///
/// let msip0 = clint.msip0(); // msip register for HART 0
/// let msip1 = clint.msip1(); // msip register for HART 1
/// let msip2 = clint.msip2(); // msip register for HART 2
/// ```
#[macro_export]
macro_rules! clint_codegen {
    () => {
        #[allow(unused_imports)]
        use CLINT as _; // assert that the CLINT struct is defined
    };
    (base $addr:literal, mtime_freq $freq:literal, $($tail:tt)*) => {
        /// CLINT peripheral
        #[allow(clippy::upper_case_acronyms)]
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub struct CLINT($crate::aclint::CLINT<Self>);

        impl CLINT {
            /// Creates a new `CLINT` peripheral.
            #[inline]
            pub const fn new() -> Self {
                Self($crate::aclint::CLINT::new())
            }
        }

        unsafe impl $crate::aclint::Clint for CLINT {
            const BASE: usize = $addr;
            const MTIME_FREQ: usize = $freq;
        }

        impl core::ops::Deref for CLINT {
            type Target = $crate::aclint::CLINT<Self>;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl core::ops::DerefMut for CLINT {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        $crate::clint_codegen!($($tail)*);
    };
    (harts [$($hart:expr => $num:literal),+], $($tail:tt)*) => {
        $crate::macros::paste! {
            impl CLINT {
                $(
                    #[doc = "Returns the `msip` register for HART [`"]
                    #[doc = stringify!($hart)]
                    #[doc = "`]."]
                    #[inline]
                    pub fn [<msip $num>](&self) -> $crate::aclint::mswi::MSIP {
                        self.mswi().msip($hart)
                    }
                    #[doc = "Returns the `mtimecmp` register for HART [`"]
                    #[doc = stringify!($hart)]
                    #[doc = "`]."]
                    #[inline]
                    pub fn [<mtimecmp $num>](&self) -> $crate::aclint::mtimer::MTIMECMP {
                        self.mtimer().mtimecmp($hart)
                    }
                )*
            }
        }
        $crate::clint_codegen!($($tail)*);
    };
}

/// Macro to create interfaces to PLIC peripherals in PACs.
/// The resulting struct will be named `PLIC`, and will provide safe access to the PLIC registers.
///
/// This macro expects 2 different argument types:
///
/// - Base address (**MANDATORY**): base address of the PLIC peripheral of the target.
/// - HART map (**OPTIONAL**): a list of HART IDs and their corresponding numbers.
///
/// Check the examples below for more details about the usage and syntax of this macro.
///
/// # Example
///
/// ## Base address only
///
/// ```
/// use riscv_peripheral::clint_codegen;
///
/// riscv_peripheral::plic_codegen!(base 0x0C00_0000,); // do not forget the ending comma!
///
/// let plic = PLIC::new(); // Create a new PLIC peripheral
/// let priorities = plic.priorities(); // Priorities registers
/// let pendings = plic.pendings();     // Pendings registers
/// ```
///
///
/// ## Base address and per-HART context proxies
///
/// ```
/// use riscv_pac::result::{Error, Result};
///
/// /// HART IDs for the target CLINT peripheral
/// #[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// pub enum HartId { H0 = 0, H1 = 1, H2 = 2 }
///
/// // Implement `HartIdNumber` for `HartId`
/// unsafe impl riscv_peripheral::aclint::HartIdNumber for HartId {
///   const MAX_HART_ID_NUMBER: usize = Self::H2 as usize;
///   fn number(self) -> usize { self as _ }
///   fn from_number(number: usize) -> Result<Self> {
///     match number {
///      0 => Ok(HartId::H0),
///      1 => Ok(HartId::H1),
///      2 => Ok(HartId::H2),
///      _ => Err(Error::InvalidVariant(number)),
///     }
///   }
/// }
///
/// riscv_peripheral::plic_codegen!(
///     base 0x0C00_0000,
///     harts [HartId::H0 => 0, HartId::H1 => 1, HartId::H2 => 2], // do not forget the ending comma!
/// );
///
/// let plic = PLIC::new(); // Create a new PLIC peripheral
/// let ctx0 = plic.ctx0(); // Context proxy for HART 0
/// let ctx1 = plic.ctx1(); // Context proxy for HART 1
/// let ctx2 = plic.ctx2(); // Context proxy for HART 2
///
/// assert_eq!(ctx0, plic.ctx(HartId::H0));
/// assert_eq!(ctx1, plic.ctx(HartId::H1));
/// assert_eq!(ctx2, plic.ctx(HartId::H2));
/// ```
#[macro_export]
macro_rules! plic_codegen {
    () => {
        #[allow(unused_imports)]
        use PLIC as _; // assert that the PLIC struct is defined
    };
    (base $addr:literal, $($tail:tt)*) => {
        /// PLIC peripheral
        #[allow(clippy::upper_case_acronyms)]
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub struct PLIC($crate::plic::PLIC<Self>);

        impl PLIC {
            /// Creates a new `CLINT` peripheral.
            #[inline]
            pub const fn new() -> Self {
                Self($crate::plic::PLIC::new())
            }
        }

        unsafe impl $crate::plic::Plic for PLIC {
            const BASE: usize = $addr;
        }

        impl core::ops::Deref for PLIC {
            type Target = $crate::plic::PLIC<Self>;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl core::ops::DerefMut for PLIC {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        $crate::plic_codegen!($($tail)*);
    };
    (harts [$($hart:expr => $num:literal),+], $($tail:tt)*) => {
        $crate::macros::paste! {
            impl PLIC {
                $(
                    #[doc = "Returns a PLIC context proxy for context of HART "]
                    #[doc = stringify!($hart)]
                    #[doc = "`]."]
                    #[inline]
                    pub fn [<ctx $num>](&self) -> $crate::plic::CTX<Self> {
                        self.ctx($hart)
                    }
                )*
            }
        }
        $crate::plic_codegen!($($tail)*);
    };
}
