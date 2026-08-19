//! tdata1 register — Trigger Data 1 (0x7a1)
//!
//! First data register of the trigger selected by [`tselect`](crate::register::tselect).
//! Layout of the lower bits depends on the trigger type.

read_write_csr! {
    /// Trigger Data 1 Register
    Tdata1: 0x7a1,
    mask: usize::MAX,
}

csr_field_enum! {
    /// Trigger type encoded in `tdata1.type` (Sdtrig).
    TriggerType {
        default: None,
        /// No trigger at this `tselect`.
        None = 0,
        /// Legacy SiFive address match trigger.
        Legacy = 1,
        /// Address/data match trigger (`mcontrol`).
        Mcontrol = 2,
        /// Instruction count trigger (`icount`).
        Icount = 3,
        /// Interrupt trigger (`itrigger`).
        Itrigger = 4,
        /// Exception trigger (`etrigger`).
        Etrigger = 5,
        /// Address/data match trigger (`mcontrol6`).
        Mcontrol6 = 6,
        /// Trigger source external to the trigger module (`tmexttrigger`).
        Tmexttrigger = 7,
        /// Custom trigger type 12.
        Custom12 = 12,
        /// Custom trigger type 13.
        Custom13 = 13,
        /// Custom trigger type 14.
        Custom14 = 14,
        /// Trigger disabled.
        Disabled = 15,
    }
}

#[cfg(target_arch = "riscv32")]
read_write_csr_field! {
    Tdata1,
    /// Trigger type (spec field `type`, bits XLEN-1:XLEN-4).
    trigger_type,
    TriggerType: [28:31],
}

#[cfg(not(target_arch = "riscv32"))]
read_write_csr_field! {
    Tdata1,
    /// Trigger type (spec field `type`, bits XLEN-1:XLEN-4).
    trigger_type,
    TriggerType: [60:63],
}

#[cfg(target_arch = "riscv32")]
read_write_csr_field! {
    Tdata1,
    /// Debug-mode-only write enable (bit XLEN-5).
    dmode: 27,
}

#[cfg(not(target_arch = "riscv32"))]
read_write_csr_field! {
    Tdata1,
    /// Debug-mode-only write enable (bit XLEN-5).
    dmode: 59,
}

#[cfg(target_arch = "riscv32")]
read_write_csr_field! {
    Tdata1,
    /// Trigger-specific data (bits XLEN-6:0).
    data: [0:26],
}

#[cfg(not(target_arch = "riscv32"))]
read_write_csr_field! {
    Tdata1,
    /// Trigger-specific data (bits XLEN-6:0).
    data: [0:58],
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::result::Error;

    #[test]
    fn test_tdata1() {
        let mut tdata1 = Tdata1::from_bits(0);

        test_csr_field!(tdata1, dmode);
        test_csr_field!(
            tdata1,
            data: [Tdata1::DATA_SHIFT, Tdata1::DATA_SHIFT + Tdata1::DATA_WIDTH - 1],
            0
        );

        [
            TriggerType::None,
            TriggerType::Legacy,
            TriggerType::Mcontrol,
            TriggerType::Icount,
            TriggerType::Itrigger,
            TriggerType::Etrigger,
            TriggerType::Mcontrol6,
            TriggerType::Tmexttrigger,
            TriggerType::Custom12,
            TriggerType::Custom13,
            TriggerType::Custom14,
            TriggerType::Disabled,
        ]
        .into_iter()
        .for_each(|variant| {
            test_csr_field!(tdata1, trigger_type: variant);
        });

        tdata1 = Tdata1::from_bits(8 << Tdata1::TRIGGER_TYPE_SHIFT);
        assert_eq!(tdata1.try_trigger_type(), Err(Error::InvalidVariant(8)));
    }
}
