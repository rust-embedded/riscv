use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::collections::HashMap;
use std::str::FromStr;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    spanned::Spanned,
    Data, DeriveInput, Ident, Token,
};

/// Traits that can be implemented using the `pac_enum` macro
enum PacTrait {
    Exception,
    Interrupt(InterruptType),
    Priority,
    HartId,
}

impl PacTrait {
    /// Returns a token stream representing the trait name
    fn trait_name(&self) -> TokenStream2 {
        match self {
            Self::Exception => quote!(ExceptionNumber),
            Self::Interrupt(_) => quote!(InterruptNumber),
            Self::Priority => quote!(PriorityNumber),
            Self::HartId => quote!(HartIdNumber),
        }
    }

    /// Returns a token stream representing the data type used to represent the number
    fn num_type(&self) -> TokenStream2 {
        match self {
            Self::Exception => quote!(usize),
            Self::Interrupt(_) => quote!(usize),
            Self::Priority => quote!(u8),
            Self::HartId => quote!(u16),
        }
    }

    /// Returns a token stream representing the name of the constant that holds the maximum number
    fn const_name(&self) -> TokenStream2 {
        match self {
            Self::Exception => quote!(MAX_EXCEPTION_NUMBER),
            Self::Interrupt(_) => quote!(MAX_INTERRUPT_NUMBER),
            Self::Priority => quote!(MAX_PRIORITY_NUMBER),
            Self::HartId => quote!(MAX_HART_ID_NUMBER),
        }
    }
}

impl Parse for PacTrait {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![unsafe]>()?;
        let trait_name: TokenStream2 = input.parse()?;
        match trait_name.to_string().as_str() {
            "ExceptionNumber" => Ok(Self::Exception),
            "CoreInterruptNumber" => Ok(Self::Interrupt(InterruptType::Core)),
            "ExternalInterruptNumber" => Ok(Self::Interrupt(InterruptType::External)),
            "PriorityNumber" => Ok(Self::Priority),
            "HartIdNumber" => Ok(Self::HartId),
            _ => Err(syn::Error::new(
                trait_name.span(),
                "Unknown trait name. Expected: 'ExceptionNumber', 'CoreInterruptNumber', 'ExternalInterruptNumber', 'PriorityNumber', or 'HartIdNumber'",
            )),
        }
    }
}

/// Marker traits for interrupts
enum InterruptType {
    Core,
    External,
}

impl InterruptType {
    /// Returns a token stream representing the name of the marker trait
    fn marker_trait_name(&self) -> TokenStream2 {
        match self {
            Self::Core => quote!(CoreInterruptNumber),
            Self::External => quote!(ExternalInterruptNumber),
        }
    }

    /// Returns a token stream representing the name of the array of interrupt service routines
    fn isr_array_name(&self) -> TokenStream2 {
        match self {
            Self::Core => quote!(__CORE_INTERRUPTS),
            Self::External => quote!(__EXTERNAL_INTERRUPTS),
        }
    }

    /// Returns a token stream representing the name of the interrupt dispatch function
    fn dispatch_fn_name(&self) -> TokenStream2 {
        match self {
            Self::Core => quote!(_dispatch_core_interrupt),
            Self::External => quote!(_dispatch_external_interrupt),
        }
    }
}

/// Struct containing the information needed to implement the `riscv-pac` traits for an enum
struct PacEnumItem {
    /// The name of the enum
    name: Ident,
    /// The maximum discriminant value
    max_number: usize,
    /// A map from discriminant values to variant names
    numbers: HashMap<usize, Ident>,
}

impl PacEnumItem {
    fn new(input: &DeriveInput) -> Self {
        let name = input.ident.clone();
        let (mut numbers, mut max_number) = (HashMap::new(), 0);

        let variants = match &input.data {
            Data::Enum(data) => &data.variants,
            _ => panic!("Input is not an enum"),
        };
        for v in variants.iter() {
            let ident = v.ident.clone();
            let value = match &v.discriminant {
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

            if numbers.insert(value, ident).is_some() {
                panic!("Duplicate discriminant value");
            }
            if value > max_number {
                max_number = value;
            }
        }

        Self {
            name,
            max_number,
            numbers,
        }
    }

    /// Returns a token stream representing the maximum discriminant value of the enum
    fn max_discriminant(&self) -> TokenStream2 {
        TokenStream2::from_str(&format!("{}", self.max_number)).unwrap()
    }

    /// Returns a vector of token streams representing the valid matches in the `pac::from_number` function
    fn valid_matches(&self) -> Vec<TokenStream2> {
        self.numbers
            .iter()
            .map(|(num, ident)| {
                TokenStream2::from_str(&format!("{num} => Ok(Self::{ident})")).unwrap()
            })
            .collect()
    }

    /// Returns a vector of token streams representing the interrupt handler functions
    fn interrupt_handlers(&self) -> Vec<TokenStream2> {
        self.numbers
            .values()
            .map(|ident| {
                quote! { fn #ident () }
            })
            .collect()
    }

    /// Returns a sorted vector of token streams representing all the elements of the interrupt array.
    /// If an interrupt number is not present in the enum, the corresponding element is `None`.
    /// Otherwise, it is `Some(<interrupt_handler>)`.
    fn interrupt_array(&self) -> Vec<TokenStream2> {
        let mut vectors = vec![];
        for i in 0..=self.max_number {
            if let Some(ident) = self.numbers.get(&i) {
                vectors.push(quote! { Some(#ident) });
            } else {
                vectors.push(quote! { None });
            }
        }
        vectors
    }

    fn vector_table(&self) -> TokenStream2 {
        let mut asm = String::from(
            r#"
#[cfg(all(feature = "v-trap", any(target_arch = "riscv32", target_arch = "riscv64")))]
core::arch::global_asm!("
    .section .trap, \"ax\"
    .global _vector_table
    .type _vector_table, @function
    
    .option push
    .balign 0x4 // TODO check if this is the correct alignment
    .option norelax
    .option norvc
    
    _vector_table:
        j _start_trap  // Interrupt 0 is used for exceptions
"#,
        );

        for i in 1..=self.max_number {
            if let Some(ident) = self.numbers.get(&i) {
                asm.push_str(&format!("        j _start_{ident}_trap\n"));
            } else {
                asm.push_str(&format!(
                    "        j _start_DefaultHandler_trap // Interrupt {i} is reserved\n"
                ));
            }
        }

        asm.push_str(
            r#"    .option pop"
);"#,
        );

        TokenStream2::from_str(&asm).unwrap()
    }

    /// Returns a vector of token streams representing the trait implementations for
    /// the enum. If the trait is an interrupt trait, the implementation also includes
    /// the interrupt handler functions and the interrupt array.
    fn impl_trait(&self, attr: &PacTrait) -> Vec<TokenStream2> {
        let mut res = vec![];

        let name = &self.name;

        let trait_name = attr.trait_name();
        let num_type = attr.num_type();
        let const_name = attr.const_name();

        let max_discriminant = self.max_discriminant();
        let valid_matches = self.valid_matches();

        // Push the trait implementation
        res.push(quote! {
            unsafe impl riscv::#trait_name for #name {
                const #const_name: #num_type = #max_discriminant;

                #[inline]
                fn number(self) -> #num_type {
                    self as _
                }

                #[inline]
                fn from_number(number: #num_type) -> riscv::result::Result<Self> {
                    match number {
                        #(#valid_matches,)*
                        _ => Err(riscv::result::Error::InvalidVariant(number as _)),
                    }
                }
            }
        });

        // Interrupt traits require additional code
        if let PacTrait::Interrupt(interrupt_type) = attr {
            let marker_trait_name = interrupt_type.marker_trait_name();

            let isr_array_name = interrupt_type.isr_array_name();
            let dispatch_fn_name = interrupt_type.dispatch_fn_name();

            // Push the marker trait implementation
            res.push(quote! { unsafe impl riscv::#marker_trait_name for #name {} });

            let interrupt_handlers = self.interrupt_handlers();
            let interrupt_array = self.interrupt_array();

            // Push the interrupt handler functions and the interrupt array
            res.push(quote! {
                extern "C" {
                    #(#interrupt_handlers;)*
                }

                #[no_mangle]
                pub static #isr_array_name: [Option<unsafe extern "C" fn()>; #max_discriminant + 1] = [
                    #(#interrupt_array),*
                ];

                #[no_mangle]
                unsafe extern "C" fn #dispatch_fn_name(code: usize) {
                    extern "C" {
                        fn DefaultHandler();
                    }

                    if code < #isr_array_name.len() {
                        let h = &#isr_array_name[code];
                        if let Some(handler) = h {
                            handler();
                        } else {
                            DefaultHandler();
                        }
                    } else {
                        DefaultHandler();
                    }
                }
            });

            if let InterruptType::Core = interrupt_type {
                res.push(self.vector_table());
            }
        }

        res
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
/// use riscv::*;
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
    let pac_enum = PacEnumItem::new(&input);

    let attr = parse_macro_input!(attr as PacTrait);

    let trait_impl = pac_enum.impl_trait(&attr);
    quote! {
        #input
        #(#trait_impl)*
    }
    .into()
}
