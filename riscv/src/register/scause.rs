//! scause register

pub use crate::interrupt::Trap;
pub use riscv_pac::{CoreInterruptNumber, ExceptionNumber, InterruptNumber}; // re-export useful riscv-pac traits

read_write_csr! {
    /// scause register
    Scause: 0x142,
    mask: usize::MAX,
}

#[cfg(target_arch = "riscv32")]
read_write_csr_field! {
    Scause,
    /// Returns the type of the trap:
    ///
    /// - `true`: an interrupt caused the trap
    /// - `false`: an exception caused the trap
    interrupt: 31,
}

#[cfg(not(target_arch = "riscv32"))]
read_write_csr_field! {
    Scause,
    /// Returns the type of the trap:
    ///
    /// - `true`: an interrupt caused the trap
    /// - `false`: an exception caused the trap
    interrupt: 63,
}

#[cfg(target_arch = "riscv32")]
read_write_csr_field! {
    Scause,
    /// Returns the code field
    code: [0:30],
}

#[cfg(not(target_arch = "riscv32"))]
read_write_csr_field! {
    Scause,
    /// Returns the code field
    code: [0:62],
}

impl Scause {
    /// Returns the trap cause represented by this register.
    ///
    /// # Note
    ///
    /// This method returns a **raw trap cause**, which means that values are represented as `usize`.
    /// To get a target-specific trap cause, use [`Trap::try_into`] with your target-specific S-Mode trap cause types.
    #[inline]
    pub fn cause(&self) -> Trap<usize, usize> {
        if self.is_interrupt() {
            Trap::Interrupt(self.code())
        } else {
            Trap::Exception(self.code())
        }
    }

    /// Is trap cause an interrupt.
    #[inline]
    pub fn is_interrupt(&self) -> bool {
        self.interrupt()
    }

    /// Is trap cause an exception.
    #[inline]
    pub fn is_exception(&self) -> bool {
        !self.interrupt()
    }
}

/// Set supervisor cause register to corresponding cause.
#[inline]
pub unsafe fn set<I: CoreInterruptNumber, E: ExceptionNumber>(cause: Trap<I, E>) {
    let bits = match cause {
        Trap::Interrupt(i) => {
            i.number() | (1 << (usize::BITS as usize - 1)) // interrupt bit is 1
        }
        Trap::Exception(e) => e.number(),
    };
    _write(bits);
}
