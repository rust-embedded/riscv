use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, spanned::Spanned, token::Comma, Error, FnArg, Ident,
    ItemFn, Result, ReturnType, Type, Visibility,
};

/// Enum representing the supported runtime function attributes
pub enum Fn {
    Entry,
    PostInit,
}

impl Fn {
    /// Convenience method to generate the token stream for the `entry` attribute
    pub fn entry(args: TokenStream, input: TokenStream) -> TokenStream {
        match Self::Entry.check_args_empty(args) {
            Ok(_) => Self::Entry.quote_fn(input),
            Err(e) => e.to_compile_error().into(),
        }
    }

    /// Convenience method to generate the token stream for the `post_init` attribute
    pub fn post_init(args: TokenStream, input: TokenStream) -> TokenStream {
        match Self::PostInit.check_args_empty(args) {
            Ok(_) => Self::PostInit.quote_fn(input),
            Err(e) => e.to_compile_error().into(),
        }
    }

    /// Generate the token stream for the function with the given attribute
    fn quote_fn(&self, item: TokenStream) -> TokenStream {
        let mut func = parse_macro_input!(item as ItemFn);

        if let Err(e) = self.check_fn(&func) {
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
    fn check_fn(&self, f: &ItemFn) -> Result<()> {
        // First, check that the function is private
        if f.vis != Visibility::Inherited {
            let attr = self.attr_name();
            return Err(Error::new(
                f.vis.span(),
                format!("`#[{attr}]` function must be private"),
            ));
        }
        let sig = &f.sig;

        // Next, check common aspects of the signature (constness, asyncness, generics, etc.)
        let valid_signature = sig.constness.is_none()
            && sig.asyncness.is_none()
            && sig.abi.is_none()
            && sig.generics.params.is_empty()
            && sig.generics.where_clause.is_none()
            && sig.variadic.is_none();
        if !valid_signature {
            let attr = self.attr_name();
            let expected = self.expected_signature();
            return Err(Error::new(
                sig.span(),
                format!("`#[{attr}]` function signature must be `{expected}`"),
            ));
        }

        // Finally, check that input arguments and output type are valid
        self.check_inputs(&sig.inputs)?;
        self.check_output(&sig.output)
    }

    /// Utility method for printing attribute name in error messages
    const fn attr_name(&self) -> &'static str {
        // Use this match to specify attribute names for different functions in the future
        match self {
            Self::Entry => "entry",
            Self::PostInit => "post_init",
        }
    }

    /// Utility method for printing expected function signature in error messages
    const fn expected_signature(&self) -> &'static str {
        // Use this match to specify expected signatures for different functions in the future
        match self {
            #[cfg(not(feature = "u-boot"))]
            Self::Entry => "[unsafe] fn([usize[, usize[, usize]]]) -> !",
            #[cfg(feature = "u-boot")]
            Self::Entry => "[unsafe] fn([c_int[, *const *const c_char]]) -> !",
            Self::PostInit => "[unsafe] fn([usize])",
        }
    }

    /// Check if the function has valid input arguments for the given attribute
    fn check_inputs(&self, inputs: &Punctuated<FnArg, Comma>) -> Result<()> {
        // Use this match to specify expected input arguments for different functions in the future
        match self {
            #[cfg(not(feature = "u-boot"))]
            Self::Entry => self.check_fn_args(inputs, &["usize", "usize", "usize"]),
            #[cfg(feature = "u-boot")]
            Self::Entry => self.check_fn_args(inputs, &["c_int", "*const *const c_char"]),
            Self::PostInit => self.check_fn_args(inputs, &["usize"]),
        }
    }

    /// Check if the function has a valid output type for the given attribute
    fn check_output(&self, output: &ReturnType) -> Result<()> {
        // Use this match to specify expected output types for different functions in the future
        match self {
            Self::Entry => check_output_never(output),
            Self::PostInit => check_output_empty(output),
        }
    }

    /// The export name for the given attribute
    fn export_name(&self, _f: &ItemFn) -> Option<TokenStream2> {
        // Use this match to specify export names for different functions in the future
        let export_name = match self {
            Self::Entry => Some("main".to_string()),
            Self::PostInit => Some("__post_init".to_string()),
        };

        export_name.map(|name| {
            quote! {
                #[cfg_attr(any(target_arch = "riscv32", target_arch = "riscv64"), export_name = #name)]
            }
        })
    }

    /// The link section attribute for the given attribute (if any)
    fn link_section(&self, _f: &ItemFn) -> Option<TokenStream2> {
        // Use this match to specify section names for different functions in the future
        let section_name: Option<String> = match self {
            Self::Entry | Self::PostInit => None,
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
            let attr = self.attr_name();
            Err(Error::new(
                args.span(),
                format!("`#[{attr}]` function does not accept any arguments"),
            ))
        }
    }

    /// Iterates through the input arguments and checks that their types match the expected types
    fn check_fn_args(
        &self,
        inputs: &Punctuated<FnArg, Comma>,
        expected_types: &[&str],
    ) -> Result<()> {
        let mut expected_iter = expected_types.iter();
        for arg in inputs.iter() {
            match expected_iter.next() {
                Some(expected) => check_arg_type(arg, expected)?,
                None => {
                    let attr = self.attr_name();
                    return Err(Error::new(
                        arg.span(),
                        format!("`#[{attr}]` function has too many input arguments"),
                    ));
                }
            }
        }
        Ok(())
    }
}

/// Check if a function argument matches the expected type
fn check_arg_type(arg: &FnArg, expected: &str) -> Result<()> {
    match arg {
        FnArg::Typed(argument) => {
            if !is_correct_type(&argument.ty, expected) {
                Err(Error::new(
                    argument.ty.span(),
                    format!("argument type must be {expected}"),
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
fn check_output_empty(output: &ReturnType) -> Result<()> {
    match output {
        ReturnType::Default => Ok(()),
        ReturnType::Type(_, ty) => match **ty {
            Type::Tuple(ref tuple) => {
                if tuple.elems.is_empty() {
                    Ok(())
                } else {
                    Err(Error::new(tuple.span(), "return type must be ()"))
                }
            }
            _ => Err(Error::new(ty.span(), "return type must be ()")),
        },
    }
}

/// Make sure the output type is `!` (never)
fn check_output_never(output: &ReturnType) -> Result<()> {
    match output {
        ReturnType::Type(_, ty) => match **ty {
            Type::Never(_) => Ok(()),
            _ => Err(Error::new(ty.span(), "return type must be !")),
        },
        ReturnType::Default => Err(Error::new(output.span(), "return type must be !")),
    }
}
