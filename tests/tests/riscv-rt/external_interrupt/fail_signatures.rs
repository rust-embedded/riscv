use riscv_rt::result::{Error, Result};

/// Just a dummy type to test the `ExternalInterrupt` trait.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ExternalInterrupt {
    GPIO,
    UART,
    PWM,
}
unsafe impl riscv_rt::InterruptNumber for ExternalInterrupt {
    const MAX_INTERRUPT_NUMBER: usize = 2;

    #[inline]
    fn number(self) -> usize {
        self as usize
    }

    #[inline]
    fn from_number(value: usize) -> Result<Self> {
        match value {
            0 => Ok(Self::GPIO),
            1 => Ok(Self::UART),
            2 => Ok(Self::PWM),
            _ => Err(Error::InvalidVariant(value)),
        }
    }
}
unsafe impl riscv::ExternalInterruptNumber for ExternalInterrupt {}

#[riscv_rt::external_interrupt(ExternalInterrupt::GPIO)]
fn my_interrupt() -> usize {}

#[riscv_rt::external_interrupt(ExternalInterrupt::UART)]
fn my_other_interrupt(code: usize) -> usize {}

#[riscv_rt::external_interrupt(ExternalInterrupt::PWM)]
async fn my_async_interrupt(code: usize) -> usize {}

fn main() {}
