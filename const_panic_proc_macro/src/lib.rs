extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn const_panicking(_input: TokenStream) -> TokenStream {
    TokenStream::from(quote! {
        const E0: usize = [0][3];
    })
}
