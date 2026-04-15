#![no_std]
#![no_main]

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[riscv::pac_enum(unsafe ExternalInterruptNumber)]
pub enum ExternalInterrupt {
    Gpio = 0,
    Uart = 1,
    I2c = 2,
}

#[cfg(not(feature = "custom-interrupts"))]
pub use riscv::interrupt::Interrupt as CoreInterrupt;

#[cfg(feature = "custom-interrupts")]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[riscv::pac_enum(unsafe CoreInterruptNumber)]
pub enum CoreInterrupt {
    MachineSoft = 3,
    MachineTimer = 7,
    MachineExternal = 11,
}

#[cfg(not(feature = "custom-exceptions"))]
pub use riscv::interrupt::Exception;

#[cfg(feature = "custom-exceptions")]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[riscv::pac_enum(unsafe ExceptionNumber)]
pub enum Exception {
    InstructionMisaligned = 0,
    InstructionFault = 1,
    IllegalInstruction = 2,
    Breakpoint = 3,
    LoadMisaligned = 4,
    LoadFault = 5,
    StoreMisaligned = 6,
    StoreFault = 7,
    MachineEnvCall = 11,
    InstructionPageFault = 12,
    LoadPageFault = 13,
    StorePageFault = 15,
}

#[cfg(feature = "pre-init")]
core::arch::global_asm!(
    "
.section .init.pre_init, \"ax\"
.global __pre_init
__pre_init:
    // Do some pre-initialization work here and return
    ret"
);
