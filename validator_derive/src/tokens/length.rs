use quote::quote;

use crate::types::Length;
use crate::utils::{quote_code, quote_message, CrateName};

pub fn length_tokens(
    crate_name: &CrateName,
    length: Length,
    field_name: &proc_macro2::TokenStream,
    field_name_str: &str,
) -> proc_macro2::TokenStream {
    let cow_type = if cfg!(feature = "std") {
        quote!(::std::borrow::Cow::from("min"))
    } else {
        quote!(::alloc::borrow::Cow::from("min"))
    };
    let (min, min_err) = if let Some(v) = length.min.as_ref() {
        (
            quote!(Some(#v)),
            quote!(err.add_param(
                #cow_type,
                &#v);),
        )
    } else {
        (quote!(None), quote!())
    };
    let cow_type = if cfg!(feature = "std") {
        quote!(::std::borrow::Cow::from("max"))
    } else {
        quote!(::alloc::borrow::Cow::from("max"))
    };
    let (max, max_err) = if let Some(v) = length.max {
        (
            quote!(Some(#v)),
            quote!(err.add_param(
            #cow_type,
            &#v);),
        )
    } else {
        (quote!(None), quote!())
    };
    let cow_type = if cfg!(feature = "std") {
        quote!(::std::borrow::Cow::from("equal"))
    } else {
        quote!(::alloc::borrow::Cow::from("equal"))
    };
    let (equal, equal_err) = if let Some(v) = length.equal {
        (
            quote!(Some(#v)),
            quote!(err.add_param(
                #cow_type,
                &#v);),
        )
    } else {
        (quote!(None), quote!())
    };

    let message = quote_message(length.message);
    let code = quote_code(crate_name, length.code, "length");

    let cow_type = if cfg!(feature = "std") {
        quote!(::std::borrow::Cow::from("value"))
    } else {
        quote!(::alloc::borrow::Cow::from("value"))
    };
    quote! {
        if !#field_name.validate_length(#min, #max, #equal) {
            #code
            #message
            #min_err
            #max_err
            #equal_err
            err.add_param(
                 #cow_type,
                 &#field_name);
            errors.add(#field_name_str, err);
        }
    }
}
