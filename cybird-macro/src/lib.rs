use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemStruct, parse_macro_input};
// Alternative: An attribute macro that adds the include automatically
#[proc_macro_attribute]
pub fn plugin(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);

    // Keep the original struct and add the include for generated exports
    let expanded = quote! {
        #input

        // Include the build script generated C exports
        include!(concat!(env!("OUT_DIR"), "/auto_exports.rs"));
    };

    TokenStream::from(expanded)
}
