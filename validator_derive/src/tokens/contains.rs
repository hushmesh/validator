use quote::quote;

use crate::types::Contains;
use crate::utils::{quote_code, quote_message, CrateName};

pub fn contains_tokens(
    crate_name: &CrateName,
    contains: Contains,
    field_name: &proc_macro2::TokenStream,
    field_name_str: &str,
) -> proc_macro2::TokenStream {
    let p = contains.pattern;
    let cow_type = if cfg!(feature = "std") {
        quote!(::std::borrow::Cow::from("needle"))
    } else {
        quote!(::alloc::borrow::Cow::from("needle"))
    };
    let (needle, needle_err) = (
        quote!(#p),
        quote!(err.add_param(
            #cow_type,
            &#p);),
    );

    let message = quote_message(contains.message);
    let code = quote_code(crate_name, contains.code, "contains");
    let cow_type = if cfg!(feature = "std") {
        quote!(::std::borrow::Cow::from("value"))
    } else {
        quote!(::alloc::borrow::Cow::from("value"))
    };
    quote! {
        if !#field_name.validate_contains(#needle) {
            #code
            #message
            #needle_err
            err.add_param(
                #cow_type,
                &#field_name);
                errors.add(#field_name_str, err);
        }
    }
}
