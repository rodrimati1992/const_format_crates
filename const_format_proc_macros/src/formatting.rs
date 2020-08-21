use proc_macro2::{Span, TokenStream as TokenStream2};

use quote::quote;

use syn::Ident;

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) enum Formatting {
    Debug(FormattingMode),
    Display,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) enum FormattingMode {
    Regular,
    Hexadecimal,
    Binary,
}

impl FormattingMode {
    pub(crate) fn is_regular(self) -> bool {
        matches!(self, FormattingMode::Regular)
    }

    pub(crate) fn tokens(self, root_mod: &TokenStream2) -> TokenStream2 {
        match self {
            Self::Regular => quote!(#root_mod::pmr::FormattingMode::Regular),
            Self::Hexadecimal => quote!(#root_mod::pmr::FormattingMode::Hexadecimal),
            Self::Binary => quote!(#root_mod::pmr::FormattingMode::Binary),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) enum IsAlternate {
    Yes,
    No,
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct FormattingFlags {
    pub(crate) formatting: Formatting,
    pub(crate) is_alternate: IsAlternate,
}

impl FormattingFlags {
    #[inline]
    pub(crate) fn display(is_alternate: IsAlternate) -> Self {
        Self {
            formatting: Formatting::Display,
            is_alternate,
        }
    }

    #[inline]
    pub(crate) fn debug(mode: FormattingMode, is_alternate: IsAlternate) -> Self {
        Self {
            formatting: Formatting::Debug(mode),
            is_alternate,
        }
    }
}

impl FormattingFlags {
    pub(crate) fn to_pargument_method_name(self) -> Ident {
        let name = match self.formatting {
            Formatting::Display => "to_pargument_display",
            Formatting::Debug { .. } => "to_pargument_debug",
        };

        Ident::new(name, Span::mixed_site())
    }

    #[allow(dead_code)]
    pub(crate) fn fmt_method_name(self) -> Ident {
        let name = match self.formatting {
            Formatting::Display => "const_display_fmt",
            Formatting::Debug { .. } => "const_debug_fmt",
        };

        Ident::new(name, Span::mixed_site())
    }

    #[allow(dead_code)]
    pub(crate) fn len_method_name(self) -> Ident {
        let name = match self.formatting {
            Formatting::Display => "const_display_len",
            Formatting::Debug { .. } => "const_debug_len",
        };

        Ident::new(name, Span::mixed_site())
    }

    pub(crate) fn tokens(self, root_mod: &TokenStream2) -> TokenStream2 {
        let is_alternate_tokens = match self.is_alternate {
            IsAlternate::Yes => quote!(#root_mod::pmr::IsAlternate::Yes),
            IsAlternate::No => quote!(#root_mod::pmr::IsAlternate::No),
        };

        match self.formatting {
            Formatting::Display => quote!(
                #root_mod::pmr::FormattingFlags::display(
                    #is_alternate_tokens,
                )
            ),
            Formatting::Debug(mode) => {
                let mode_tokens = mode.tokens(root_mod);
                quote!(
                    #root_mod::pmr::FormattingFlags::debug(
                        #mode_tokens,
                        #is_alternate_tokens,
                    )
                )
            }
        }
    }
}
