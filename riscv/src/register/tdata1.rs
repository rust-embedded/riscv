//! tdata1 register — Trigger Data 1 (0x7a1)
//!
//! First data register of the trigger selected by [`tselect`](crate::register::tselect). Layout of the
//! lower bits depends on the trigger type; this module exposes the register as
//! a raw `usize` for save/restore and discovery.

read_write_csr_as_usize!(0x7a1);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::result::Error;

    #[test]
    fn test_tdata1_read_write() {
        for i in 0..usize::BITS {
            let val = 1usize << i;
            assert_eq!(unsafe { try_write(val) }, Err(Error::Unimplemented));
            assert_eq!(try_read(), Err(Error::Unimplemented));
        }
    }
}
