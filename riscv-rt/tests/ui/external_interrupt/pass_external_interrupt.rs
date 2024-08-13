use riscv_rt::result::{Error, Result};

/// Just a dummy type to test the `ExternalInterrupt` trait.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ExternalInterrupt {
    GPIO,
    UART,
}
unsafe impl riscv_rt::InterruptNumber for ExternalInterrupt {
    const MAX_INTERRUPT_NUMBER: usize = 1;

    #[inline]
    fn number(self) -> usize {
        self as usize
    }

    #[inline]
    fn from_number(value: usize) -> Result<Self> {
        match value {
            0 => Ok(Self::GPIO),
            1 => Ok(Self::UART),
            _ => Err(Error::InvalidVariant(value)),
        }
    }
}
unsafe impl riscv::ExternalInterruptNumber for ExternalInterrupt {}

#[riscv_rt::external_interrupt(ExternalInterrupt::GPIO)]
fn simple_interrupt() {}

#[riscv_rt::external_interrupt(ExternalInterrupt::UART)]
unsafe fn no_return_interrupt() -> ! {
    loop {}
}

fn main() {}
