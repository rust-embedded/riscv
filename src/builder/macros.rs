macro_rules! impl_get_bit {
    ($field: ident, $inner: ident, $getter: ident, $idx: literal) => {
        #[inline]
        pub fn $field(&self) -> bool {
            self.$inner.$getter($idx)
        }
    };
}

macro_rules! impl_set_bit {
    ($field_setter: ident, $inner: ident, $setter: ident, $idx: literal) => {
        #[inline]
        pub fn $field_setter(&mut self, value: bool) {
            self.$inner.$setter($idx, value);
        }
    };
}
