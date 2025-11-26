//! Interrupt claim/complete register

use crate::common::unsafe_peripheral;
use riscv::ExternalInterruptNumber;

unsafe_peripheral!(CLAIM, u32, RW);

impl CLAIM {
    /// Claims the number of a pending interrupt for for the PLIC context.
    /// If no interrupt is pending for this context, it returns [`None`].
    #[inline]
    pub fn claim<I: ExternalInterruptNumber>(self) -> Option<I> {
        match self.register.read() {
            0 => None,
            i => Some(I::from_number(i as _).unwrap()),
        }
    }

    /// Marks a pending interrupt as complete for the PLIC context.
    ///
    /// # Note
    ///
    /// If the source ID does not match an interrupt source that is
    /// currently enabled for the target, the completion is silently ignored.
    #[inline]
    pub fn complete<I: ExternalInterruptNumber>(self, source: I) {
        self.register.write(source.number() as _)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::Interrupt;
    use riscv_pac::InterruptNumber;

    #[test]
    fn test_claim() {
        let mut raw_reg = 0u32;
        // SAFETY: valid memory address
        let claim = unsafe { CLAIM::new(&mut raw_reg as *mut _ as _) };

        assert_eq!(claim.claim::<Interrupt>(), None);

        for i in 1..=Interrupt::MAX_INTERRUPT_NUMBER {
            let interrupt = Interrupt::from_number(i).unwrap();
            claim.complete(interrupt);
            assert_eq!(claim.claim(), Some(interrupt));
        }
    }
}
