//! Interrupts Priorities register.

use crate::common::{Reg, RW};
use riscv::{ExternalInterruptNumber, PriorityNumber};

/// Interrupts priorities register.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct PRIORITIES {
    ptr: *mut u32,
}

impl PRIORITIES {
    /// Creates a new Interrupts priorities register from a base address.
    ///
    /// # Safety
    ///
    /// The base address must point to a valid Interrupts priorities register.
    #[inline]
    pub(crate) const unsafe fn new(address: usize) -> Self {
        Self { ptr: address as _ }
    }

    #[cfg(test)]
    #[inline]
    pub(crate) fn address(self) -> usize {
        self.ptr as _
    }

    /// Returns the priority assigned to a given interrupt source.
    #[inline]
    pub fn get_priority<I: ExternalInterruptNumber, P: PriorityNumber>(self, source: I) -> P {
        // SAFETY: valid interrupt number
        let reg: Reg<u32, RW> = unsafe { Reg::new(self.ptr.add(source.number())) };
        P::from_number(reg.read() as _).unwrap()
    }

    /// Sets the priority level of a given interrupt source.
    ///
    /// # Safety
    ///
    /// Changing the priority level can break priority-based critical sections.
    #[inline]
    pub unsafe fn set_priority<I: ExternalInterruptNumber, P: PriorityNumber>(
        self,
        source: I,
        priority: P,
    ) {
        // SAFETY: valid interrupt number
        let reg: Reg<u32, RW> = unsafe { Reg::new(self.ptr.add(source.number())) };
        reg.write(priority.number() as _);
    }

    /// Resets all the priority levels of all the external interrupt sources to 0.
    ///
    /// # Note
    ///
    /// Priority level 0 is reserved for "no interrupt".
    /// Thus, this method effectively disables the all the external interrupts.
    #[inline]
    pub fn reset<I: ExternalInterruptNumber>(self) {
        for source in 0..=I::MAX_INTERRUPT_NUMBER as _ {
            // SAFETY: interrupt number within range
            let reg: Reg<u32, RW> = unsafe { Reg::new(self.ptr.offset(source)) };
            reg.write(0);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::{Interrupt, Priority};
    use riscv::InterruptNumber;

    #[test]
    fn test_priorities() {
        // slice to emulate the interrupt priorities register
        let mut raw_reg = [0u32; 1024];
        // SAFETY: valid memory address
        let priorities = unsafe { PRIORITIES::new(raw_reg.as_mut_ptr() as _) };

        for i in 1..=Interrupt::MAX_INTERRUPT_NUMBER {
            let source = Interrupt::from_number(i).unwrap();
            for j in 0..=Priority::MAX_PRIORITY_NUMBER {
                let priority = Priority::from_number(j).unwrap();
                unsafe { priorities.set_priority(source, priority) };
                assert_eq!(priorities.get_priority::<_, Priority>(source), priority);
            }
        }
        priorities.reset::<Interrupt>();
        for i in 1..=Interrupt::MAX_INTERRUPT_NUMBER {
            let source = Interrupt::from_number(i).unwrap();
            assert_eq!(priorities.get_priority::<_, Priority>(source), Priority::P0);
        }
    }
}
