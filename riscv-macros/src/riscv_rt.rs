use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, spanned::Spanned, token::Comma, Error, FnArg, Ident,
    ItemFn, Result, ReturnType, Type, Visibility,
};

/// Enum representing the supported runtime function attributes
pub enum Fn {
    PostInit,
    Entry,
}

impl Fn {
    /// Convenience method to generate the token stream for the `post_init` attribute
    pub fn post_init(args: TokenStream, input: TokenStream) -> TokenStream {
        let errors = Self::PostInit.check_args_empty(args).err();
        Self::PostInit.quote_fn(input, errors)
    }

    /// Convenience method to generate the token stream for the `entry` attribute
    pub fn entry(args: TokenStream, input: TokenStream) -> TokenStream {
        let errors = Self::Entry.check_args_empty(args).err();
        Self::Entry.quote_fn(input, errors)
    }

    /// Generate the token stream for the function with the given attribute
    fn quote_fn(&self, item: TokenStream, errors: Option<Error>) -> TokenStream {
        let mut func = parse_macro_input!(item as ItemFn);

        if let Err(e) = self.check_fn(&func, errors) {
            return e.to_compile_error().into();
        }

        let export_name = self.export_name(&func);
        let link_section = self.link_section(&func);

        // Append to function name the prefix __riscv_rt_ (to prevent users from calling it directly)
        // Note that we do not change the export name, only the internal function name in the Rust code.
        func.sig.ident = Ident::new(
            &format!("__riscv_rt_{}", func.sig.ident),
            func.sig.ident.span(),
        );

        quote! {
            #export_name
            #link_section
            #func
        }
        .into()
    }

    /// Check if the function signature is valid for the given attribute
    fn check_fn(&self, f: &ItemFn, mut errors: Option<Error>) -> Result<()> {
        // First, check that the function is private
        if f.vis != Visibility::Inherited {
            combine_err(
                &mut errors,
                Error::new(f.vis.span(), "function must be private"),
            );
        }

        let sig = &f.sig;
        // Next, check common aspects of the signature individually to accumulate errors
        if let Some(constness) = sig.constness {
            combine_err(
                &mut errors,
                Error::new(constness.span(), "function must not be const"),
            );
        }
        if let Some(asyncness) = sig.asyncness {
            combine_err(
                &mut errors,
                Error::new(asyncness.span(), "function must not be async"),
            );
        }
        if let Some(abi) = &sig.abi {
            combine_err(
                &mut errors,
                Error::new(abi.span(), "ABI must not be specified"),
            );
        }
        if !sig.generics.params.is_empty() {
            // Use to_token_stream to get a span covering the entire <...> block
            let span = sig.generics.params.span();
            combine_err(&mut errors, Error::new(span, "generics are not allowed"));
        }

        // Check input parameters...
        self.check_inputs(&sig.inputs, &mut errors);
        // ... and variadic arguments (they are at the end of input parameters)
        if let Some(variadic) = &sig.variadic {
            combine_err(
                &mut errors,
                Error::new(variadic.span(), "variadic arguments are not allowed"),
            );
        }

        // Check output type...
        self.check_output(&sig.output, &mut errors);
        // ... and where clause (they are after output type)
        if let Some(where_clause) = &sig.generics.where_clause {
            combine_err(
                &mut errors,
                Error::new(where_clause.span(), "where clause is not allowed"),
            );
        }

        match errors {
            Some(e) => Err(e),
            None => Ok(()),
        }
    }

    /// Check if the function has valid input arguments for the given attribute
    fn check_inputs(&self, inputs: &Punctuated<FnArg, Comma>, errors: &mut Option<Error>) {
        // Use this match to specify expected input arguments for different functions in the future
        match self {
            Self::PostInit => self.check_fn_args(inputs, &["usize"], errors),
            #[cfg(not(feature = "u-boot"))]
            Self::Entry => self.check_fn_args(inputs, &["usize", "usize", "usize"], errors),
            #[cfg(feature = "u-boot")]
            Self::Entry => self.check_fn_args(inputs, &["c_int", "*const *const c_char"], errors),
        }
    }

    /// Check if the function has a valid output type for the given attribute
    fn check_output(&self, output: &ReturnType, errors: &mut Option<Error>) {
        // Use this match to specify expected output types for different functions in the future
        match self {
            Self::PostInit => check_output_empty(output, errors),
            Self::Entry => check_output_never(output, errors),
        }
    }

    /// The export name for the given attribute
    fn export_name(&self, _f: &ItemFn) -> Option<TokenStream2> {
        // Use this match to specify export names for different functions in the future
        let export_name = match self {
            Self::PostInit => Some("__post_init".to_string()),
            Self::Entry => Some("main".to_string()),
        };

        export_name.map(|name| match self {
            Self::Entry => quote! {
                // to avoid two main symbols when testing on host
                #[cfg_attr(any(target_arch = "riscv32", target_arch = "riscv64"), export_name = #name)]
            },
            _ => quote! {
                #[export_name = #name]
            },
        })
    }

    /// The link section attribute for the given attribute (if any)
    fn link_section(&self, _f: &ItemFn) -> Option<TokenStream2> {
        // Use this match to specify section names for different functions in the future
        let section_name: Option<String> = match self {
            Self::PostInit | Self::Entry => None,
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
    let correct: Type = syn::parse_str(expected).unwrap();
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
    match output {
        ReturnType::Default => {}
        ReturnType::Type(_, ty) => match **ty {
            Type::Tuple(ref tuple) => {
                if !tuple.elems.is_empty() {
                    combine_err(errors, Error::new(tuple.span(), "return type must be ()"));
                }
            }
            _ => combine_err(errors, Error::new(ty.span(), "return type must be ()")),
        },
    }
}

/// Make sure the output type is `!` (never)
fn check_output_never(output: &ReturnType, errors: &mut Option<Error>) {
    if !matches!(output, ReturnType::Type(_, ty) if matches!(**ty, Type::Never(_))) {
        combine_err(errors, Error::new(output.span(), "return type must be !"));
    }
}
