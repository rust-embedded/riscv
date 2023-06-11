//! Interrupt Priority register.

use super::{InterruptNumber, PriorityNumber, PRIORITIES};
use crate::peripheral::common::{unsafe_peripheral, RW};

impl PRIORITIES {
    /// Creates a new Interrupts priorities register from a base address.
    ///
    /// # Safety
    ///
    /// The base address must point to a valid Interrupts priorities register.
    #[inline(always)]
    pub unsafe fn new(address: usize) -> Self {
        Self {
            priority0: PRIORITY::new(address),
        }
    }

    /// Returns the priority register assigned to a given interrupt source.
    #[inline(always)]
    pub fn priority<I: InterruptNumber>(&self, source: I) -> PRIORITY {
        // SAFETY: valid interrupt number
        unsafe { PRIORITY::new(self.priority0.get_ptr().offset(source.number() as _) as _) }
    }
}

unsafe_peripheral!(PRIORITY, u32, RW);

impl PRIORITY {
    /// Returns the priority level associated to the interrupt source.
    #[inline(always)]
    pub fn get_priority<P: PriorityNumber>(self) -> P {
        P::try_from(self.register.read() as _).unwrap()
    }

    /// Sets the priority level of a given interrupt source.
    ///
    /// # Note
    ///
    /// Interrupt source priorities are shared among all the contexts of the PLIC.
    /// Thus, changing the priority of sources  may affect other PLIC contexts.
    ///
    /// # Safety
    ///
    /// Changing priority levels can break priority-based critical sections and compromise memory safety.
    #[inline(always)]
    pub unsafe fn set_priority<P: PriorityNumber>(self, priority: P) {
        self.register.write(priority.number() as _);
    }
}
