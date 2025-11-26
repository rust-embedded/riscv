//! Utility macros for generating standard peripherals-related code in RISC-V PACs.

pub use paste::paste;

/// Macro to create interfaces to CLINT peripherals in PACs.
/// The resulting struct will provide safe access to the CLINT registers.
///
/// This macro expects 5 different argument types:
///
/// - Visibility (**MANDATORY**): visibility of the `fn new()` function for creating a new CLINT.
///   It can be ``, `pub`, `pub(crate)`, or `pub(super)`. If empty, the function will be private.
/// - Peripheral name (**MANDATORY**): name of the resulting CLINT peripheral.
/// - Base address (**MANDATORY**): base address of the CLINT peripheral of the target.
/// - MTIME Frequency (**MANDATORY**): clock frequency (in Hz) of the `MTIME` register.
/// - HART map (**OPTIONAL**): a list of HART IDs and their corresponding numbers.
///
/// Check the examples below for more details about the usage and syntax of this macro.
///
/// # Example
///
/// ## Mandatory fields only, public `fn new()` function
///
/// ```
/// riscv_peripheral::clint_codegen!(pub CLINT, base 0x0200_0000, mtime_freq 32_768);
///
/// let clint = CLINT::new(); // Create a new CLINT peripheral (new is public)
/// let mswi = clint.mswi();     // MSWI peripheral
/// let mtimer = clint.mtimer(); // MTIMER peripheral
/// ```
///
/// ## Base address and per-HART mtimecmp registers, private `fn new()` function
///
/// ```
/// use riscv::result::{Error, Result};
///
/// /// HART IDs for the target CLINT peripheral
/// #[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// #[riscv::pac_enum(unsafe HartIdNumber)]
/// pub enum HartId { H0 = 0, H1 = 1, H2 = 2 }
///
/// riscv_peripheral::clint_codegen!(
///     Clint,
///     base 0x0200_0000,
///     mtime_freq 32_768,
///     harts [HartId::H0 => 0, HartId::H1 => 1, HartId::H2 => 2]
/// );
///
/// let clint = Clint::new(); // Create a new CLINT peripheral (new is private)
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
    ($vis:vis $name:ident, base $addr:literal, mtime_freq $freq:literal) => {
        /// CLINT peripheral
        #[allow(clippy::upper_case_acronyms)]
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub struct $name($crate::aclint::CLINT<Self>);

        impl $name {
            /// Creates a new `CLINT` peripheral.
            #[inline]
            $vis const fn new() -> Self {
                Self($crate::aclint::CLINT::new())
            }
        }

        unsafe impl $crate::aclint::Clint for $name {
            const BASE: usize = $addr;
            const MTIME_FREQ: usize = $freq;
        }

        impl core::ops::Deref for $name {
            type Target = $crate::aclint::CLINT<Self>;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
    ($vis:vis $name:ident, base $addr:literal, mtime_freq $freq:literal, harts [$($hart:expr => $num:literal),+]) => {
        $crate::clint_codegen!($vis $name, base $addr, mtime_freq $freq);
        $crate::macros::paste! {
            impl $name {
                $(
                    #[doc = "Returns the `msip` register for HART "]
                    #[doc = stringify!($num)]
                    #[doc = "."]
                    #[inline]
                    pub fn [<msip $num>](&self) -> $crate::aclint::mswi::MSIP {
                        self.mswi().msip($hart)
                    }
                    #[doc = "Returns the `mtimecmp` register for HART "]
                    #[doc = stringify!($num)]
                    #[doc = "."]
                    #[inline]
                    pub fn [<mtimecmp $num>](&self) -> $crate::aclint::mtimer::MTIMECMP {
                        self.mtimer().mtimecmp($hart)
                    }
                )*
            }
        }
    };
}

/// Macro to create interfaces to PLIC peripherals in PACs.
/// The resulting struct will be named `PLIC`, and will provide safe access to the PLIC registers.
///
/// This macro expects 4 different argument types:
///
/// - Visibility (**MANDATORY**): visibility of the `fn new()` function for creating a new PLIC.
///   It can be ``, `pub`, `pub(crate)`, or `pub(super)`. If empty, the function will be private.
/// - Peripheral name (**MANDATORY**): name of the resulting PLIC peripheral.
/// - Base address (**MANDATORY**): base address of the PLIC peripheral of the target.
/// - HART map (**OPTIONAL**): a list of HART IDs and their corresponding numbers.
///
/// Check the examples below for more details about the usage and syntax of this macro.
///
/// # Example
///
/// ## Base address only, public `fn new()` function
///
/// ```
/// use riscv_peripheral::clint_codegen;
///
/// riscv_peripheral::plic_codegen!(pub PLIC, base 0x0C00_0000);
///
/// let plic = PLIC::new(); // Create a new PLIC peripheral (new is public)
/// let priorities = plic.priorities(); // Priorities registers
/// let pendings = plic.pendings();     // Pendings registers
/// ```
///
/// ## Base address and per-HART context proxies, private `fn new()` function
///
/// ```
/// use riscv::result::{Error, Result};
///
/// /// HART IDs for the target CLINT peripheral
/// #[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// #[riscv::pac_enum(unsafe HartIdNumber)]
/// pub enum HartId { H0 = 0, H1 = 1, H2 = 2 }
///
/// riscv_peripheral::plic_codegen!(
///     Plic,
///     base 0x0C00_0000,
///     harts [HartId::H0 => 0, HartId::H1 => 1, HartId::H2 => 2]
/// );
///
/// let plic = Plic::new(); // Create a new PLIC peripheral (new is private)
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
    ($vis:vis $name:ident, base $addr:literal) => {
        /// PLIC peripheral
        #[allow(clippy::upper_case_acronyms)]
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub struct $name($crate::plic::PLIC<Self>);

        impl $name {
            /// Creates a new `PLIC` peripheral.
            #[inline]
            $vis const fn new() -> Self {
                Self($crate::plic::PLIC::new())
            }
        }

        unsafe impl $crate::plic::Plic for $name {
            const BASE: usize = $addr;
        }

        impl core::ops::Deref for $name {
            type Target = $crate::plic::PLIC<Self>;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
    ($vis:vis $name:ident, base $addr:literal, harts [$($hart:expr => $num:literal),+]) => {
        $crate::plic_codegen!($vis $name, base $addr);
        $crate::macros::paste! {
            impl $name {
                $(
                    #[doc = "Returns a PLIC context proxy for context of HART "]
                    #[doc = stringify!($num)]
                    #[doc = "."]
                    #[inline]
                    pub fn [<ctx $num>](&self) -> $crate::plic::CTX<Self> {
                        self.ctx($hart)
                    }
                )*
            }
        }
    };
}
