use proc_macro2::{Span, TokenStream as TokenStream2};

use quote::quote_spanned;

use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Error {
    messages: Vec<CompileError>,
}

#[derive(Debug, Clone)]
enum CompileError {
    Basic {
        span: Span,
        msg: String,
    },
    #[cfg(feature = "derive")]
    Syn(TokenStream2),
}

impl Error {
    pub fn new<T: Display>(span: Span, msg: T) -> Self {
        Error {
            messages: vec![CompileError::Basic {
                span,
                msg: msg.to_string(),
            }],
        }
    }

    pub fn to_compile_error(&self) -> TokenStream2 {
        self.messages
            .iter()
            .map(|em| match em {
                CompileError::Basic { span, msg } => {
                    quote_spanned! (*span=> (compile_error!{#msg}) )
                }
                #[cfg(feature = "derive")]
                CompileError::Syn(x) => x.clone(),
            })
            .collect()
    }

    pub fn combine(&mut self, another: Error) {
        self.messages.extend(another.messages)
    }
}

#[cfg(feature = "derive")]
impl From<syn::Error> for Error {
    fn from(err: syn::Error) -> Self {
        Self {
            messages: vec![CompileError::Syn(err.to_compile_error())],
        }
    }
}
