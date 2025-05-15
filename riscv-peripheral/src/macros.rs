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
/// - Per-HART contexts (**OPTIONAL**): a list of `ctx` contexts for easing access to per-HART PLIC contexts.
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
/// let priorities = PLIC::priorities(); // Priorities registers
/// let pendings = PLIC::pendings();     // Pendings registers
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
        pub struct PLIC;

        unsafe impl $crate::plic::Plic for PLIC {
            const BASE: usize = $addr;
        }

        impl PLIC {
            /// Returns `true` if a machine external interrupt is pending.
            #[inline]
            pub fn is_interrupting() -> bool {
                $crate::riscv::register::mip::read().mext()
            }

            /// Returns true if Machine External Interrupts are enabled.
            #[inline]
            pub fn is_enabled() -> bool {
                $crate::riscv::register::mie::read().mext()
            }

            /// Enables machine external interrupts to allow the PLIC to trigger interrupts.
            ///
            /// # Safety
            ///
            /// Enabling the `PLIC` may break mask-based critical sections.
            #[inline]
            pub unsafe fn enable() {
                $crate::riscv::register::mie::set_mext();
            }

            /// Disables machine external interrupts to prevent the PLIC from triggering interrupts.
            #[inline]
            pub fn disable() {
                // SAFETY: it is safe to disable interrupts
                unsafe { $crate::riscv::register::mie::clear_mext() };
            }

            /// Returns the priorities register of the PLIC.
            #[inline]
            pub fn priorities() -> $crate::plic::priorities::PRIORITIES {
                $crate::plic::PLIC::<PLIC>::priorities()
            }

            /// Returns the pendings register of the PLIC.
            #[inline]
            pub fn pendings() -> $crate::plic::pendings::PENDINGS {
                $crate::plic::PLIC::<PLIC>::pendings()
            }

            /// Returns the context proxy of a given PLIC HART context.
            #[inline]
            pub fn ctx<H: $crate::plic::HartIdNumber>(hart_id: H) -> $crate::plic::CTX<Self> {
                $crate::plic::PLIC::<PLIC>::ctx(hart_id)
            }

            /// Returns the PLIC HART context for the current HART.
            ///
            /// # Note
            ///
            /// This function determines the current HART ID by reading the [`riscv::register::mhartid`] CSR.
            /// Thus, it can only be used in M-mode. For S-mode, use [`PLIC::ctx`] instead.
            #[inline]
            pub fn ctx_mhartid() -> $crate::plic::CTX<Self> {
                $crate::plic::PLIC::<PLIC>::ctx_mhartid()
            }
        }
        $crate::plic_codegen!($($tail)*);
    };
    (ctxs [$($fn:ident = ($ctx:expr , $sctx:expr)),+], $($tail:tt)*) => {
        impl PLIC {
            $(
                #[doc = "Returns a PLIC context proxy for context of HART "]
                #[doc = $sctx]
                #[doc = "."]
                #[inline]
                pub fn $fn() -> $crate::plic::CTX<Self> {
                    Self::ctx($ctx)
                }
            )*
        }
        $crate::plic_codegen!($($tail)*);
    };
}
