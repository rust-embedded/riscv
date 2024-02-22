#![no_std]

#[cfg(feature = "riscv-pac-macros")]
pub use riscv_pac_macros::*;

/// Trait for enums of target-specific exception numbers.
///
/// This trait should be implemented by a peripheral access crate (PAC) on its enum of available
/// exceptions for a specific device. Alternatively, the `riscv` crate provides a default
/// implementation for the RISC-V ISA. Each variant must convert to a `u16` of its exception number.
///
/// # Safety
///
/// * This trait must only be implemented on the `riscv` crate or on a PAC of a RISC-V target.
/// * This trait must only be implemented on enums of exceptions.
/// * Each enum variant must represent a distinct value (no duplicates are permitted),
/// * Each enum variant must always return the same value (do not change at runtime).
/// * All the exception numbers must be less than or equal to `MAX_EXCEPTION_NUMBER`.
/// * `MAX_EXCEPTION_NUMBER` must coincide with the highest allowed exception number.
pub unsafe trait ExceptionNumber: Copy {
    /// Highest number assigned to an exception.
    const MAX_EXCEPTION_NUMBER: u16;

    /// Converts an exception to its corresponding number.
    fn number(self) -> u16;

    /// Tries to convert a number to a valid exception.
    /// If the conversion fails, it returns an error with the number back.
    fn from_number(value: u16) -> Result<Self, u16>;
}

/// Trait for enums of target-specific interrupt numbers.
///
/// This trait should be implemented by a peripheral access crate (PAC) on its enum of available
/// interrupts for a specific device. Alternatively, the `riscv` crate provides a default
/// implementation for the RISC-V ISA. Each variant must convert to a `u16` of its interrupt number.
///
/// # Safety
///
/// * This trait must only be implemented on the `riscv` crate or on a PAC of a RISC-V target.
/// * This trait must only be implemented on enums of interrupts.
/// * Each enum variant must represent a distinct value (no duplicates are permitted),
/// * Each enum variant must always return the same value (do not change at runtime).
/// * All the interrupt numbers must be less than or equal to `MAX_INTERRUPT_NUMBER`.
/// * `MAX_INTERRUPT_NUMBER` must coincide with the highest allowed interrupt number.
pub unsafe trait InterruptNumber: Copy {
    /// Highest number assigned to an interrupt source.
    const MAX_INTERRUPT_NUMBER: u16;

    /// Converts an interrupt source to its corresponding number.
    fn number(self) -> u16;

    /// Tries to convert a number to a valid interrupt source.
    /// If the conversion fails, it returns an error with the number back.
    fn from_number(value: u16) -> Result<Self, u16>;
}

/// Marker trait for enums of target-specific core interrupt numbers.
///
/// Core interrupts are interrupts are retrieved from the `mcause` CSR. Usually, vectored mode is
/// only available for core interrupts. The `riscv` crate provides a default implementation for
/// the RISC-V ISA. However, a PAC may override the default implementation if the target has a
/// different interrupt numbering scheme (e.g., ESP32C3).
///
/// # Safety
///
/// Each enum variant must represent a valid core interrupt number read from the `mcause` CSR.
pub unsafe trait CoreInterruptNumber: InterruptNumber {}

/// Marker trait for enums of target-specific external interrupt numbers.
///
/// External interrupts are interrupts caused by external sources (e.g., GPIO, UART, SPI).
/// External interrupts are **not** retrieved from the `mcause` CSR.
/// Instead, RISC-V processors have a single core interrupt for all external interrupts.
/// An additional peripheral (e.g., PLIC) is used to multiplex the external interrupts.
///
/// # Safety
///
/// Each enum variant must represent a valid external interrupt number.
pub unsafe trait ExternalInterruptNumber: InterruptNumber {}

/// Trait for enums of priority levels.
///
/// This trait should be implemented by a peripheral access crate (PAC) on its enum of available
/// priority numbers for a specific device. Each variant must convert to a `u8` of its priority level.
///
/// # Safety
///
/// * This trait must only be implemented on a PAC of a RISC-V target.
/// * This trait must only be implemented on enums of priority levels.
/// * Each enum variant must represent a distinct value (no duplicates are permitted).
/// * Each enum variant must always return the same value (do not change at runtime).
/// * All the priority level numbers must be less than or equal to `MAX_PRIORITY_NUMBER`.
/// * `MAX_PRIORITY_NUMBER` must coincide with the highest allowed priority number.
pub unsafe trait PriorityNumber: Copy {
    /// Number assigned to the highest priority level.
    const MAX_PRIORITY_NUMBER: u8;

    /// Converts a priority level to its corresponding number.
    fn number(self) -> u8;

    /// Tries to convert a number to a valid priority level.
    /// If the conversion fails, it returns an error with the number back.
    fn from_number(value: u8) -> Result<Self, u8>;
}

/// Trait for enums of HART identifiers.
///
/// This trait should be implemented by a peripheral access crate (PAC) on its enum of available
/// HARTs for a specific device. Each variant must convert to a `u16` of its HART ID number.
///
/// # Safety
///
/// * This trait must only be implemented on a PAC of a RISC-V target.
/// * This trait must only be implemented on enums of HART IDs.
/// * Each enum variant must represent a distinct value (no duplicates are permitted),
/// * Each anum variant must always return the same value (do not change at runtime).
/// * All the HART ID numbers must be less than or equal to `MAX_HART_ID_NUMBER`.
/// * `MAX_HART_ID_NUMBER` must coincide with the highest allowed HART ID number.
pub unsafe trait HartIdNumber: Copy {
    /// Highest number assigned to a context.
    const MAX_HART_ID_NUMBER: u16;

    /// Converts a HART ID to its corresponding number.
    fn number(self) -> u16;

    /// Tries to convert a number to a valid HART ID.
    /// If the conversion fails, it returns an error with the number back.
    fn from_number(value: u16) -> Result<Self, u16>;
}
