//! Functions for accessing Control and Status Registers

#[cfg(target_arch = "riscv")]
macro_rules! csr_asm {
    ($op:ident, $csr:expr, $value:expr) => (
        {
            let res: usize;
            unsafe {
                asm!(concat!(stringify!($op), " $0, ", stringify!($csr), ", $1")
                     : "=r"(res)
                     : "r"($value)
                     :
                     : "volatile");
            }
            res
        }
    )
}


#[cfg(not(target_arch = "riscv"))]
macro_rules! csr_asm {
    ($op:ident, $csr:expr, $value:expr) => {
        0
    }
}

macro_rules! r {
    ($MOD:ident, $TYPE:ident, $CSR:expr) => (
        pub struct R {
            bits: u32,
        }

        impl super::$TYPE {
            #[inline(always)]
            pub fn read(&self) -> R {
                R { bits: csr_asm!(csrrs, $CSR, 0) as u32 }
            }
        }

        impl R {
            #[inline(always)]
            pub fn bits(&self) -> u32 {
                self.bits
            }
        }
    )
}

macro_rules! w {
    ($MOD:ident, $TYPE:ident, $CSR:expr) => (
        macro_rules! func {
            ($fnname:ident, $csrop:ident) => (
                #[inline(always)]
                pub fn $fnname<F>(&self, f: F)
                    where
                    F: FnOnce(&mut W) -> &mut W,
                {
                    let mut w = W { bits: 0 };
                    f(&mut w);
                    csr_asm!($csrop, $CSR, w.bits as usize);
                }
            )
        }

        pub struct W {
            bits: u32,
        }

        impl super::$TYPE {
            func!(write, csrrw);
            func!(set, csrrs);
            func!(clear, csrrc);
        }

        impl W {
            #[inline(always)]
            pub fn bits(&mut self, value: u32) -> &mut W {
                self.bits = value;
                self
            }
            #[inline(always)]
            pub fn set_bits(&mut self, value: u32) -> &mut W {
                self.bits |= value;
                self
            }
            #[inline(always)]
            pub fn clear_bits(&mut self, value: u32) -> &mut W {
                self.bits &= !value;
                self
            }
        }
    )
}

macro_rules! rw {
    ($MOD:ident, $TYPE:ident, $CSR:expr) => (
        r!($MOD, $TYPE, $CSR);
        w!($MOD, $TYPE, $CSR);
    )
}

macro_rules! csr {
    ($MOD:ident, $TYPE:ident, $CSR:expr, $MACRO:ident) => (
        pub struct $TYPE {}
        #[allow(non_upper_case_globals)]
        pub const $MOD: $TYPE = $TYPE {};

        pub mod $MOD {
            $MACRO!($MOD, $TYPE, $CSR);
        }
    )
}

/// User Trap Setup
csr!(ustatus, USTATUS, 0x000, rw);
csr!(uie, UIE, 0x004, rw);
csr!(utvec, UTVEC, 0x005, rw);
/// User Trap Handling
csr!(uscratch, USCRATCH, 0x040, rw);
csr!(uepc, UEPC, 0x041, rw);
csr!(ucause, UCAUSE, 0x042, rw);
csr!(utval, UTVAL, 0x043, rw);
csr!(uip, UIP, 0x044, r);
/// User Floating-Point CSRs
csr!(fflags, FFLAGS, 0x001, rw);
csr!(frm, FRM, 0x002, rw);
csr!(fcsr, FCSR, 0x003, rw);
/// User Counter/Timers
csr!(cycle, CYCLE, 0xC00, rw);
csr!(time, TIME, 0xC01, rw);
csr!(instret, INSTRET, 0xC02, rw);
// TODO: hpmcounter3 - hpmcounter31
csr!(cycleh, CYCLEH, 0xC80, rw);
csr!(timeh, TIMEH, 0xC81, rw);
csr!(instreth, INSTRETH, 0xC82, rw);
// TODO: hpmcounter3h - hpmcounter31h

/// Supervisor Trap Setup
csr!(sstatus, SSTATUS, 0x100, rw);
csr!(sedeleg, SEDELEG, 0x102, rw);
csr!(sideleg, SIDELEG, 0x103, rw);
csr!(sie, SIE, 0x104, rw);
csr!(stvec, STVEC, 0x105, rw);
csr!(scounteren, SCOUNTEREN, 0x106, rw);
/// Supervisor Trap Handling
csr!(sscratch, SSCRATCH, 0x140, rw);
csr!(sepc, SEPC, 0x141, rw);
csr!(scause, SCAUSE, 0x142, rw);
csr!(stval, STVAL, 0x143, rw);
csr!(sip, SIP, 0x144, r);
/// Supervisor Protection and Translation
csr!(satp, SATP, 0x180, rw);

/// Machine Information Registers
csr!(mvendorid, MVENDORID, 0xF11, r);
csr!(marchid, MARCHID, 0xF12, r);
csr!(mimpid, MIMPID, 0xF13, r);
csr!(mhartid, MHARTID, 0xF14, r);
/// Machine Trap Setup
csr!(mstatus, MSTATUS, 0x300, rw);
csr!(misa, MISA, 0x301, r);
csr!(medeleg, MEDELEG, 0x302, rw);
csr!(mideleg, MIDELEG, 0x303, rw);
csr!(mie, MIE, 0x304, rw);
csr!(mtvec, MTVEC, 0x305, rw);
csr!(mcounteren, MCOUNTEREN, 0x306, rw);
/// Machine Trap Handling
csr!(mscratch, MSCRATCH, 0x340, rw);
csr!(mepc, MEPC, 0x341, rw);
csr!(mcause, MCAUSE, 0x342, rw);
csr!(mtval, MTVAL, 0x343, rw);
csr!(mip, MIP, 0x344, r);
/// Machine Protection and Translation
csr!(pmpcfg0, PMPCFG0, 0x3A0, rw);
csr!(pmpcfg1, PMPCFG1, 0x3A1, rw);
csr!(pmpcfg2, PMPCFG2, 0x3A2, rw);
csr!(pmpcfg3, PMPCFG3, 0x3A3, rw);
// TODO pmpaddr0 - pmpaddr15

/// Machine Counter/Timers
csr!(mcycle, MCYCLE, 0xB00, rw);
csr!(minstret, MINSTRET, 0xB02, rw);
// TODO mhpmcounter3 .. mhpmcounter31
csr!(mcycleh, MCYCLEH, 0xB80, rw);
csr!(minstreth, MINSTRETH, 0xB82, rw);
// TODO mhpmcounter3h .. mhpmcounter31h
/// Machine Counter Setup
// TODO mhpmevent3 .. mhpmevent31

/// Debug/Trace Registers (shared with Debug Mode)
csr!(tselect, TSELECT, 0x7A0, rw);
csr!(tdata1, TDATA1, 0x7A1, rw);
csr!(tdata2, TDATA2, 0x7A2, rw);
csr!(tdata3, TDATA3, 0x7A3, rw);
/// Debug Mode Registers
csr!(dcsr, DCSR, 0x7B0, rw);
csr!(dpc, DPC, 0x7B1, rw);
csr!(dscratch, DSCRATCH, 0x7B2, rw);

/// Machine Cause CSR (mcause) is ReadOnly.
/// Trap Cause
#[derive(Copy, Clone, Debug)]
pub enum Trap {
    Interrupt(Interrupt),
    Exception(Exception),
}

/// Interrupt
#[derive(Copy, Clone, Debug)]
pub enum Interrupt {
    UserSoft,
    SupervisorSoft,
    MachineSoft,
    UserTimer,
    SupervisorTimer,
    MachineTimer,
    UserExternal,
    SupervisorExternal,
    MachineExternal,
}

impl Interrupt {
    pub fn from(nr: u32) -> Self {
        match nr {
            0 => Interrupt::UserSoft,
            1 => Interrupt::SupervisorSoft,
            3 => Interrupt::MachineSoft,
            4 => Interrupt::UserTimer,
            5 => Interrupt::SupervisorTimer,
            7 => Interrupt::MachineTimer,
            8 => Interrupt::UserExternal,
            9 => Interrupt::SupervisorExternal,
            11 => Interrupt::MachineExternal,
            _ => unreachable!()
        }
    }
}

/// Exception
#[derive(Copy, Clone, Debug)]
pub enum Exception {
    InstructionMisaligned,
    InstructionFault,
    IllegalInstruction,
    Breakpoint,
    LoadMisaligned,
    LoadFault,
    StoreMisaligned,
    StoreFault,
    UserEnvCall,
    SupervisorEnvCall,
    MachineEnvCall,
    InstructionPageFault,
    LoadPageFault,
    StorePageFault,
}

impl Exception {
    pub fn from(nr: u32) -> Self {
        match nr {
            0 => Exception::InstructionMisaligned,
            1 => Exception::InstructionFault,
            2 => Exception::IllegalInstruction,
            3 => Exception::Breakpoint,
            4 => Exception::LoadMisaligned,
            5 => Exception::LoadFault,
            6 => Exception::StoreMisaligned,
            7 => Exception::StoreFault,
            8 => Exception::UserEnvCall,
            9 => Exception::SupervisorEnvCall,
            11 => Exception::MachineEnvCall,
            12 => Exception::InstructionPageFault,
            13 => Exception::LoadPageFault,
            15 => Exception::StorePageFault,
            _ => unreachable!()
        }
    }
}


impl mcause::R {
    #[inline(always)]
    /// Trap Cause
    pub fn cause(&self) -> Trap {
        let bits = self.bits();
        let code = bits & !(1 << 31);
        match bits & (1 << 31) == 1 << 31 {
            true => Trap::Interrupt(Interrupt::from(code)),
            false => Trap::Exception(Exception::from(code)),
        }
    }

    #[inline(always)]
    /// Is trap cause an interrupt.
    pub fn is_interrupt(&self) -> bool {
        match self.cause() {
            Trap::Interrupt(_) => true,
            _ => false,
        }
    }

    #[inline(always)]
    /// Is trap cause an exception.
    pub fn is_exception(&self) -> bool {
        match self.cause() {
            Trap::Exception(_) => true,
            _ => false,
        }
    }
}

/// Machine Status CSR is ReadWrite
// TODO: Virtualization, Memory Privilege and Extension Context Fields

/// Machine Previous Privilege Mode
pub enum MPP {
    Machine = 3,
    Supervisor = 1,
    User = 0,
}

/// Supervisor Previous Privilege Mode
pub enum SPP {
    Supervisor = 1,
    User = 0,
}


impl mstatus::R {
    #[inline(always)]
    /// User Interrupt Enable
    pub fn uie(&self) -> bool {
        self.bits() & (1 << 0) == 1 << 0
    }

    #[inline(always)]
    /// Supervisor Interrupt Enable
    pub fn sie(&self) -> bool {
        self.bits() & (1 << 1) == 1 << 1
    }

    #[inline(always)]
    /// Machine Interrupt Enable
    pub fn mie(&self) -> bool {
        self.bits() & (1 << 3) == 1 << 3
    }

    #[inline(always)]
    /// User Previous Interrupt Enable
    pub fn upie(&self) -> bool {
        self.bits() & (1 << 4) == 1 << 4
    }

    #[inline(always)]
    /// Supervisor Previous Interrupt Enable
    pub fn spie(&self) -> bool {
        self.bits() & (1 << 5) == 1 << 5
    }

    #[inline(always)]
    /// User Previous Interrupt Enable
    pub fn mpie(&self) -> bool {
        self.bits() & (1 << 7) == 1 << 7
    }

    #[inline(always)]
    /// Supervisor Previous Privilege Mode
    pub fn spp(&self) -> SPP {
        match self.bits() & (1 << 8) == (1 << 8) {
            true => SPP::Supervisor,
            false => SPP::User,
        }
    }

    #[inline(always)]
    /// Machine Previous Privilege Mode
    pub fn mpp(&self) -> MPP {
        match (self.bits() & (0b11 << 11)) >> 11 {
            0b00 => MPP::User,
            0b01 => MPP::Supervisor,
            0b11 => MPP::Machine,
            _ => unreachable!(),
        }
    }
}

impl mstatus::W {
    #[inline(always)]
    /// User Interrupt Enable
    pub fn uie(&mut self) -> &mut mstatus::W {
        self.set_bits(1 << 0)
    }

    #[inline(always)]
    /// Supervisor Interrupt Enable
    pub fn sie(&mut self) -> &mut mstatus::W {
        self.set_bits(1 << 1)
    }

    #[inline(always)]
    /// Machine Interrupt Enable
    pub fn mie(&mut self) -> &mut mstatus::W {
        self.set_bits(1 << 3)
    }

    #[inline(always)]
    /// User Previous Interrupt Enable
    pub fn upie(&mut self) -> &mut mstatus::W {
        self.set_bits(1 << 4)
    }

    #[inline(always)]
    /// User Previous Interrupt Enable
    pub fn spie(&mut self) -> &mut mstatus::W {
        self.set_bits(1 << 5)
    }

    #[inline(always)]
    /// User Previous Interrupt Enable
    pub fn mpie(&mut self) -> &mut mstatus::W {
        self.set_bits(1 << 7)
    }

    #[inline(always)]
    /// Supervisor Previous Privilege Mode
    pub fn spp(&mut self, value: SPP) -> &mut mstatus::W {
        self.set_bits((value as u32) << 8)
    }

    #[inline(always)]
    /// Machine Previous Privilege Mode
    pub fn mpp(&mut self, value: MPP) -> &mut mstatus::W {
        self.set_bits((value as u32) << 11)
    }
}

/// Machine Interrupt Enable CSR (mie) is ReadWrite.
impl mie::R {
    #[inline(always)]
    /// User Software Interrupt Enable
    pub fn usoft(&self) -> bool {
        self.bits() & (1 << 0) == 1 << 0
    }

    #[inline(always)]
    /// Supervisor Software Interrupt Enable
    pub fn ssoft(&self) -> bool {
        self.bits() & (1 << 1) == 1 << 1
    }

    #[inline(always)]
    /// Machine Software Interrupt Enable
    pub fn msoft(&self) -> bool {
        self.bits() & (1 << 3) == 1 << 3
    }

    #[inline(always)]
    /// User Timer Interrupt Enable
    pub fn utimer(&self) -> bool {
        self.bits() & (1 << 4) == 1 << 4
    }

    #[inline(always)]
    /// Supervisor Timer Interrupt Enable
    pub fn stimer(&self) -> bool {
        self.bits() & (1 << 5) == 1 << 5
    }

    #[inline(always)]
    /// Machine Timer Interrupt Enable
    pub fn mtimer(&self) -> bool {
        self.bits() & (1 << 7) == 1 << 7
    }

    #[inline(always)]
    /// User External Interrupt Enable
    pub fn uext(&self) -> bool {
        self.bits() & (1 << 8) == 1 << 8
    }

    #[inline(always)]
    /// Supervisor External Interrupt Enable
    pub fn sext(&self) -> bool {
        self.bits() & (1 << 9) == 1 << 9
    }

    #[inline(always)]
    /// Machine External Interrupt Enable
    pub fn mext(&self) -> bool {
        self.bits() & (1 << 11) == 1 << 11
    }
}

impl mie::W {
    #[inline(always)]
    /// User Software Interrupt Enable
    pub fn usoft(&mut self) -> &mut mie::W {
        self.set_bits(1 << 0)
    }

    #[inline(always)]
    /// Supervisor Software Interrupt Enable
    pub fn ssoft(&mut self) -> &mut mie::W {
        self.set_bits(1 << 1)
    }

    #[inline(always)]
    /// Machine Software Interrupt Enable
    pub fn msoft(&mut self) -> &mut mie::W {
        self.set_bits(1 << 3)
    }

    #[inline(always)]
    /// User Timer Interrupt Enable
    pub fn utimer(&mut self) -> &mut mie::W {
        self.set_bits(1 << 4)
    }

    #[inline(always)]
    /// Supervisor Timer Interrupt Enable
    pub fn stimer(&mut self) -> &mut mie::W {
        self.set_bits(1 << 5)
    }

    #[inline(always)]
    /// Machine Timer Interrupt Enable
    pub fn mtimer(&mut self) -> &mut mie::W {
        self.set_bits(1 << 7)
    }

    #[inline(always)]
    /// User External Interrupt Enable
    pub fn uext(&mut self) -> &mut mie::W {
        self.set_bits(1 << 8)
    }

    #[inline(always)]
    /// Supervisor External Interrupt Enable
    pub fn sext(&mut self) -> &mut mie::W {
        self.set_bits(1 << 9)
    }

    #[inline(always)]
    /// Machine External Interrupt Enable
    pub fn mext(&mut self) -> &mut mie::W {
        self.set_bits(1 << 11)
    }
}

/// Machine Interrupt Pending CSR (mip) is ReadOnly.
impl mip::R {
    #[inline(always)]
    /// User Software Interrupt Enable
    pub fn usoft(&self) -> bool {
        self.bits() & (1 << 0) == 1 << 0
    }

    #[inline(always)]
    /// Supervisor Software Interrupt Enable
    pub fn ssoft(&self) -> bool {
        self.bits() & (1 << 1) == 1 << 1
    }

    #[inline(always)]
    /// Machine Software Interrupt Enable
    pub fn msoft(&self) -> bool {
        self.bits() & (1 << 3) == 1 << 3
    }

    #[inline(always)]
    /// User Timer Interrupt Enable
    pub fn utimer(&self) -> bool {
        self.bits() & (1 << 4) == 1 << 4
    }

    #[inline(always)]
    /// Supervisor Timer Interrupt Enable
    pub fn stimer(&self) -> bool {
        self.bits() & (1 << 5) == 1 << 5
    }

    #[inline(always)]
    /// Machine Timer Interrupt Enable
    pub fn mtimer(&self) -> bool {
        self.bits() & (1 << 7) == 1 << 7
    }

    #[inline(always)]
    /// User External Interrupt Enable
    pub fn uext(&self) -> bool {
        self.bits() & (1 << 8) == 1 << 8
    }

    #[inline(always)]
    /// Supervisor External Interrupt Enable
    pub fn sext(&self) -> bool {
        self.bits() & (1 << 9) == 1 << 9
    }

    #[inline(always)]
    /// Machine External Interrupt Enable
    pub fn mext(&self) -> bool {
        self.bits() & (1 << 11) == 1 << 11
    }
}
