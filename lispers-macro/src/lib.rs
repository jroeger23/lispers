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

struct Attrs {
    pub eval: bool,
    pub fname: Option<Ident>,
}

impl syn::parse::Parse for Attrs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let exprs = Punctuated::<FlagOrKV, Token![,]>::parse_terminated(input)?;

        let mut ret = Attrs {
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
    let attr = parse_macro_input!(attr as Attrs);

    // Extract argument conversion statements
    let mut conversion_statements = Vec::new();

    for arg in &sig.inputs {
        if let FnArg::Typed(PatType { pat, ty, .. }) = arg {
            if let Pat::Ident(ident) = pat.as_ref() {
                let arg_name = &ident.ident;
                let arg_name_str = arg_name.to_string();
                if attr.eval {
                    conversion_statements.push(quote! {
                        let #arg_name: #ty = eval(env, args_iter.next().ok_or(EvalError::ArgumentError(format!("Missing Argument {}", #arg_name_str)))?)?.try_into()?;
                    });
                } else {
                    conversion_statements.push(quote! {
                        let #arg_name: #ty = args_iter.next().ok_or(EvalError::ArgumentError(format!("Missing Argument {}", #arg_name_str)))?.try_into()?;
                    });
                }
            }
        }
    }

    let func_name = match attr.fname {
        Some(fname) => fname,
        None => func_name.clone(),
    };

    let gen = quote! {
        #vis fn #func_name(env: &Environment, expr: Expression) -> Result<Expression, EvalError> {
            let args: Vec<Expression> = expr.try_into()?;
            let mut args_iter = args.into_iter();

            #(#conversion_statements)*

            Ok((|| #ret #block)()?.into())
        }
    };

    let out: TokenStream = gen.into();
    print!("{}", out);
    out
}
