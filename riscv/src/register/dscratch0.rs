//! dscratch0

read_write_csr_as_usize!(0x7b2);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dscratch0_read_write() {
        for i in 0..usize::BITS {
            let val = 1usize << i;
            let _ = unsafe { try_write(val) };
            let _ = try_read();
        }
    }
}
