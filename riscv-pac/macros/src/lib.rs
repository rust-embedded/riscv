extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::str::FromStr;
use syn::{parse_macro_input, Data, DeriveInput, Ident};

struct PacNumberEnum {
    name: Ident,
    numbers: Vec<(Ident, usize)>,
}

impl PacNumberEnum {
    fn new(input: &DeriveInput) -> Self {
        let name = input.ident.clone();

        let variants = match &input.data {
            Data::Enum(data) => &data.variants,
            _ => panic!("Input is not an enum"),
        };
        let numbers = variants
            .iter()
            .map(|variant| {
                let ident = &variant.ident;
                let value = match &variant.discriminant {
                    Some(d) => match &d.1 {
                        syn::Expr::Lit(expr_lit) => match &expr_lit.lit {
                            syn::Lit::Int(lit_int) => match lit_int.base10_parse::<usize>() {
                                Ok(num) => num,
                                Err(_) => {
                                    panic!("All variant discriminants must be unsigned integers")
                                }
                            },
                            _ => panic!("All variant discriminants must be unsigned integers"),
                        },
                        _ => panic!("All variant discriminants must be unsigned integers"),
                    },
                    _ => panic!("Variant must have a discriminant"),
                };
                (ident.clone(), value)
            })
            .collect();

        Self { name, numbers }
    }

    fn max_discriminant(&self) -> TokenStream2 {
        let max_discriminant = self.numbers.iter().map(|(_, num)| num).max().unwrap();
        TokenStream2::from_str(&format!("{max_discriminant}")).unwrap()
    }

    fn valid_matches(&self) -> Vec<TokenStream2> {
        self.numbers
            .iter()
            .map(|(ident, num)| {
                TokenStream2::from_str(&format!("{num} => Ok(Self::{ident})")).unwrap()
            })
            .collect()
    }

    fn quote(&self, trait_name: &str, num_type: &str, const_name: &str) -> TokenStream2 {
        let name = &self.name;
        let max_discriminant = self.max_discriminant();
        let valid_matches = self.valid_matches();

        let trait_name = TokenStream2::from_str(trait_name).unwrap();
        let num_type = TokenStream2::from_str(num_type).unwrap();
        let const_name = TokenStream2::from_str(const_name).unwrap();

        quote! {
            unsafe impl riscv_pac::#trait_name for #name {
                const #const_name: #num_type = #max_discriminant;

                #[inline]
                fn number(self) -> #num_type {
                    self as _
                }

                #[inline]
                fn from_number(number: #num_type) -> Result<Self, #num_type> {
                    match number {
                        #(#valid_matches,)*
                        _ => Err(number),
                    }
                }
            }
        }
    }
}

/// Attribute-like macro that implements the traits of the `riscv-pac` crate for a given enum.
///
/// As these traits are unsafe, the macro must be called with the `unsafe` keyword followed by the trait name.
/// In this way, we warn callers that they must comply with the requirements of the trait.
///
/// The trait name must be one of `ExceptionNumber`, `InterruptNumber`, `PriorityNumber`, or `HartIdNumber`.
/// Marker traits `CoreInterruptNumber` and `ExternalInterruptNumber` cannot be implemented using this macro.
///
/// # Safety
///
/// The struct to be implemented must comply with the requirements of the specified trait.
///
/// # Example
///
/// ```rust
/// use riscv_pac::*;
///
/// #[repr(usize)]
/// #[pac_enum(unsafe ExceptionNumber)]
/// #[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// enum Exception {
///     E1 = 1,
///     E3 = 3,
/// }
///
/// fn main() {
///     assert_eq!(Exception::E1.number(), 1);
///     assert_eq!(Exception::E3.number(), 3);
///
///     assert_eq!(Exception::from_number(1), Ok(Exception::E1));
///     assert_eq!(Exception::from_number(2), Err(2));
///     assert_eq!(Exception::from_number(3), Ok(Exception::E3));
///
///     assert_eq!(Exception::MAX_EXCEPTION_NUMBER, 3);
/// }
///```
#[proc_macro_attribute]
pub fn pac_enum(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let pac_enum = PacNumberEnum::new(&input);

    // attr should be unsafe ExceptionNumber, unsafe InterruptNumber, unsafe PriorityNumber, or unsafe HartIdNumber
    // assert that attribute starts with the unsafe token. If not, raise a panic error
    let attr = attr.to_string();
    // split string into words and check if the first word is "unsafe"
    let attrs = attr.split_whitespace().collect::<Vec<&str>>();
    if attrs.is_empty() {
        panic!("Attribute is empty. Expected: 'riscv_pac::pac_enum(unsafe <PacTraitToImplement>)'");
    }
    if attrs.len() > 2 {
        panic!(
            "Wrong attribute format. Expected: 'riscv_pac::pac_enum(unsafe <PacTraitToImplement>)'"
        );
    }
    if attrs[0] != "unsafe" {
        panic!("Attribute does not start with 'unsafe'. Expected: 'riscv_pac::pac_enum(unsafe <PacTraitToImplement>)'");
    }

    let trait_impl = match attrs[1] {
        "ExceptionNumber" => pac_enum.quote("ExceptionNumber", "usize", "MAX_EXCEPTION_NUMBER"),
        "InterruptNumber" => pac_enum.quote("InterruptNumber", "usize", "MAX_INTERRUPT_NUMBER"),
        "PriorityNumber" => pac_enum.quote("PriorityNumber", "u8", "MAX_PRIORITY_NUMBER"),
        "HartIdNumber" => pac_enum.quote("HartIdNumber", "u16", "MAX_HART_ID_NUMBER"),
        _ => panic!("Unknown trait '{}'. Expected: 'ExceptionNumber', 'InterruptNumber', 'PriorityNumber', or 'HartIdNumber'", attrs[1]),
    };
    quote! {
        #input
        #trait_impl
    }
    .into()
}
