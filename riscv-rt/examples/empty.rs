#![no_std]
#![no_main]

extern crate panic_halt;

use riscv_rt::{core_interrupt, entry, exception, external_interrupt};

use riscv::{
    interrupt::{Exception, Interrupt},
    result::*,
};

/// Just a dummy type to test the `ExternalInterrupt` trait.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ExternalInterrupt {
    GPIO,
    UART,
}

unsafe impl riscv::InterruptNumber for ExternalInterrupt {
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

#[entry]
fn main() -> ! {
    // do something here
    loop {}
}

/* EXAMPLES OF USING THE core_interrupt MACRO FOR CORE INTERRUPT HANDLERS.
IF v-trap ENABLED, THE MACRO ALSO DEFINES _start_COREINTERRUPT_trap routines */

/// Handler with the simplest signature.
#[core_interrupt(Interrupt::SupervisorSoft)]
fn supervisor_soft() {
    // do something here
    loop {}
}

/// Handler with the most complete signature.
#[core_interrupt(Interrupt::SupervisorTimer)]
unsafe fn supervisor_timer() -> ! {
    // do something here
    loop {}
}

/* EXAMPLES OF USING THE external_interrupt MACRO FOR EXTERNAL INTERRUPT HANDLERS. */

/// Handler with the simplest signature.
#[external_interrupt(ExternalInterrupt::GPIO)]
fn external_gpio() {
    // do something here
    loop {}
}

/// Handler with the most complete signature.
#[external_interrupt(ExternalInterrupt::UART)]
unsafe fn external_uart() -> ! {
    // do something here
    loop {}
}

/* EXAMPLES OF USING THE exception MACRO FOR EXCEPTION HANDLERS. */

/// Handler with the simplest signature.
#[exception(Exception::InstructionMisaligned)]
fn instruction_misaligned() {
    // do something here
    loop {}
}

/// Handler with the most complete signature.
#[exception(Exception::IllegalInstruction)]
unsafe fn illegal_instruction(_trap: &riscv_rt::TrapFrame) -> ! {
    // do something here
    loop {}
}

// The reference to TrapFrame can be mutable if the handler needs to modify it.
#[exception(Exception::Breakpoint)]
unsafe fn breakpoint(_trap: &mut riscv_rt::TrapFrame) -> ! {
    // do something here
    loop {}
}
