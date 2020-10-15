//! mcounteren register


use bit_field::BitField;
use core::mem::size_of;

/// mcounteren register
#[derive(Clone, Copy, Debug)]
pub struct Mcounteren {
    bits: usize,
}


/*
/// Cycle enable/disable in mcounteren
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CY {
    Enabled = 1,
    Disabled = 0,
}


/// Time enable/disable in mcounteren
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Time {
    Enabled = 1,
    Disabled = 0,
}


/// Instret enable/disable in mcounteren
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Instret {
    Enabled = 1,
    Disabled = 0,
}

*/

/// Enable/disable in mcounteren
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum State {
    Enabled = 1,
    Disabled = 0,
}
impl Mcounteren {
    /// User "cycle[h]" Enable
    #[inline]
    pub fn cy(&self) -> State {
        match self.bits.get_bit(0) {
            true => State::Enabled,
            false => State::Disabled,
        }
    }

    /// User "time[h]" Enable
    #[inline]
    pub fn tm(&self) -> State {
        match self.bits.get_bit(1) {
            true => State::Enabled,
            false => State::Disabled,
        }
    }

    /// User "instret[h]" Enable
    #[inline]
    pub fn ir(&self) -> State {
        match self.bits.get_bit(2) {
            true => State::Enabled,
            false => State::Disabled,
        }
    }


        /// User "hpm3" Enable
        #[inline]
        pub fn hpm3(&self) -> State {
            match self.bits.get_bit(3) {
                true => State::Enabled,
                false => State::Disabled,
            }
        }

        /// User "hpm4" Enable
        #[inline]
        pub fn hpm4(&self) -> State {
            match self.bits.get_bit(4){
                true => State::Enabled,
                false => State::Disabled,
            }
        }

        /// User "hpm5" Enable
        #[inline]
        pub fn hpm5(&self) -> State {
            match self.bits.get_bit(5){
                true => State::Enabled,
                false => State::Disabled,
            }
        }

        /// User "hpm6" Enable
        #[inline]
        pub fn hpm6(&self) -> State {
            match self.bits.get_bit(6){
                true => State::Enabled,
                false => State::Disabled,
            }
        }

        /// User "hpm7" Enable
        #[inline]
        pub fn hpm7(&self) -> State {
            match self.bits.get_bit(7){
                true => State::Enabled,
                false => State::Disabled,
            }
        }

        /// User "hpm8" Enable
        #[inline]
        pub fn hpm8(&self) -> State {
            match self.bits.get_bit(8){
                true => State::Enabled,
                false => State::Disabled,
            }
        }

        /// User "hpm9" Enable
        #[inline]
        pub fn hpm9(&self) -> State {
            match self.bits.get_bit(9){
                true => State::Enabled,
                false => State::Disabled,
            }
        }

        /// User "hpm10" Enable
        #[inline]
        pub fn hpm10(&self) -> State {
            match self.bits.get_bit(10){
                true => State::Enabled,
                false => State::Disabled,
            }
        }

        /// User "hpm11" Enable
        #[inline]
        pub fn hpm11(&self) -> State {
            match self.bits.get_bit(11){
                true => State::Enabled,
                false => State::Disabled,
            }
        }

        /// User "hpm12" Enable
        #[inline]
        pub fn hpm12(&self) -> State {
            match self.bits.get_bit(12){
                true => State::Enabled,
                false => State::Disabled,
            }
        }

        /// User "hpm13" Enable
        #[inline]
        pub fn hpm13(&self) -> State {
            match self.bits.get_bit(13){
                true => State::Enabled,
                false => State::Disabled,
            }
        }

        /// User "hpm14" Enable
        #[inline]
        pub fn hpm14(&self) -> State {
            match self.bits.get_bit(14){
                true => State::Enabled,
                false => State::Disabled,
            }
        }

        /// User "hpm15" Enable
        #[inline]
        pub fn hpm15(&self) -> State {
            match self.bits.get_bit(15){
                true => State::Enabled,
                false => State::Disabled,
            }
        }

        /// User "hpm16" Enable
        #[inline]
        pub fn hpm16(&self) -> State {
            match self.bits.get_bit(16){
                true => State::Enabled,
                false => State::Disabled,
            }
        }

        /// User "hpm17" Enable
        #[inline]
        pub fn hpm17(&self) -> State {
            match self.bits.get_bit(17){
                true => State::Enabled,
                false => State::Disabled,
            }
        }

        /// User "hpm18" Enable
        #[inline]
        pub fn hpm18(&self) -> State {
            match self.bits.get_bit(18){
                true => State::Enabled,
                false => State::Disabled,
            }
        }

        /// User "hpm19" Enable
        #[inline]
        pub fn hpm19(&self) -> State {
            match self.bits.get_bit(19){
                true => State::Enabled,
                false => State::Disabled,
            }
        }

        /// User "hpm20" Enable
        #[inline]
        pub fn hpm20(&self) -> State {
            match self.bits.get_bit(20){
                true => State::Enabled,
                false => State::Disabled,
            }
        }

        /// User "hpm21" Enable
        #[inline]
        pub fn hpm21(&self) -> State {
            match self.bits.get_bit(21){
                true => State::Enabled,
                false => State::Disabled,
            }
        }

        /// User "hpm22" Enable
        #[inline]
        pub fn hpm22(&self) -> State {
            match self.bits.get_bit(22){
                true => State::Enabled,
                false => State::Disabled,
            }
        }

        /// User "hpm23" Enable
        #[inline]
        pub fn hpm23(&self) -> State {
            match self.bits.get_bit(23){
                true => State::Enabled,
                false => State::Disabled,
            }
        }

        /// User "hpm24" Enable
        #[inline]
        pub fn hpm24(&self) -> State {
            match self.bits.get_bit(24){
                true => State::Enabled,
                false => State::Disabled,
            }
        }

        /// User "hpm25" Enable
        #[inline]
        pub fn hpm25(&self) -> State {
            match self.bits.get_bit(25){
                true => State::Enabled,
                false => State::Disabled,
            }
        }

        /// User "hpm26" Enable
        #[inline]
        pub fn hpm26(&self) -> State {
            match self.bits.get_bit(26){
                true => State::Enabled,
                false => State::Disabled,
            }
        }

        /// User "hpm27" Enable
        #[inline]
        pub fn hpm27(&self) -> State {
            match self.bits.get_bit(27){
                true => State::Enabled,
                false => State::Disabled,
            }
        }

        /// User "hpm24" Enable
        #[inline]
        pub fn hpm28(&self) -> State {
            match self.bits.get_bit(28){
                true => State::Enabled,
                false => State::Disabled,
            }
        }

        /// User "hpm29" Enable
        #[inline]
        pub fn hpm29(&self) -> State {
            match self.bits.get_bit(29){
                true => State::Enabled,
                false => State::Disabled,
            }
        }

        /// User "hpm30" Enable
        #[inline]
        pub fn hpm30(&self) -> State {
            match self.bits.get_bit(30){
                true => State::Enabled,
                false => State::Disabled,
            }
        }

        /// User "hpm31" Enable
        #[inline]
        pub fn hpm31(&self) -> State {
            match self.bits.get_bit(31){
                true => State::Enabled,
                false => State::Disabled,
            }
        }



    read_csr_as!(Mcounteren, 0x306, __read_mcounteren);
    write_csr!(0x306, __write_mcounteren);
    set!(0x306, __set_mcounteren);
    clear!(0x306, __clear_mcounteren);

    set_clear_csr!(
    /// User cycle Enable
    , set_cy, clear_cy, 1 << 0);

    set_clear_csr!(
    /// User time Enable
    , set_tm, clear_tm, 1 << 1);

    set_clear_csr!(
    /// User instret Enable
    , set_ir, clear_ir, 1 << 2);

    set_clear_csr!(
    /// User hpmcounter3 Enable
    , set_hpm3, clear_hpm3, 1 << 3);

    set_clear_csr!(
    /// User hpmcounter4 Enable
    , set_hpm4, clear_hpm4, 1 << 4);

    set_clear_csr!(
    /// User hpmcounter5 Enable
    , set_hpm5, clear_hpm5, 1 << 5);

    set_clear_csr!(
    /// User hpmcounter6 Enable
    , set_hpm6, clear_hpm6, 1 << 6);

    set_clear_csr!(
    /// User hpmcounter7 Enable
    , set_hpm7, clear_hpm7, 1 << 7);

    set_clear_csr!(
    /// User hpmcounter8 Enable
    , set_hpm8, clear_hpm8, 1 << 8);

    set_clear_csr!(
    /// User hpmcounter9 Enable
    , set_hpm9, clear_hpm9, 1 << 9);

    set_clear_csr!(
    /// User hpmcounter10 Enable
    , set_hpm10, clear_hpm10, 1 << 10);

    set_clear_csr!(
    /// User hpmcounter11 Enable
    , set_hpm11, clear_hpm11, 1 << 11);

    set_clear_csr!(
    /// User hpmcounter12 Enable
    , set_hpm12, clear_hpm12, 1 << 12);

    set_clear_csr!(
    /// User hpmcounter13 Enable
    , set_hpm13, clear_hpm13, 1 << 13);


    set_clear_csr!(
    /// User hpmcounter14 Enable
    , set_hpm14, clear_hpm14, 1 << 14);

    set_clear_csr!(
    /// User hpmcounter15 Enable
    , set_hpm15, clear_hpm15, 1 << 15);

    set_clear_csr!(
    /// User hpmcounter16 Enable
    , set_hpm16, clear_hpm16, 1 << 16);

    set_clear_csr!(
    /// User hpmcounter17 Enable
    , set_hpm17, clear_hpm17, 1 << 17);

    set_clear_csr!(
    /// User hpmcounter18 Enable
    , set_hpm18, clear_hpm18, 1 << 18);

    set_clear_csr!(
    /// User hpmcounter19 Enable
    , set_hpm19, clear_hpm19, 1 << 19);

    set_clear_csr!(
    /// User hpmcounter20 Enable
    , set_hpm20, clear_hpm20, 1 << 20);

    set_clear_csr!(
    /// User hpmcounter21 Enable
    , set_hpm21, clear_hpm21, 1 << 21);

    set_clear_csr!(
    /// User hpmcounter22 Enable
    , set_hpm22, clear_hpm22, 1 << 22);

    set_clear_csr!(
    /// User hpmcounter23 Enable
    , set_hpm23, clear_hpm23, 1 << 23);

    set_clear_csr!(
    /// User hpmcounter24 Enable
    , set_hpm24, clear_hpm24, 1 << 24);

    set_clear_csr!(
    /// User hpmcounter25 Enable
    , set_hpm25, clear_hpm25, 1 << 25);

    set_clear_csr!(
    /// User hpmcounter26 Enable
    , set_hpm26, clear_hpm26, 1 << 26);

    set_clear_csr!(
    /// User hpmcounter27 Enable
    , set_hpm27, clear_hpm27, 1 << 27);

    set_clear_csr!(
    /// User hpmcounter28 Enable
    , set_hpm28, clear_hpm28, 1 << 28);

    set_clear_csr!(
    /// User hpmcounter29 Enable
    , set_hpm29, clear_hpm29, 1 << 29);

    set_clear_csr!(
    /// User hpmcounter30 Enable
    , set_hpm30, clear_hpm30, 1 << 30);

    set_clear_csr!(
    /// User hpmcounter31 Enable
    , set_hpm31, clear_hpm31, 1 << 31);
}
