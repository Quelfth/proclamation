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

    pub use crate::wrap;
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
