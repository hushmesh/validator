use quote::quote;

use crate::types::Range;
use crate::utils::{quote_code, quote_message, CrateName};

pub fn range_tokens(
    crate_name: &CrateName,
    range: Range,
    field_name: &proc_macro2::TokenStream,
    field_name_str: &str,
) -> proc_macro2::TokenStream {
    let cow_type = if cfg!(feature = "std") {
        quote!(::std::borrow::Cow::from("min"))
    } else {
        quote!(::alloc::borrow::Cow::from("min"))
    };
    let (min, min_err) = if let Some(m) = range.min {
        (
            quote!(Some(#m)),
            quote!(err.add_param(
                #cow_type,
                &#m);),
        )
    } else {
        (quote!(None), quote!())
    };

    let cow_type = if cfg!(feature = "std") {
        quote!(::std::borrow::Cow::from("max"))
    } else {
        quote!(::alloc::borrow::Cow::from("max"))
    };
    let (max, max_err) = if let Some(m) = range.max {
        (
            quote!(Some(#m)),
            quote!(err.add_param(
                #cow_type,
                &#m);),
        )
    } else {
        (quote!(None), quote!())
    };

    let cow_type = if cfg!(feature = "std") {
        quote!(::std::borrow::Cow::from("exclusive_min"))
    } else {
        quote!(::alloc::borrow::Cow::from("exclusive_min"))
    };
    let (ex_min, ex_min_err) = if let Some(m) = range.exclusive_min {
        (
            quote!(Some(#m)),
            quote!(err.add_param(
                #cow_type,
                &#m);),
        )
    } else {
        (quote!(None), quote!())
    };

    let cow_type = if cfg!(feature = "std") {
        quote!(::std::borrow::Cow::from("exclusive_max"))
    } else {
        quote!(::alloc::borrow::Cow::from("exclusive_max"))
    };
    let (ex_max, ex_max_err) = if let Some(m) = range.exclusive_max {
        (
            quote!(Some(#m)),
            quote!(err.add_param(
                #cow_type,
                &#m);),
        )
    } else {
        (quote!(None), quote!())
    };

    let message = quote_message(range.message);
    let code = quote_code(crate_name, range.code, "range");

    let cow_type = if cfg!(feature = "std") {
        quote!(::std::borrow::Cow::from("value"))
    } else {
        quote!(::alloc::borrow::Cow::from("value"))
    };
    quote! {
        if !#field_name.validate_range(#min, #max, #ex_min, #ex_max) {
            #code
            #message
            #min_err
            #max_err
            #ex_min_err
            #ex_max_err
            err.add_param(
                 #cow_type,
                 &#field_name);
            errors.add(#field_name_str, err);
        }
    }
}
