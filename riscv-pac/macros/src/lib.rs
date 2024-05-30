extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::{collections::HashMap, ops::Range, str::FromStr};
use syn::{parse_macro_input, Data, DeriveInput, Ident};

struct PacNumberEnum {
    name: Ident,
    valid_ranges: Vec<Range<usize>>,
}

impl PacNumberEnum {
    fn new(input: &DeriveInput) -> Self {
        let variants = match &input.data {
            Data::Enum(data) => &data.variants,
            _ => panic!("Input is not an enum"),
        };

        // Collect the variants and their associated number discriminants
        let mut var_map = HashMap::new();
        let mut numbers = Vec::new();
        for variant in variants {
            let ident = &variant.ident;
            let value = match &variant.discriminant {
                Some(d) => match &d.1 {
                    syn::Expr::Lit(expr_lit) => match &expr_lit.lit {
                        syn::Lit::Int(lit_int) => match lit_int.base10_parse::<usize>() {
                            Ok(num) => num,
                            Err(_) => panic!("All variant discriminants must be unsigned integers"),
                        },
                        _ => panic!("All variant discriminants must be unsigned integers"),
                    },
                    _ => panic!("All variant discriminants must be unsigned integers"),
                },
                _ => panic!("Variant must have a discriminant"),
            };
            var_map.insert(value, ident);
            numbers.push(value);
        }

        // sort the number discriminants and generate a list of valid ranges
        numbers.sort_unstable();
        let mut valid_ranges = Vec::new();
        let mut start = numbers[0];
        let mut end = start;
        for &number in &numbers[1..] {
            if number == end + 1 {
                end = number;
            } else {
                valid_ranges.push(start..end + 1);
                start = number;
                end = start;
            }
        }
        valid_ranges.push(start..end + 1);

        Self {
            name: input.ident.clone(),
            valid_ranges,
        }
    }

    fn valid_condition(&self) -> TokenStream2 {
        let mut arms = Vec::new();
        for range in &self.valid_ranges {
            let (start, end) = (range.start, range.end);
            if end - start == 1 {
                arms.push(TokenStream2::from_str(&format!("number == {start}")).unwrap());
            } else {
                arms.push(
                    TokenStream2::from_str(&format!("({start}..{end}).contains(&number)")).unwrap(),
                );
            }
        }
        quote! { #(#arms) || * }
    }

    fn max_discriminant(&self) -> TokenStream2 {
        let max_discriminant = self.valid_ranges.last().expect("invalid range").end - 1;
        TokenStream2::from_str(&format!("{max_discriminant}")).unwrap()
    }

    fn quote(&self, trait_name: &str, num_type: &str, const_name: &str) -> TokenStream2 {
        let name = &self.name;
        let max_discriminant = self.max_discriminant();
        let valid_condition = self.valid_condition();

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
                    if #valid_condition {
                        // SAFETY: The number is valid for this enum
                        Ok(unsafe { core::mem::transmute::<#num_type, Self>(number) })
                    } else {
                        Err(number)
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
/// # Note
///
/// To implement number-to-enum operation, the macro works with ranges of valid discriminant numbers.
/// If the number is within any of the valid ranges, the number is transmuted to the enum variant.
/// In this way, the macro achieves better performance for enums with a large number of consecutive variants.
/// Thus, the enum must comply with the following requirements:
///
/// - All the enum variants must have a valid discriminant number (i.e., a number that is within the valid range of the enum).
/// - For the `ExceptionNumber`, `InterruptNumber`, and `HartIdNumber` traits, the enum must be annotated as `#[repr(u16)]`
/// - For the `PriorityNumber` trait, the enum must be annotated as `#[repr(u8)]`
///
/// If the enum does not meet these requirements, you will have to implement the traits manually (e.g., `riscv::mcause::Interrupt`).
/// For enums with a small number of consecutive variants, it might be better to implement the traits manually.
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
