/// Macro to create interfaces to CLINT peripherals in PACs.
/// The resulting struct will be named `CLINT`, and will provide safe access to the CLINT registers.
///
/// This macro expects 2 different argument types:
///
/// - Base address (**MANDATORY**): base address of the CLINT peripheral of the target.
/// - Per-HART mtimecmp registers (**OPTIONAL**): a list of `mtimecmp` registers for easing access to per-HART mtimecmp regs.
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
/// clint_codegen!(base 0x0200_0000,); // do not forget the ending comma!
///
/// let mswi = CLINT::mswi(); // MSWI peripheral
/// let mtimer = CLINT::mtimer(); // MTIMER peripheral
/// ```
///
/// ## Base address and per-HART mtimecmp registers
///
/// ```
/// use riscv_peripheral::clint_codegen;
///
/// /// HART IDs for the target CLINT peripheral
/// #[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// #[repr(u16)]
/// pub enum HartId { H0 = 0, H1 = 1, H2 = 2 }
///
/// // Implement `HartIdNumber` for `HartId`
/// unsafe impl riscv_peripheral::aclint::HartIdNumber for HartId {
///   const MAX_HART_ID_NUMBER: u16 = 2;
///   fn number(self) -> u16 { self as _ }
///   fn from_number(number: u16) -> Result<Self, u16> {
///     if number > Self::MAX_HART_ID_NUMBER {
///        Err(number)
///     } else {
///        // SAFETY: valid context number
///        Ok(unsafe { core::mem::transmute(number) })
///     }
///   }
/// }
///
/// clint_codegen!(
///     base 0x0200_0000,
///     mtimecmps [mtimecmp0 = HartId::H0, mtimecmp1 = HartId::H1, mtimecmp2 = HartId::H2], // do not forget the ending comma!
/// );
///
/// let mswi = CLINT::mswi(); // MSWI peripheral
/// let mtimer = CLINT::mtimer(); // MTIMER peripheral
///
/// let mtimecmp0 = CLINT::mtimecmp0(); // mtimecmp register for HART 0
/// let mtimecmp1 = CLINT::mtimecmp1(); // mtimecmp register for HART 1
/// let mtimecmp2 = CLINT::mtimecmp2(); // mtimecmp register for HART 2
/// ```
#[macro_export]
macro_rules! clint_codegen {
    () => {
        #[allow(unused_imports)]
        use CLINT as _; // assert that the CLINT struct is defined
    };
    (base $addr:literal, $($tail:tt)*) => {
        /// CLINT peripheral
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub struct CLINT;

        unsafe impl $crate::aclint::Clint for CLINT {
            const BASE: usize = $addr;
        }

        impl CLINT {
            /// Returns the `MSWI` peripheral.
            #[inline]
            pub const fn mswi() -> $crate::aclint::mswi::MSWI {
                $crate::aclint::CLINT::<CLINT>::mswi()
            }

            /// Returns the `MTIMER` peripheral.
            #[inline]
            pub const fn mtimer() -> $crate::aclint::mtimer::MTIMER {
                $crate::aclint::CLINT::<CLINT>::mtimer()
            }
        }
        $crate::clint_codegen!($($tail)*);
    };
    (mtimecmps [$($fn:ident = $hart:expr),+], $($tail:tt)*) => {
        impl CLINT {
            $(
                /// Returns the `MTIMECMP` register for the given HART.
                #[inline]
                pub fn $fn() -> $crate::aclint::mtimer::MTIMECMP {
                    Self::mtimer().mtimecmp($hart)
                }
            )*
        }
        $crate::clint_codegen!($($tail)*);
    };
}

#[cfg(test)]
mod tests {
    use crate::aclint::test::HartId;

    #[test]
    fn test_clint_mtimecmp() {
        // Call CLINT macro with a base address and a list of mtimecmps for easing access to per-HART mtimecmp regs.
        clint_codegen!(
            base 0x0200_0000,
            mtimecmps [mtimecmp0=HartId::H0, mtimecmp1=HartId::H1, mtimecmp2=HartId::H2],
        );

        let mswi = CLINT::mswi();
        let mtimer = CLINT::mtimer();

        assert_eq!(mswi.msip0.get_ptr() as usize, 0x0200_0000);
        assert_eq!(mtimer.mtimecmp0.get_ptr() as usize, 0x0200_4000);
        assert_eq!(mtimer.mtime.get_ptr() as usize, 0x0200_bff8);

        let mtimecmp0 = mtimer.mtimecmp(HartId::H0);
        let mtimecmp1 = mtimer.mtimecmp(HartId::H1);
        let mtimecmp2 = mtimer.mtimecmp(HartId::H2);

        assert_eq!(mtimecmp0.get_ptr() as usize, 0x0200_4000);
        assert_eq!(mtimecmp1.get_ptr() as usize, 0x0200_4000 + 1 * 8); // 8 bytes per register
        assert_eq!(mtimecmp2.get_ptr() as usize, 0x0200_4000 + 2 * 8);

        // Check that the mtimecmpX functions are equivalent to the mtimer.mtimecmp(X) function.
        let mtimecmp0 = CLINT::mtimecmp0();
        let mtimecmp1 = CLINT::mtimecmp1();
        let mtimecmp2 = CLINT::mtimecmp2();

        assert_eq!(
            mtimecmp0.get_ptr() as usize,
            mtimer.mtimecmp(HartId::H0).get_ptr() as usize
        );
        assert_eq!(
            mtimecmp1.get_ptr() as usize,
            mtimer.mtimecmp(HartId::H1).get_ptr() as usize
        );
        assert_eq!(
            mtimecmp2.get_ptr() as usize,
            mtimer.mtimecmp(HartId::H2).get_ptr() as usize
        );
    }
}
