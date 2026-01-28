use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod riscv;

#[cfg(feature = "riscv-rt")]
mod riscv_rt;

/// Attribute-like macro that implements the traits of the `riscv-types` crate for a given enum.
///
/// As these traits are unsafe, the macro must be called with the `unsafe` keyword followed by the trait name.
/// In this way, we warn callers that they must comply with the requirements of the trait.
///
/// The trait name must be one of `ExceptionNumber`, `CoreInterruptNumber`, `ExternalInterruptNumber`,
/// `PriorityNumber`, or `HartIdNumber`.
///
/// # Note
///
/// Crates using this macro must depend on the `riscv` crate, as the generated code references it.
///
/// If the `rt` feature is enabled, the generated code may also include the necessary runtime support
/// for interrupt and exception handling. Thus, the calling crate must also depend on the `riscv-rt` crate.
///
/// # Safety
///
/// The struct to be implemented must comply with the requirements of the specified trait.
///
/// # Example
///
/// ```rust,ignore,no_run
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
/// assert_eq!(Exception::E1.number(), 1);
/// assert_eq!(Exception::E3.number(), 3);
///
/// assert_eq!(Exception::from_number(1), Ok(Exception::E1));
/// assert_eq!(Exception::from_number(2), Err(2));
/// assert_eq!(Exception::from_number(3), Ok(Exception::E3));
///
/// assert_eq!(Exception::MAX_EXCEPTION_NUMBER, 3);
///```
#[proc_macro_attribute]
pub fn pac_enum(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let pac_enum = riscv::PacEnumItem::new(&input);

    let attr = parse_macro_input!(attr as riscv::PacTrait);

    let trait_impl = pac_enum.impl_trait(&attr);
    quote! {
        #input
        #(#trait_impl)*
    }
    .into()
}

/// Attribute to mark which function will be called before jumping to the entry point.
/// You must enable the `post-init` feature in the `riscv-rt` crate to use this macro.
///
/// In contrast with `__pre_init`, this function is called after the static variables
/// are initialized, so it is safe to access them. It is also safe to run Rust code.
///
/// The function must have the signature of `[unsafe] fn([usize])`, where the argument
/// corresponds to the hart ID of the current hart. This is useful for multi-hart systems
/// to perform hart-specific initialization.
///
/// # IMPORTANT
///
/// This attribute can appear at most *once* in the dependency graph.
///
/// # Examples
///
/// ```
/// #[riscv_macros::post_init]
/// unsafe fn before_main(hart_id: usize) {
///     // do something here
/// }
/// ```
#[cfg(feature = "riscv-rt")]
#[proc_macro_attribute]
pub fn post_init(args: TokenStream, input: TokenStream) -> TokenStream {
    riscv_rt::Fn::post_init(args, input)
}

/// Attribute to declare the entry point of the program
///
/// The specified function will be called by the reset handler *after* RAM has been initialized.
/// If present, the FPU will also be enabled before the function is called.
///
/// # Signature
///
/// ## Regular Usage
///
/// The type of the specified function must be `[unsafe] fn([usize[, usize[, usize]]]) -> !` (never ending function).
/// The optional arguments correspond to the values passed in registers `a0`, `a1`, and `a2`.
/// The first argument holds the hart ID of the current hart, which is useful for multi-hart systems.
/// The other two arguments are currently unused and reserved for future use.
///
/// ## With U-Boot
///
/// This runtime supports being booted by U-Boot. In this case, the entry point function
/// must have the signature `[unsafe] fn([c_int[, *const *const c_char]]) -> !`, where the first argument
/// corresponds to the `argc` parameter and the second argument corresponds to the `argv` parameter passed by U-Boot.
///
/// Remember to enable the `u-boot` feature in the `riscv-rt` crate to use this functionality.
///
/// # IMPORTANT
///
/// This attribute can appear at most *once* in the dependency graph.
///
/// The entry point will be called by the reset handler. The program can't reference to the entry
/// point, much less invoke it.
///
/// # Examples
///
/// ``` no_run
/// #[riscv_macros::entry]
/// fn main() -> ! {
///     loop {
///         /* .. */
///     }
/// }
/// ```
#[cfg(feature = "riscv-rt")]
#[proc_macro_attribute]
pub fn entry(args: TokenStream, input: TokenStream) -> TokenStream {
    riscv_rt::Fn::entry(args, input)
}
