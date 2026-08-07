//! tcontrol register — Trigger Control (0x7a5)
//!
//! Global control for machine-mode triggers (Sdtrig). Exposed as a raw `usize`
//! so callers can save and restore the full register value.

read_write_csr_as_usize!(0x7a5);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::result::Error;

    #[test]
    fn test_tcontrol_read_write() {
        for i in 0..usize::BITS {
            let val = 1usize << i;
            assert_eq!(unsafe { try_write(val) }, Err(Error::Unimplemented));
            assert_eq!(try_read(), Err(Error::Unimplemented));
        }
    }
}
