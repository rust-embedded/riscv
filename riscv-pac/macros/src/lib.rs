extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::{collections::HashMap, convert::TryFrom, ops::Range, str::FromStr};
use syn::{parse_macro_input, Data, DeriveInput, Error, Ident};

struct PacNumberEnum {
    name: Ident,
    valid_ranges: Vec<Range<usize>>,
}

impl PacNumberEnum {
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
            unsafe impl #trait_name for #name {
                const #const_name: #num_type = #max_discriminant;

                #[inline]
                fn number(self) -> #num_type {
                    self as _
                }

                #[inline]
                fn from_number(number: #num_type) -> Result<Self, #num_type> {
                    if #valid_condition {
                        // SAFETY: The number is valid for this enum
                        Ok(unsafe { core::mem::transmute(number) })
                    } else {
                        Err(number)
                    }
                }
            }
        }
    }
}

impl TryFrom<DeriveInput> for PacNumberEnum {
    type Error = Error;

    fn try_from(input: DeriveInput) -> Result<Self, Self::Error> {
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
            // check for duplicate discriminant values
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

        Ok(PacNumberEnum {
            name: input.ident.clone(),
            valid_ranges,
        })
    }
}

#[proc_macro_derive(ExceptionNumber)]
pub fn exception_number_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let pac_enum = PacNumberEnum::try_from(input).unwrap();
    pac_enum
        .quote("ExceptionNumber", "u16", "MAX_EXCEPTION_NUMBER")
        .into()
}

#[proc_macro_derive(InterruptNumber)]
pub fn interrupt_number_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let pac_enum = PacNumberEnum::try_from(input).unwrap();
    pac_enum
        .quote("InterruptNumber", "u16", "MAX_INTERRUPT_NUMBER")
        .into()
}

#[proc_macro_derive(PriorityNumber)]
pub fn priority_number_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let pac_enum = PacNumberEnum::try_from(input).unwrap();
    pac_enum
        .quote("PriorityNumber", "u8", "MAX_PRIORITY_NUMBER")
        .into()
}

#[proc_macro_derive(HartIdNumber)]
pub fn hart_id_number_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let pac_enum = PacNumberEnum::try_from(input).unwrap();
    pac_enum
        .quote("HartIdNumber", "u16", "MAX_HART_ID_NUMBER")
        .into()
}
