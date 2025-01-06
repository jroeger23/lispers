extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, FnArg, Ident, ItemFn, Pat, PatType, Token};

enum FlagOrKV {
    Flag(Ident),
    KV(Ident, Ident),
}

impl syn::parse::Parse for FlagOrKV {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        if input.peek(Token![=]) {
            input.parse::<Token![=]>()?;
            let value: Ident = input.parse()?;
            Ok(FlagOrKV::KV(ident, value))
        } else {
            Ok(FlagOrKV::Flag(ident))
        }
    }
}

struct NativeLispAttrs {
    pub eval: bool,
    pub fname: Option<Ident>,
}

impl syn::parse::Parse for NativeLispAttrs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let exprs = Punctuated::<FlagOrKV, Token![,]>::parse_terminated(input)?;

        let mut ret = NativeLispAttrs {
            eval: false,
            fname: None,
        };

        for e in exprs {
            match e {
                FlagOrKV::Flag(flag) => {
                    if flag.to_string() == "eval" {
                        ret.eval = true;
                    } else {
                        return Err(syn::Error::new_spanned(flag, "Unknown flag"));
                    }
                }
                FlagOrKV::KV(k, v) => {
                    if k.to_string() == "fname" {
                        ret.fname = Some(v);
                    } else {
                        return Err(syn::Error::new_spanned(k, "Unknown key"));
                    }
                }
            }
        }

        Ok(ret)
    }
}

struct NativeLispProxyAttrs {
    pub eval: bool,
    pub fname: Ident,
    pub dispatcher: Vec<Ident>,
}

impl syn::parse::Parse for NativeLispProxyAttrs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let exprs = Punctuated::<FlagOrKV, Token![,]>::parse_terminated(input)?;

        let mut ret = NativeLispProxyAttrs {
            eval: false,
            fname: Ident::new("proxy", proc_macro2::Span::call_site()),
            dispatcher: Vec::new(),
        };

        for e in exprs {
            match e {
                FlagOrKV::Flag(flag) => {
                    if flag.to_string() == "eval" {
                        ret.eval = true;
                    } else {
                        return Err(syn::Error::new_spanned(flag, "Unknown flag"));
                    }
                }
                FlagOrKV::KV(k, v) => {
                    if k.to_string() == "dispatch" {
                        ret.dispatcher.push(v);
                    } else if k.to_string() == "fname" {
                        ret.fname = v;
                    } else {
                        return Err(syn::Error::new_spanned(k, "Unknown key"));
                    }
                }
            }
        }

        Ok(ret)
    }
}

#[proc_macro_attribute]
pub fn native_lisp_function(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse function
    let input = parse_macro_input!(item as ItemFn);
    let vis = &input.vis;
    let sig = &input.sig;
    let func_name = &sig.ident;
    let block = &input.block;
    let ret = &sig.output;

    // Parse attrs
    let attr = parse_macro_input!(attr as NativeLispAttrs);

    // Extract argument conversion statements
    let mut conversion_statements = Vec::new();

    for arg in &sig.inputs {
        if let FnArg::Typed(PatType { pat, ty, .. }) = arg {
            if let Pat::Ident(ident) = pat.as_ref() {
                let arg_name_str = ident.ident.to_string();
                if attr.eval {
                    conversion_statements.push(quote! {
                        let #ident: #ty = eval(env, args_iter.next().ok_or(EvalError::ArgumentError(format!("Missing Argument {}", #arg_name_str)))?)?.try_into()?;
                    });
                } else {
                    conversion_statements.push(quote! {
                        let #ident: #ty = args_iter.next().ok_or(EvalError::ArgumentError(format!("Missing Argument {}", #arg_name_str)))?.try_into()?;
                    });
                }
            }
        }
    }

    let func_name = match attr.fname {
        Some(fname) => fname,
        None => func_name.clone(),
    };

    quote! {
        #vis fn #func_name(env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
            let args: Vec<Expression> = expr.try_into()?;
            let mut args_iter = args.into_iter();

            #(#conversion_statements)*

            Ok((|| #ret #block)()?.into())
        }
    }
    .into()
}

#[proc_macro]
pub fn native_lisp_function_proxy(item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(item as NativeLispProxyAttrs);
    let fname = &args.fname;

    let eval_statement = if args.eval {
        quote! {
            let exprs: Vec<Expression> = expr.try_into()?;
            let exprs = exprs.into_iter().map(|expr| eval(env, expr)).collect::<Result<Vec<Expression>, EvalError>>()?;
            let expr: Expression = exprs.into();
        }
    } else {
        quote! {}
    };

    let try_apply_statements = args
        .dispatcher
        .iter()
        .map(|impl_name| {
            quote! {
                match #impl_name(env, expr.clone()) {
                    Err(EvalError::ArgumentError(e)) => {/*Pass*/},
                    Err(EvalError::TypeError(e)) => {/*Pass*/},
                    x => return x,
                }
            }
        })
        .collect::<Vec<_>>();

    quote! {
        fn #fname(env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
            #eval_statement

            #(#try_apply_statements)*

            Err(EvalError::TypeError("No applicable method found".to_string()))
        }
    }
    .into()
}
