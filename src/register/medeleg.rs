//! medeleg register

use bit_field::BitField;

/// medeleg register
#[derive(Clone, Copy, Debug)]
pub struct Medeleg {
    bits: usize,
}

impl Medeleg {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }
    
    /// Instruction address misaligned delegate
    #[inline]
    pub fn instruction_misaligned(&self) -> bool {
        self.bits.get_bit(0)
    }

    /// Instruction access fault delegate
    #[inline]
    pub fn instruction_fault(&self) -> bool {
        self.bits.get_bit(1)
    }

    /// Illegal instruction delegate
    #[inline]
    pub fn illegal_instruction(&self) -> bool {
        self.bits.get_bit(2)
    }

    /// Breakpoint delegate
    #[inline]
    pub fn breakpoint(&self) -> bool {
        self.bits.get_bit(3)
    }

    /// Load address misaligned delegate
    #[inline]
    pub fn load_misaligned(&self) -> bool {
        self.bits.get_bit(4)
    }

    /// Load access fault delegate
    #[inline]
    pub fn load_fault(&self) -> bool {
        self.bits.get_bit(5)
    }

    /// Store/AMO address misaligned delegate
    #[inline]
    pub fn store_misaligned(&self) -> bool {
        self.bits.get_bit(6)
    }

    /// Store/AMO access fault delegate
    #[inline]
    pub fn store_fault(&self) -> bool {
        self.bits.get_bit(7)
    }

    /// Environment call from U-mode delegate
    #[inline]
    pub fn user_env_call(&self) -> bool {
        self.bits.get_bit(8)
    }

    /// Environment call from S-mode delegate
    #[inline]
    pub fn supervisor_env_call(&self) -> bool {
        self.bits.get_bit(9)
    }

    /// Environment call from M-mode delegate
    #[inline]
    pub fn machine_env_call(&self) -> bool {
        self.bits.get_bit(11)
    }

    /// Instruction page fault delegate
    #[inline]
    pub fn instruction_page_fault(&self) -> bool {
        self.bits.get_bit(12)
    }

    /// Load page fault delegate
    #[inline]
    pub fn load_page_fault(&self) -> bool {
        self.bits.get_bit(13)
    }

    /// Store/AMO page fault delegate
    #[inline]
    pub fn store_page_fault(&self) -> bool {
        self.bits.get_bit(15)
    }
}

read_csr_as!(Medeleg, 0x302, __read_medeleg);
set!(0x302, __set_medeleg);
clear!(0x302, __clear_medeleg);

set_clear_csr!(
    /// Instruction address misaligned delegate
    , set_instruction_misaligned, clear_instruction_misaligned, 1 << 0);
set_clear_csr!(
    /// Instruction access fault delegate
    , set_instruction_fault, clear_instruction_fault, 1 << 1);
set_clear_csr!(
    /// Illegal instruction delegate
    , set_illegal_instruction, clear_illegal_instruction, 1 << 2);
set_clear_csr!(
    /// Breakpoint delegate
    , set_breakpoint, clear_breakpoint, 1 << 3);
set_clear_csr!(
    /// Load address misaligned delegate
    , set_load_misaligned, clear_load_misaligned, 1 << 4);
set_clear_csr!(
    /// Load access fault delegate
    , set_load_fault, clear_load_fault, 1 << 5);
set_clear_csr!(
    /// Store/AMO address misaligned delegate
    , set_store_misaligned, clear_store_misaligned, 1 << 6);
set_clear_csr!(
    /// Store/AMO access fault
    , set_store_fault, clear_store_fault, 1 << 7);
set_clear_csr!(
    /// Environment call from U-mode delegate
    , set_user_env_call, clear_user_env_call, 1 << 8);
set_clear_csr!(
    /// Environment call from S-mode delegate
    , set_supervisor_env_call, clear_supervisor_env_call, 1 << 9);
set_clear_csr!(
    /// Environment call from M-mode delegate
    , set_machine_env_call, clear_machine_env_call, 1 << 11);
set_clear_csr!(
    /// Instruction page fault delegate
    , set_instruction_page_fault, clear_instruction_page_fault, 1 << 12);
set_clear_csr!(
    /// Load page fault delegate
    , set_load_page_fault, clear_load_page_fault, 1 << 13);
set_clear_csr!(
    /// Store/AMO page fault delegate
    , set_store_page_fault, clear_store_page_fault, 1 << 15);
