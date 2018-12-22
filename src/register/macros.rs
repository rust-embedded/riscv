macro_rules! read_csr {
    ($csr_number:expr) => {
        /// Reads the CSR
        #[inline]
        #[cfg(riscv)]
        unsafe fn _read() -> usize {
            let r: usize;
            asm!("csrrs $0, $1, x0" : "=r"(r) : "i"($csr_number) :: "volatile");
            r
        }

        #[inline]
        #[cfg(not(riscv))]
        unsafe fn _read() -> usize {
            unimplemented!()
        }
    };
}

macro_rules! read_csr_rv32 {
    ($csr_number:expr) => {
        /// Reads the CSR
        #[inline]
        #[cfg(riscv32)]
        unsafe fn _read() -> usize {
            let r: usize;
            asm!("csrrs $0, $1, x0" : "=r"(r) : "i"($csr_number) :: "volatile");
            r
        }

        #[inline]
        #[cfg(not(riscv32))]
        unsafe fn _read() -> usize {
            unimplemented!()
        }
    };
}

macro_rules! read_csr_as {
    ($register:ident, $csr_number:expr) => {
        read_csr!($csr_number);

        /// Reads the CSR
        #[inline]
        pub fn read() -> $register {
            $register { bits: unsafe{ _read() } }
        }
    };
}

macro_rules! read_csr_as_usize {
    ($csr_number:expr) => {
        read_csr!($csr_number);

        /// Reads the CSR
        #[inline]
        pub fn read() -> usize {
            unsafe{ _read() }
        }
    };
}

macro_rules! read_csr_as_usize_rv32 {
    ($csr_number:expr) => {
        read_csr_rv32!($csr_number);

        /// Reads the CSR
        #[inline]
        pub fn read() -> usize {
            unsafe{ _read() }
        }
    };
}

macro_rules! write_csr {
    ($csr_number:expr) => {
        /// Writes the CSR
        #[inline]
        #[cfg(riscv)]
        unsafe fn _write(bits: usize) {
            asm!("csrrw x0, $1, $0" :: "r"(bits), "i"($csr_number) :: "volatile");
        }

        #[inline]
        #[cfg(not(riscv))]
        unsafe fn _write(_bits: usize) {
            unimplemented!()
        }
    };
}

macro_rules! write_csr_as_usize {
    ($csr_number:expr) => {
        write_csr!($csr_number);

        /// Writes the CSR
        #[inline]
        pub fn write(bits: usize) {
            unsafe{ _write(bits) }
        }
    };
}

macro_rules! set {
    ($csr_number:expr) => {
        /// Set the CSR
        #[inline]
        #[cfg(riscv)]
        unsafe fn _set(bits: usize) {
            asm!("csrrs x0, $1, $0" :: "r"(bits), "i"($csr_number) :: "volatile");
        }

        #[inline]
        #[cfg(not(riscv))]
        unsafe fn _set(_bits: usize) {
            unimplemented!()
        }
    };
}

macro_rules! clear {
    ($csr_number:expr) => {
        /// Clear the CSR
        #[inline]
        #[cfg(riscv)]
        unsafe fn _clear(bits: usize) {
            asm!("csrrc x0, $1, $0" :: "r"(bits), "i"($csr_number) :: "volatile");
        }

        #[inline]
        #[cfg(not(riscv))]
        unsafe fn _clear(_bits: usize) {
            unimplemented!()
        }
    };
}

macro_rules! set_csr {
    ($set_field:ident, $e:expr) => {
        #[inline]
        pub unsafe fn $set_field() {
            _set($e);
        }
    }
}

macro_rules! clear_csr {
    ($clear_field:ident, $e:expr) => {
        #[inline]
        pub unsafe fn $clear_field() {
            _clear($e);
        }
    }
}

macro_rules! set_clear_csr {
    ($set_field:ident, $clear_field:ident, $e:expr) => {
        set_csr!($set_field, $e);
        clear_csr!($clear_field, $e);
    }
}