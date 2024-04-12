#![deny(warnings)]

extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate core;
extern crate proc_macro2;
#[macro_use]
extern crate syn;

use proc_macro2::Span;
use syn::{
    parse::{self, Parse},
    spanned::Spanned,
    FnArg, ItemFn, LitInt, LitStr, PathArguments, ReturnType, Type, Visibility,
};

use proc_macro::TokenStream;

/// Attribute to declare the entry point of the program
///
/// **IMPORTANT**: This attribute must appear exactly *once* in the dependency graph. Also, if you
/// are using Rust 1.30 the attribute must be used on a reachable item (i.e. there must be no
/// private modules between the item and the root of the crate); if the item is in the root of the
/// crate you'll be fine. This reachability restriction doesn't apply to Rust 1.31 and newer releases.
///
/// The specified function will be called by the reset handler *after* RAM has been initialized.
/// If present, the FPU will also be enabled before the function is called.
///
/// The type of the specified function must be `[unsafe] fn() -> !` (never ending function)
///
/// # Properties
///
/// The entry point will be called by the reset handler. The program can't reference to the entry
/// point, much less invoke it.
///
/// # Examples
///
/// - Simple entry point
///
/// ``` no_run
/// # #![no_main]
/// # use riscv_rt_macros::entry;
/// #[entry]
/// fn main() -> ! {
///     loop {
///         /* .. */
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn entry(args: TokenStream, input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as ItemFn);

    // check the function arguments
    if f.sig.inputs.len() > 3 {
        return parse::Error::new(
            f.sig.inputs.last().unwrap().span(),
            "`#[entry]` function has too many arguments",
        )
        .to_compile_error()
        .into();
    }
    for arg in &f.sig.inputs {
        match arg {
            FnArg::Receiver(_) => {
                return parse::Error::new(arg.span(), "invalid argument")
                    .to_compile_error()
                    .into();
            }
            FnArg::Typed(t) => {
                if !is_simple_type(&t.ty, "usize") {
                    return parse::Error::new(t.ty.span(), "argument type must be usize")
                        .to_compile_error()
                        .into();
                }
            }
        }
    }

    // check the function signature
    let valid_signature = f.sig.constness.is_none()
        && f.sig.asyncness.is_none()
        && f.vis == Visibility::Inherited
        && f.sig.abi.is_none()
        && f.sig.generics.params.is_empty()
        && f.sig.generics.where_clause.is_none()
        && f.sig.variadic.is_none()
        && match f.sig.output {
            ReturnType::Default => false,
            ReturnType::Type(_, ref ty) => matches!(**ty, Type::Never(_)),
        };

    if !valid_signature {
        return parse::Error::new(
            f.span(),
            "`#[entry]` function must have signature `[unsafe] fn([arg0: usize, ...]) -> !`",
        )
        .to_compile_error()
        .into();
    }

    if !args.is_empty() {
        return parse::Error::new(Span::call_site(), "This attribute accepts no arguments")
            .to_compile_error()
            .into();
    }

    // XXX should we blacklist other attributes?
    let attrs = f.attrs;
    let unsafety = f.sig.unsafety;
    let args = f.sig.inputs;
    let stmts = f.block.stmts;

    quote!(
        #[allow(non_snake_case)]
        #[export_name = "main"]
        #(#attrs)*
        pub #unsafety fn __risc_v_rt__main(#args) -> ! {
            #(#stmts)*
        }
    )
    .into()
}

#[allow(unused)]
fn is_simple_type(ty: &Type, name: &str) -> bool {
    if let Type::Path(p) = ty {
        if p.qself.is_none() && p.path.leading_colon.is_none() && p.path.segments.len() == 1 {
            let segment = p.path.segments.first().unwrap();
            if segment.ident == name && segment.arguments == PathArguments::None {
                return true;
            }
        }
    }
    false
}

/// Attribute to mark which function will be called at the beginning of the reset handler.
///
/// **IMPORTANT**: This attribute can appear at most *once* in the dependency graph. Also, if you
/// are using Rust 1.30 the attribute must be used on a reachable item (i.e. there must be no
/// private modules between the item and the root of the crate); if the item is in the root of the
/// crate you'll be fine. This reachability restriction doesn't apply to Rust 1.31 and newer
/// releases.
///
/// The function must have the signature of `unsafe fn()`.
///
/// The function passed will be called before static variables are initialized. Any access of static
/// variables will result in undefined behavior.
///
/// # Examples
///
/// ```
/// # use riscv_rt_macros::pre_init;
/// #[pre_init]
/// unsafe fn before_main() {
///     // do something here
/// }
///
/// # fn main() {}
/// ```
#[proc_macro_attribute]
pub fn pre_init(args: TokenStream, input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as ItemFn);

    // check the function signature
    let valid_signature = f.sig.constness.is_none()
        && f.sig.asyncness.is_none()
        && f.vis == Visibility::Inherited
        && f.sig.unsafety.is_some()
        && f.sig.abi.is_none()
        && f.sig.inputs.is_empty()
        && f.sig.generics.params.is_empty()
        && f.sig.generics.where_clause.is_none()
        && f.sig.variadic.is_none()
        && match f.sig.output {
            ReturnType::Default => true,
            ReturnType::Type(_, ref ty) => match **ty {
                Type::Tuple(ref tuple) => tuple.elems.is_empty(),
                _ => false,
            },
        };

    if !valid_signature {
        return parse::Error::new(
            f.span(),
            "`#[pre_init]` function must have signature `unsafe fn()`",
        )
        .to_compile_error()
        .into();
    }

    if !args.is_empty() {
        return parse::Error::new(Span::call_site(), "This attribute accepts no arguments")
            .to_compile_error()
            .into();
    }

    // XXX should we blacklist other attributes?
    let attrs = f.attrs;
    let ident = f.sig.ident;
    let block = f.block;

    quote!(
        #[export_name = "__pre_init"]
        #(#attrs)*
        pub unsafe fn #ident() #block
    )
    .into()
}

struct AsmLoopArgs {
    asm_template: String,
    count_from: usize,
    count_to: usize,
}

impl Parse for AsmLoopArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let template: LitStr = input.parse().unwrap();
        _ = input.parse::<Token![,]>().unwrap();
        let count: LitInt = input.parse().unwrap();
        if input.parse::<Token![,]>().is_ok() {
            let count_to: LitInt = input.parse().unwrap();
            Ok(Self {
                asm_template: template.value(),
                count_from: count.base10_parse().unwrap(),
                count_to: count_to.base10_parse().unwrap(),
            })
        } else {
            Ok(Self {
                asm_template: template.value(),
                count_from: 0,
                count_to: count.base10_parse().unwrap(),
            })
        }
    }
}

/// Loops an asm expression n times.
///
/// `loop_asm!` takes 2 or 3 arguments, the first is a string literal and the rest are a number literal
/// See [the formatting syntax documentation in `std::fmt`](../std/fmt/index.html) for details.
///
/// Argument 1 is an assembly expression, all "{}" in this assembly expression will be replaced with the
/// current loop index.
///
/// If 2 arguments are provided, the loop will start at 0 and end at the number provided in argument 2.
///
/// If 3 arguments are provided, the loop will start at the number provided in argument 2 and end at
/// the number provided in argument 3.
///
/// # Examples
///
/// ```
/// # use riscv_rt_macros::loop_asm;
/// unsafe {
///     loop_asm!("fmv.w.x f{}, x0", 32); // => core::arch::asm!("fmv.w.x f0, x0") ... core::arch::asm!("fmv.w.x f31, x0")
///     loop_asm!("fmv.w.x f{}, x0", 1, 32); // => core::arch::asm!("fmv.w.x f1, x0") ... core::arch::asm!("fmv.w.x f31, x0")
/// }
/// ```
#[proc_macro]
pub fn loop_asm(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as AsmLoopArgs);

    let tokens = (args.count_from..args.count_to)
        .map(|i| {
            let i = i.to_string();
            let asm = args.asm_template.replace("{}", &i);
            format!("core::arch::asm!(\"{}\");", asm)
        })
        .collect::<Vec<String>>()
        .join("\n");
    tokens.parse().unwrap()
}

/// Loops a global_asm expression n times.
///
/// `loop_global_asm!` takes 2 or 3 arguments, the first is a string literal and the rest are a number literal
/// See [the formatting syntax documentation in `std::fmt`](../std/fmt/index.html) for details.
///
/// Argument 1 is an assembly expression, all "{}" in this assembly expression will be replaced with the
/// current loop index.
///
/// If 2 arguments are provided, the loop will start at 0 and end at the number provided in argument 2.
///
/// If 3 arguments are provided, the loop will start at the number provided in argument 2 and end at
/// the number provided in argument 3.
///
/// # Examples
///
/// ```
/// # use riscv_rt_macros::loop_global_asm;
/// unsafe {
///     loop_global_asm!("fmv.w.x f{}, x0", 32); // => core::arch::global_asm!("fmv.w.x f0, x0") ... core::arch::global_asm!("fmv.w.x f31, x0")
///     loop_global_asm!("fmv.w.x f{}, x0", 1, 32); // => core::arch::global_asm!("fmv.w.x f1, x0") ... core::arch::global_asm!("fmv.w.x f31, x0")
/// }
/// ```
#[proc_macro]
pub fn loop_global_asm(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as AsmLoopArgs);

    let instructions = (args.count_from..args.count_to)
        .map(|i| {
            let i = i.to_string();
            args.asm_template.replace("{}", &i)
        })
        .collect::<Vec<String>>()
        .join("\n");

    let res = format!("core::arch::global_asm!(\n\"{}\"\n);", instructions);
    res.parse().unwrap()
}

enum RiscvArch {
    Rv32,
    Rv64,
}

#[proc_macro_attribute]
pub fn interrupt_riscv32(args: TokenStream, input: TokenStream) -> TokenStream {
    interrupt(args, input, RiscvArch::Rv32)
}

#[proc_macro_attribute]
pub fn interrupt_riscv64(args: TokenStream, input: TokenStream) -> TokenStream {
    interrupt(args, input, RiscvArch::Rv64)
}

fn interrupt(args: TokenStream, input: TokenStream, _arch: RiscvArch) -> TokenStream {
    let f = parse_macro_input!(input as ItemFn);

    // check the function arguments
    if !f.sig.inputs.is_empty() {
        return parse::Error::new(
            f.sig.inputs.first().unwrap().span(),
            "`#[interrupt]` function should not have arguments",
        )
        .to_compile_error()
        .into();
    }

    // check the function signature
    let valid_signature = f.sig.constness.is_none()
        && f.sig.asyncness.is_none()
        && f.vis == Visibility::Inherited
        && f.sig.abi.is_none()
        && f.sig.generics.params.is_empty()
        && f.sig.generics.where_clause.is_none()
        && f.sig.variadic.is_none()
        && match f.sig.output {
            ReturnType::Default => true,
            ReturnType::Type(_, ref ty) => matches!(**ty, Type::Never(_)),
        };

    if !valid_signature {
        return parse::Error::new(
            f.span(),
            "`#[interrupt]` function must have signature `[unsafe] fn() [-> !]`",
        )
        .to_compile_error()
        .into();
    }

    if !args.is_empty() {
        return parse::Error::new(Span::call_site(), "This attribute accepts no arguments")
            .to_compile_error()
            .into();
    }

    // XXX should we blacklist other attributes?
    let ident = &f.sig.ident;
    let export_name = format!("{:#}", ident);

    #[cfg(not(feature = "v-trap"))]
    let start_trap = proc_macro2::TokenStream::new();
    #[cfg(feature = "v-trap")]
    let start_trap = v_trap::start_interrupt_trap_asm(ident, _arch);

    quote!(
        #start_trap
        #[export_name = #export_name]
        #f
    )
    .into()
}

#[cfg(feature = "v-trap")]
mod v_trap {
    use super::*;

    const TRAP_SIZE: usize = 16;

    #[rustfmt::skip]
    const TRAP_FRAME: [&str; TRAP_SIZE] = [
        "ra",
        "t0",
        "t1",
        "t2",
        "t3",
        "t4",
        "t5",
        "t6",
        "a0",
        "a1",
        "a2",
        "a3",
        "a4",
        "a5",
        "a6",
        "a7",
    ];

    pub(crate) fn start_interrupt_trap_asm(
        ident: &syn::Ident,
        arch: RiscvArch,
    ) -> proc_macro2::TokenStream {
        let function = ident.to_string();
        let (width, store, load) = match arch {
            RiscvArch::Rv32 => (4, "sw", "lw"),
            RiscvArch::Rv64 => (8, "sd", "ld"),
        };

        let (mut stores, mut loads) = (Vec::new(), Vec::new());
        for (i, r) in TRAP_FRAME.iter().enumerate() {
            stores.push(format!("        {store} {r}, {i}*{width}(sp)"));
            loads.push(format!("        {load} {r}, {i}*{width}(sp)"));
        }
        let store = stores.join("\n");
        let load = loads.join("\n");

        #[cfg(feature = "s-mode")]
        let ret = "sret";
        #[cfg(not(feature = "s-mode"))]
        let ret = "mret";

        let instructions = format!(
            "
core::arch::global_asm!(
    \".section .trap, \\\"ax\\\"
    .align {width}
    .global _start_{function}_trap
    _start_{function}_trap:
        addi sp, sp, - {TRAP_SIZE} * {width}
{store}
        call {function}
{load}
        addi sp, sp, {TRAP_SIZE} * {width}
        {ret}\"
);"
        );

        instructions.parse().unwrap()
    }
}
