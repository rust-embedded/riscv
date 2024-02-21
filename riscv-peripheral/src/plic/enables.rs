//! Interrupt enables register of a PLIC context.

use crate::common::{Reg, RW};
use riscv_pac::ExternalInterruptNumber;

/// Enables register of a PLIC context.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct ENABLES {
    ptr: *mut u32,
}

impl ENABLES {
    /// Creates a new Interrupts enables register from a base address.
    ///
    /// # Safety
    ///
    /// The base address must point to a valid Interrupts enables register.
    #[inline]
    pub(crate) const unsafe fn new(address: usize) -> Self {
        Self { ptr: address as _ }
    }

    #[cfg(test)]
    #[inline]
    pub(crate) fn address(self) -> usize {
        self.ptr as _
    }

    /// Checks if an interrupt source is enabled for the PLIC context.
    #[inline]
    pub fn is_enabled<I: ExternalInterruptNumber>(self, source: I) -> bool {
        let source = source.number() as usize;
        let offset = (source / u32::BITS as usize) as _;
        // SAFETY: valid interrupt number
        let reg: Reg<u32, RW> = unsafe { Reg::new(self.ptr.offset(offset)) };
        reg.read_bit(source % u32::BITS as usize)
    }

    /// Enables an interrupt source for the PLIC context.
    ///
    /// # Note
    ///
    /// It performs non-atomic read-modify-write operations, which may lead to **wrong** behavior.
    ///
    /// # Safety
    ///
    /// * Enabling an interrupt source can break mask-based critical sections.
    #[inline]
    pub unsafe fn enable<I: ExternalInterruptNumber>(self, source: I) {
        let source = source.number() as usize;
        let offset = (source / u32::BITS as usize) as _;
        // SAFETY: valid interrupt number
        let reg: Reg<u32, RW> = unsafe { Reg::new(self.ptr.offset(offset)) };
        reg.set_bit(source % u32::BITS as usize);
    }

    #[cfg(target_has_atomic = "32")]
    /// Enables an interrupt source for the PLIC context atomically.
    ///
    /// # Note
    ///
    /// This method is only available on targets that support atomic operations on 32-bit registers.
    ///
    /// # Safety
    ///
    /// * Enabling an interrupt source can break mask-based critical sections.
    /// * Register must be properly aligned **for atomic operations**.
    /// * The register must not be accessed through non-atomic operations until this function returns.
    #[inline]
    pub unsafe fn atomic_enable<I: ExternalInterruptNumber>(
        self,
        source: I,
        order: core::sync::atomic::Ordering,
    ) {
        let source = source.number() as usize;
        let offset = (source / u32::BITS as usize) as _;
        // SAFETY: valid interrupt number
        let reg: Reg<u32, RW> = unsafe { Reg::new(self.ptr.offset(offset)) };
        reg.atomic_set_bit(source % u32::BITS as usize, order);
    }

    /// Disables an interrupt source for the PLIC context.
    ///
    /// # Note
    ///
    /// It performs non-atomic read-modify-write operations, which may lead to **wrong** behavior.
    #[inline]
    pub fn disable<I: ExternalInterruptNumber>(self, source: I) {
        let source = source.number() as usize;
        let offset = (source / u32::BITS as usize) as _;
        // SAFETY: valid interrupt number
        let reg: Reg<u32, RW> = unsafe { Reg::new(self.ptr.offset(offset)) };
        reg.clear_bit(source % u32::BITS as usize);
    }

    #[cfg(target_has_atomic = "32")]
    /// Disables an interrupt source for the PLIC context atomically.
    ///
    /// # Note
    ///
    /// This method is only available on targets that support atomic operations on 32-bit registers.
    ///
    /// # Safety
    ///
    /// * Register must be properly aligned **for atomic operations**.
    /// * The register must not be accessed through non-atomic operations until this function returns.
    #[inline]
    pub unsafe fn atomic_disable<I: ExternalInterruptNumber>(
        self,
        source: I,
        order: core::sync::atomic::Ordering,
    ) {
        let source = source.number() as usize;
        let offset = (source / u32::BITS as usize) as _;
        // SAFETY: valid interrupt number
        let reg: Reg<u32, RW> = unsafe { Reg::new(self.ptr.offset(offset)) };
        reg.atomic_clear_bit(source % u32::BITS as usize, order);
    }

    /// Enables all the external interrupt sources for the PLIC context.
    ///
    /// # Safety
    ///
    ///* Enabling all interrupt sources can break mask-based critical sections.
    #[inline]
    pub unsafe fn enable_all<I: ExternalInterruptNumber>(self) {
        for offset in 0..=(I::MAX_INTERRUPT_NUMBER as u32 / u32::BITS) as isize {
            // SAFETY: valid offset
            let reg: Reg<u32, RW> = unsafe { Reg::new(self.ptr.offset(offset)) };
            reg.write(0xFFFF_FFFF);
        }
    }

    /// Disables all the external interrupt sources for the PLIC context.
    #[inline]
    pub fn disable_all<I: ExternalInterruptNumber>(self) {
        for offset in 0..=(I::MAX_INTERRUPT_NUMBER as u32 / u32::BITS) as _ {
            // SAFETY: valid offset
            let reg: Reg<u32, RW> = unsafe { Reg::new(self.ptr.offset(offset)) };
            reg.write(0);
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::test::Interrupt;
    use super::*;

    #[test]
    fn test_enables() {
        // slice to emulate the interrupt enables register
        let mut raw_reg = [0u32; 32];
        // SAFETY: valid memory address
        let enables = unsafe { ENABLES::new(raw_reg.as_mut_ptr() as _) };

        for i in 0..255 {
            if i & 0x2 != 0 {
                unsafe { enables.enable(Interrupt::I1) };
            } else {
                enables.disable(Interrupt::I1);
            }
            if i & 0x4 != 0 {
                unsafe { enables.enable(Interrupt::I2) };
            } else {
                enables.disable(Interrupt::I2);
            }
            if i & 0x8 != 0 {
                unsafe { enables.enable(Interrupt::I3) };
            } else {
                enables.disable(Interrupt::I3);
            }
            if i & 0x10 != 0 {
                unsafe { enables.enable(Interrupt::I4) };
            } else {
                enables.disable(Interrupt::I4);
            }

            assert_eq!(enables.is_enabled(Interrupt::I1), i & 0x2 != 0);
            assert_eq!(enables.is_enabled(Interrupt::I2), i & 0x4 != 0);
            assert_eq!(enables.is_enabled(Interrupt::I3), i & 0x8 != 0);
            assert_eq!(enables.is_enabled(Interrupt::I4), i & 0x10 != 0);

            enables.disable_all::<Interrupt>();
            assert!(!enables.is_enabled(Interrupt::I1));
            assert!(!enables.is_enabled(Interrupt::I2));
            assert!(!enables.is_enabled(Interrupt::I3));
            assert!(!enables.is_enabled(Interrupt::I4));

            unsafe { enables.enable_all::<Interrupt>() };
            assert!(enables.is_enabled(Interrupt::I1));
            assert!(enables.is_enabled(Interrupt::I2));
            assert!(enables.is_enabled(Interrupt::I3));
            assert!(enables.is_enabled(Interrupt::I4));
        }
    }

    #[cfg(target_has_atomic = "32")]
    #[test]
    fn test_atomic_enables() {
        // slice to emulate the interrupt enables register
        use core::sync::atomic::Ordering;
        let mut raw_reg = [0u32; 32];
        // SAFETY: valid memory address
        let enables = unsafe { ENABLES::new(raw_reg.as_mut_ptr() as _) };

        for i in 0..255 {
            if i & 0x2 != 0 {
                unsafe { enables.atomic_enable(Interrupt::I1, Ordering::Relaxed) };
            } else {
                unsafe { enables.atomic_disable(Interrupt::I1, Ordering::Relaxed) };
            }
            if i & 0x4 != 0 {
                unsafe { enables.atomic_enable(Interrupt::I2, Ordering::Relaxed) };
            } else {
                unsafe { enables.atomic_disable(Interrupt::I2, Ordering::Relaxed) };
            }
            if i & 0x8 != 0 {
                unsafe { enables.atomic_enable(Interrupt::I3, Ordering::Relaxed) };
            } else {
                unsafe { enables.atomic_disable(Interrupt::I3, Ordering::Relaxed) };
            }
            if i & 0x10 != 0 {
                unsafe { enables.atomic_enable(Interrupt::I4, Ordering::Relaxed) };
            } else {
                unsafe { enables.atomic_disable(Interrupt::I4, Ordering::Relaxed) };
            }

            assert_eq!(enables.is_enabled(Interrupt::I1), i & 0x2 != 0);
            assert_eq!(enables.is_enabled(Interrupt::I2), i & 0x4 != 0);
            assert_eq!(enables.is_enabled(Interrupt::I3), i & 0x8 != 0);
            assert_eq!(enables.is_enabled(Interrupt::I4), i & 0x10 != 0);
        }
    }
}
