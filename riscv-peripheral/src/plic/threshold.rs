//! Priority threshold register.

use crate::{common::unsafe_peripheral, plic::PriorityNumber};

unsafe_peripheral!(THRESHOLD, u32, RW);

impl THRESHOLD {
    /// Returns the priority threshold level.
    #[inline]
    pub fn get_threshold<P: PriorityNumber>(self) -> P {
        P::from_number(self.register.read() as _).unwrap()
    }

    /// Sets the priority threshold level.
    ///
    /// # Safety
    ///
    /// Changing the priority threshold can break priority-based critical sections.
    #[inline]
    pub unsafe fn set_threshold<P: PriorityNumber>(self, threshold: P) {
        self.register.write(threshold.number() as _)
    }

    /// Resets the priority threshold level to 0.
    ///
    /// # Note
    ///
    /// Threshold 0 implies that all interrupts are accepted.
    /// Thus, resetting the threshold is equivalent to accepting interrupts from any enabled interrupt source.
    #[inline]
    pub fn reset(self) {
        self.register.write(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::Priority;

    #[test]
    fn test_threshold() {
        let mut raw_reg = 0u32;
        // SAFETY: valid memory address
        let threshold = unsafe { THRESHOLD::new(&mut raw_reg as *mut _ as _) };

        for i in 0..=Priority::MAX_PRIORITY_NUMBER {
            let priority = Priority::from_number(i).unwrap();
            unsafe { threshold.set_threshold(priority) };
            assert_eq!(threshold.get_threshold::<Priority>(), priority);
        }
        threshold.reset();
        assert_eq!(threshold.get_threshold::<Priority>(), Priority::P0);
    }
}
