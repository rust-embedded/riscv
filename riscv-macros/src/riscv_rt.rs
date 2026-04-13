use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, spanned::Spanned, token::Comma, Error, FnArg, Ident,
    ItemFn, Path, Result, ReturnType, Type, Visibility,
};

pub mod asm;

/// Convenience struct to represent a trap handler
pub struct Trap {
    /// Path to trap variant. Must implement the appropriate `riscv-types` trait.
    path: syn::Path,
    /// Type of trap handler
    ty: TrapType,
}

/// Enum representing the type of trap handler
pub enum TrapType {
    Exception,
    CoreInterrupt,
    ExternalInterrupt,
}

impl TrapType {
    /// Get the trait that must be implemented by the corresponding trap
    fn impl_trait(&self) -> &str {
        match self {
            Self::Exception => "riscv_rt::ExceptionNumber",
            Self::ExternalInterrupt => "riscv_rt::ExternalInterruptNumber",
            Self::CoreInterrupt => "riscv_rt::CoreInterruptNumber",
        }
    }
}

/// Enum representing the supported runtime function attributes
pub enum Fn {
    PostInit,
    SetupInterrupts,
    Entry,
    Trap(Trap),
}

impl Fn {
    /// Convenience method to generate the token stream for the `post_init` attribute
    pub fn post_init(args: TokenStream, input: TokenStream) -> TokenStream {
        let errors = Self::PostInit.check_args_empty(args).err();
        Self::PostInit.quote_fn(input, errors)
    }

    /// Convenience method to generate the token stream for the `setup_interrupts` attribute
    pub fn setup_interrupts(args: TokenStream, input: TokenStream) -> TokenStream {
        let errors = Self::SetupInterrupts.check_args_empty(args).err();
        Self::SetupInterrupts.quote_fn(input, errors)
    }

    /// Convenience method to generate the token stream for the `entry` attribute
    pub fn entry(args: TokenStream, input: TokenStream) -> TokenStream {
        let errors = Self::Entry.check_args_empty(args).err();
        Self::Entry.quote_fn(input, errors)
    }

    /// Convenience method to generate the token stream for trap handler attributes
    pub fn trap(args: TokenStream, input: TokenStream, ty: TrapType) -> TokenStream {
        let (path, errors) = match syn::parse::<Path>(args) {
            Ok(path) => (path, None),
            Err(e) => {
                let path = syn::parse_str("invalid").unwrap();
                let impl_trait = ty.impl_trait();
                let err = Error::new(
                    e.span(),
                    format!("attribute expects a path to a variant of an enum that implements the `{impl_trait}` trait"),
                );
                (path, Some(err))
            }
        };
        let trap = Trap { path, ty };
        Self::Trap(trap).quote_fn(input, errors)
    }

    /// Generate the token stream for the function with the given attribute
    fn quote_fn(&self, item: TokenStream, mut errors: Option<Error>) -> TokenStream {
        let mut func = parse_macro_input!(item as ItemFn);

        self.check_fn(&func, &mut errors);
        let extras = self.add_extras(&mut func, &mut errors);
        let export_name = self.export_name(&func);
        let link_section = self.link_section(&func);

        let tokens = match errors {
            Some(err) => err.to_compile_error(),
            None => quote! {
                #export_name
                #link_section
                #func
                #extras
            },
        };
        tokens.into()
    }

    /// Check if the function signature is valid for the given attribute
    fn check_fn(&self, f: &ItemFn, errors: &mut Option<Error>) {
        // First, check that the function is private
        if f.vis != Visibility::Inherited {
            combine_err(errors, Error::new(f.vis.span(), "function must be private"));
        }

        let sig = &f.sig;
        // Next, check common aspects of the signature individually to accumulate errors
        if let Some(constness) = sig.constness {
            let span = constness.span();
            combine_err(errors, Error::new(span, "function must not be const"));
        }
        if let Some(asyncness) = sig.asyncness {
            let span = asyncness.span();
            combine_err(errors, Error::new(span, "function must not be async"));
        }
        if let Some(abi) = &sig.abi {
            combine_err(errors, Error::new(abi.span(), "ABI must not be specified"));
        }
        if !sig.generics.params.is_empty() {
            let span = sig.generics.params.span();
            combine_err(errors, Error::new(span, "generics are not allowed"));
        }

        // Check input parameters...
        self.check_inputs(&sig.inputs, errors);
        // ... and variadic arguments (they are at the end of input parameters)
        if let Some(variadic) = &sig.variadic {
            combine_err(
                errors,
                Error::new(variadic.span(), "variadic arguments are not allowed"),
            );
        }

        // Check output type...
        self.check_output(&sig.output, errors);
        // ... and where clause (they are after output type)
        if let Some(where_clause) = &sig.generics.where_clause {
            let span = where_clause.span();
            combine_err(errors, Error::new(span, "where clause is not allowed"));
        }
    }

    /// Check if the function has valid input arguments for the given attribute
    fn check_inputs(&self, inputs: &Punctuated<FnArg, Comma>, errors: &mut Option<Error>) {
        // Use this match to specify expected input arguments for different functions in the future
        match self {
            Self::PostInit | Self::SetupInterrupts => {
                #[cfg(not(feature = "rvrt-u-boot"))]
                self.check_fn_args(inputs, &["usize"], errors);
                #[cfg(feature = "rvrt-u-boot")]
                self.check_fn_args(inputs, &[], errors);
            }
            #[cfg(not(feature = "rvrt-u-boot"))]
            Self::Entry => self.check_fn_args(inputs, &["usize", "usize", "usize"], errors),
            #[cfg(feature = "rvrt-u-boot")]
            Self::Entry => self.check_fn_args(inputs, &["c_int", "*const *const c_char"], errors),
            Self::Trap(Trap { ty, .. }) => match ty {
                TrapType::Exception => {
                    self.check_fn_args(inputs, &["&riscv_rt::TrapFrame"], errors)
                }
                TrapType::CoreInterrupt | TrapType::ExternalInterrupt => {
                    self.check_fn_args(inputs, &[], errors)
                }
            },
        }
    }

    /// Check if the function has a valid output type for the given attribute
    fn check_output(&self, output: &ReturnType, errors: &mut Option<Error>) {
        // Use this match to specify expected output types for different functions in the future
        match self {
            Self::PostInit | Self::SetupInterrupts => check_output_empty(output, errors),
            Self::Entry => check_output_never(output, errors),
            Self::Trap(_) => check_output_empty_or_never(output, errors),
        }
    }

    /// Additional items to append to the function for the given attribute
    fn add_extras(&self, func: &mut ItemFn, errors: &mut Option<Error>) -> Option<TokenStream2> {
        // Append to function name the prefix __riscv_rt_ (to prevent users from calling it directly)
        func.sig.ident = Ident::new(
            &format!("__riscv_rt_{}", func.sig.ident),
            func.sig.ident.span(),
        );

        // Use this match to specify extra items for different functions in the future
        match self {
            Self::PostInit | Self::SetupInterrupts | Self::Entry => None,
            Self::Trap(Trap { path, ty }) => {
                let mut extras = vec![];

                // Set ABI to extern "C"
                func.sig.abi = Some(syn::parse(quote! { extern "C" }.into()).unwrap());

                // Compile-time check to ensure the trap path implements the trap trait
                let impl_trait = format!("::{}", ty.impl_trait());
                let impl_trait: Path = syn::parse_str(&impl_trait).unwrap();

                extras.push(quote! {

                   const _: fn() = || {
                       fn assert_impl<T: #impl_trait>(_arg: T) {}
                       assert_impl(#path);
                   };
                });

                if cfg!(feature = "rt-v-trap") && matches!(ty, TrapType::CoreInterrupt) {
                    let interr_ident = &path.segments.last().unwrap().ident;
                    match asm::RiscvArch::try_from_env() {
                        Some(arch) => extras.push(arch.start_interrupt_trap(interr_ident)),
                        None => combine_err(errors, Error::new(
                            path.span(),
                            "RISCV_RT_BASE_ISA must be defined for core interrupt handlers when `v-trap` feature is enabled",
                        )),
                    }
                }

                Some(quote! { #(#extras)* })
            }
        }
    }

    /// The export name for the given attribute
    fn export_name(&self, _f: &ItemFn) -> Option<TokenStream2> {
        // Use this match to specify export names for different functions in the future
        let export_name = match self {
            Self::PostInit => Some("__post_init".to_string()),
            Self::SetupInterrupts => Some("_setup_interrupts".to_string()),
            Self::Entry => Some("main".to_string()),
            Self::Trap(Trap { path, .. }) => Some(path.segments.last().unwrap().ident.to_string()),
        };

        export_name.map(|name| match self {
            Self::PostInit | Self::SetupInterrupts | Self::Trap(_) => quote! {
                #[export_name = #name]
            },
            Self::Entry => quote! {
                // to avoid two main symbols when testing on host
                #[cfg_attr(any(target_arch = "riscv32", target_arch = "riscv64"), export_name = #name)]
            },
        })
    }

    /// The link section attribute for the given attribute (if any)
    fn link_section(&self, _f: &ItemFn) -> Option<TokenStream2> {
        // Use this match to specify section names for different functions in the future
        let section_name: Option<String> = match self {
            // TODO: check if we want specific sections for these functions
            Self::PostInit | Self::SetupInterrupts | Self::Entry | Self::Trap(_) => None,
        };

        section_name.map(|section| quote! {
            #[cfg_attr(any(target_arch = "riscv32", target_arch = "riscv64"), link_section = #section)]
        })
    }

    /// Check that no arguments were provided to the macro attribute
    fn check_args_empty(&self, args: TokenStream) -> Result<()> {
        if args.is_empty() {
            Ok(())
        } else {
            let args: TokenStream2 = args.into();
            Err(Error::new(args.span(), "macro arguments are not allowed"))
        }
    }

    /// Iterates through the input arguments and checks that their types match the expected types
    fn check_fn_args(
        &self,
        inputs: &Punctuated<FnArg, Comma>,
        expected_types: &[&str],
        errors: &mut Option<Error>,
    ) {
        let mut expected_iter = expected_types.iter();
        for arg in inputs.iter() {
            match expected_iter.next() {
                Some(expected) => {
                    if let Err(e) = check_arg_type(arg, expected) {
                        combine_err(errors, e);
                    }
                }
                None => {
                    combine_err(errors, Error::new(arg.span(), "too many input arguments"));
                }
            }
        }
    }
}

/// Combine a new error into an optional accumulator
fn combine_err(acc: &mut Option<Error>, err: Error) {
    match acc {
        Some(e) => e.combine(err),
        None => *acc = Some(err),
    }
}

/// Check if a function argument matches the expected type
fn check_arg_type(arg: &FnArg, expected: &str) -> Result<()> {
    match arg {
        FnArg::Typed(argument) => {
            if !is_correct_type(&argument.ty, expected) {
                Err(Error::new(
                    argument.ty.span(),
                    format!("argument type must be `{expected}`"),
                ))
            } else {
                Ok(())
            }
        }
        FnArg::Receiver(_) => Err(Error::new(arg.span(), "invalid argument")),
    }
}

/// Check if a type matches the expected type name
fn is_correct_type(ty: &Type, expected: &str) -> bool {
    // Parse the expected type string into a Type. We strip the path to compare only the last segment.
    let mut correct = syn::parse_str(expected).unwrap();
    correct = strip_type_path(&correct).unwrap();

    if let Some(ty) = strip_type_path(ty) {
        ty == correct
    } else {
        false
    }
}

/// Strip the path of a type, returning only the last segment (e.g., `core::usize` -> `usize`)
fn strip_type_path(ty: &Type) -> Option<Type> {
    match ty {
        Type::Ptr(ty) => {
            let mut ty = ty.clone();
            *ty.elem = strip_type_path(&ty.elem)?;
            Some(Type::Ptr(ty))
        }
        Type::Reference(ty) => {
            let mut ty = ty.clone();
            // We ignore mutability when comparing reference types
            ty.mutability = None;
            ty.elem = Box::new(strip_type_path(&ty.elem)?);
            Some(Type::Reference(ty))
        }
        Type::Path(ty) => {
            let mut ty = ty.clone();
            let last_segment = ty.path.segments.last().unwrap().clone();
            ty.path.segments = Punctuated::new();
            ty.path.segments.push_value(last_segment);
            Some(Type::Path(ty))
        }
        _ => None,
    }
}

/// Make sure the output type is either `()` or absent
fn check_output_empty(output: &ReturnType, errors: &mut Option<Error>) {
    let is_valid = matches!(output, ReturnType::Default)
        || matches!(output, ReturnType::Type(_, ty) if matches!(**ty, Type::Tuple(ref tuple) if tuple.elems.is_empty()));

    if !is_valid {
        combine_err(errors, Error::new(output.span(), "return type must be ()"));
    }
}

/// Make sure the output type is `!` (never)
fn check_output_never(output: &ReturnType, errors: &mut Option<Error>) {
    if !matches!(output, ReturnType::Type(_, ty) if matches!(**ty, Type::Never(_))) {
        combine_err(errors, Error::new(output.span(), "return type must be !"));
    }
}

/// Make sure the output type is either `()`, `!` (never), or absent
fn check_output_empty_or_never(output: &ReturnType, errors: &mut Option<Error>) {
    let is_valid = matches!(output, ReturnType::Default)
        || matches!(output, ReturnType::Type(_, ty) if matches!(**ty, Type::Tuple(ref tuple) if tuple.elems.is_empty()))
        || matches!(output, ReturnType::Type(_, ty) if matches!(**ty, Type::Never(_)));

    if !is_valid {
        combine_err(
            errors,
            Error::new(output.span(), "return type must be () or !"),
        );
    }
}
