use quote::quote;

use crate::types::Custom;
use crate::utils::quote_message;

pub fn custom_tokens(
    custom: Custom,
    field_name: &proc_macro2::TokenStream,
    field_name_str: &str,
) -> proc_macro2::TokenStream {
    let fn_call = custom.function.unwrap();

    let args = if let Some(arg) = custom.use_context {
        if arg {
            quote!(#field_name, args)
        } else {
            quote!(#field_name)
        }
    } else {
        quote!(#field_name)
    };

    let message = quote_message(custom.message);

    let code = if let Some(c) = custom.code {
        if cfg!(feature = "std") {
            quote!(err.code = ::std::borrow::Cow::from(#c);)
        } else {
            quote!(err.code = ::alloc::borrow::Cow::from(#c);)
        }
    } else {
        quote!()
    };

    let cow_type = if cfg!(feature = "std") {
        quote!(::std::borrow::Cow::from("value"))
    } else {
        quote!(::alloc::borrow::Cow::from("value"))
    };
    quote! {
        match #fn_call(#args) {
            ::core::result::Result::Ok(()) => {}
            ::core::result::Result::Err(mut err) => {
                #code
                #message
                err.add_param(
                    #cow_type,
                    &#field_name);
                errors.add(#field_name_str, err);
            }
        }
    }
}
