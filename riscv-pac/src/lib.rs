#![no_std]

/// Trait for enums of target-specific external interrupt numbers.
///
/// This trait should be implemented by a peripheral access crate (PAC)
/// on its enum of available external interrupts for a specific device.
/// Each variant must convert to a `u16` of its interrupt number.
///
/// # Safety
///
/// * This trait must only be implemented on a PAC of a RISC-V target.
/// * This trait must only be implemented on enums of external interrupts.
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

/// Trait for enums of priority levels.
///
/// This trait should be implemented by a peripheral access crate (PAC)
/// on its enum of available priority numbers for a specific device.
/// Each variant must convert to a `u8` of its priority level.
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
/// This trait should be implemented by a peripheral access crate (PAC)
/// on its enum of available HARTs for a specific device.
/// Each variant must convert to a `u16` of its HART ID number.
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
