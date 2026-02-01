#[cfg(feature = "syn")]
pub use syn;

#[cfg(feature = "quote")]
pub use quote;

pub use proc_macro2;

pub mod prelude {
    #[cfg(feature = "syn")]
    pub use syn::*;

    #[cfg(feature = "quote")]
    pub use quote::{format_ident, quote, quote_spanned};

    pub use proc_macro2::TokenStream;

    pub use crate::macros;
}

pub trait MacroError {
    fn into_compiler_error(self) -> proc_macro2::TokenStream;
}

#[cfg(feature = "syn")]
impl MacroError for syn::Error {
    fn into_compiler_error(self) -> proc_macro2::TokenStream {
        self.into_compile_error()
    }
}

#[macro_export]
macro_rules! wrap {
    ($impl_name:path: $($item:expr),* $(,)?) => {
        ::std::convert::Into::into(
            $impl_name($(::std::convert::Into::into($item),)*)
                .unwrap_or_else($crate::MacroError::into_compiler_error),
        )
    };
}

#[macro_export]
macro_rules! r#macro {
    (fn $name:ident => $inner:path) => {
        #[proc_macro]
        pub fn $name(item: ::proc_macro::TokenStream) -> ::proc_macro::TokenStream {
            $crate::wrap!($inner: item)
        }
    };
    (attr $name:ident => $inner:path) => {
        #[proc_macro]
        pub fn $name(args: ::proc_macro::TokenStream, item: ::proc_macro::TokenStream) -> ::proc_macro::TokenStream {
            $crate::wrap!($inner: args, item)
        }
    };
}

#[macro_export]
macro_rules! macros {
    ($($kind:tt $name:ident => $inner:path;)*) => {$(
        $crate::r#macro!{$kind $name => $inner}
    )*};
}
