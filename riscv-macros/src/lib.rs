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
///
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

/// Attribute to mark which function will set interrupts before jumping to the entry point.
///
/// The `riscv-rt` crates provides a default implementation that works for most cases.
/// If you want to provide your own implementation, you must enable the `custom-setup-interrupts`
/// feature in the `riscv-rt` crate and use this macro on your function.
/// The `riscv-rt` crate re-exports this macro if the `custom-setup-interrupts` feature is enabled,
/// so you can use it as `riscv_rt::setup_interrupts` without depending on `riscv-macros` directly.
///
/// The function must have the signature of `[unsafe] fn([usize])`, where the argument
/// corresponds to the hart ID of the current hart. This is useful for multi-hart systems
/// to perform hart-specific interrupt setup.
///
/// # IMPORTANT
///
/// This attribute can appear at most *once* in the dependency graph.
///
/// # Examples
///
/// ```
/// #[riscv_macros::setup_interrupts]
/// unsafe fn setup_interrupts(hart_id: usize) {
///     // do something here
/// }
/// ```
#[cfg(feature = "riscv-rt")]
#[proc_macro_attribute]
pub fn setup_interrupts(args: TokenStream, input: TokenStream) -> TokenStream {
    riscv_rt::Fn::setup_interrupts(args, input)
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

/// Attribute to declare an exception handler.
///
/// The function must have the signature `[unsafe] fn([&[mut] riscv_rt::TrapFrame]) [-> !]`.
///
/// The argument of the macro must be a path to a variant of an enum that implements the `riscv_rt::ExceptionNumber` trait.
///
/// # Example
///
/// ``` ignore,no_run
/// #[riscv_rt::exception(riscv::interrupt::Exception::LoadMisaligned)]
/// fn load_misaligned(trap_frame: &mut riscv_rt::TrapFrame) -> ! {
///     loop{};
/// }
/// ```
#[cfg(feature = "riscv-rt")]
#[proc_macro_attribute]
pub fn exception(args: TokenStream, input: TokenStream) -> TokenStream {
    riscv_rt::Fn::trap(args, input, riscv_rt::TrapType::Exception)
}

/// Attribute to declare a core interrupt handler.
///
/// The function must have the signature `[unsafe] fn() [-> !]`.
///
/// The argument of the macro must be a path to a variant of an enum that implements the `riscv_rt::CoreInterruptNumber` trait.
///
/// If the `v-trap` feature is enabled, this macro generates the corresponding interrupt trap handler in assembly.
/// This feature relies on the `RISCV_RT_BASE_ISA` environment variable being set to one of
/// `rv32i`, `rv32e`, `rv64i`, or `rv64e`. Otherwise, this will **panic**.
///
/// # Example
///
/// ``` ignore,no_run
/// #[riscv_rt::core_interrupt(riscv::interrupt::Interrupt::SupervisorSoft)]
/// fn supervisor_soft() -> ! {
///     loop{};
/// }
/// ```
#[cfg(feature = "riscv-rt")]
#[proc_macro_attribute]
pub fn core_interrupt(args: TokenStream, input: TokenStream) -> TokenStream {
    riscv_rt::Fn::trap(args, input, riscv_rt::TrapType::CoreInterrupt)
}

/// Attribute to declare an external interrupt handler.
///
/// The function must have the signature `[unsafe] fn() [-> !]`.
///
/// The argument of the macro must be a path to a variant of an enum that implements the `riscv_rt::ExternalInterruptNumber` trait.
///
/// # Example
///
/// ``` ignore,no_run
/// #[riscv_rt::external_interrupt(e310x::interrupt::Interrupt::GPIO0)]
/// fn gpio0() -> ! {
///     loop{};
/// }
/// ```
#[cfg(feature = "riscv-rt")]
#[proc_macro_attribute]
pub fn external_interrupt(args: TokenStream, input: TokenStream) -> TokenStream {
    riscv_rt::Fn::trap(args, input, riscv_rt::TrapType::ExternalInterrupt)
}

/// Temporary patch macro to deal with LLVM bug.
///
/// # Note
///
/// This macro is intended to be used internally by the `riscv-rt` crate. Do not use it directly in your code.
#[cfg(feature = "riscv-rt")]
#[proc_macro]
pub fn rvrt_llvm_arch_patch(_input: TokenStream) -> TokenStream {
    let q = if let Ok(arch) = std::env::var("RISCV_RT_LLVM_ARCH_PATCH") {
        let patch = format!(".attribute arch,\"{arch}\"");
        quote! { core::arch::global_asm!{#patch} }
    } else {
        quote!(compile_error!("RISCV_RT_LLVM_ARCH_PATCH is not set"))
    };
    q.into()
}

/// Generates assembly code required for the default handling of traps.
///
/// The main routine generated is `_default_start_trap`. If no `_start_trap` function
/// is defined, the linker will use this function as the default trap entry point.
///
/// If the `pre-default-start-trap` feature is enabled, the generated code will also
/// include a call to a user-defined function `_pre_default_start_trap` at the beginning
/// of the `_default_start_trap` routine.
///
/// If the `rt-v-trap` feature is enabled, the macro will also include the assembly code
/// for the `_start_DefaultInterrupt_trap` and `_continue_interrupt_trap` routines, which
/// are required for handling core interrupts in vectored trap mode.
///
/// # Note
///
/// This macro is intended to be used internally by the `riscv-rt` crate. Do not use it directly in your code.
#[cfg(feature = "riscv-rt")]
#[proc_macro]
pub fn rvrt_default_start_trap(_input: TokenStream) -> TokenStream {
    match riscv_rt::asm::RiscvArch::try_from_env() {
        Some(arch) => arch.default_start_trap().into(),
        None => quote! {
            compile_error!("RISCV_RT_BASE_ISA environment variable is not set or is invalid");
        }
        .into(),
    }
}
