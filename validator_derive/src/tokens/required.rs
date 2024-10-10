use quote::quote;
use syn::Ident;

use crate::types::Required;
use crate::utils::{quote_code, quote_message, CrateName};

pub fn required_tokens(
    crate_name: &CrateName,
    required: Required,
    field_name: &Ident,
    field_name_str: &str,
) -> proc_macro2::TokenStream {
    let message = quote_message(required.message);
    let code = quote_code(crate_name, required.code, "required");

    let cow_type = if cfg!(feature = "std") {
        quote!(::std::borrow::Cow::from("value"))
    } else {
        quote!(::alloc::borrow::Cow::from("value"))
    };
    quote! {
        if !self.#field_name.validate_required() {
            #code
            #message
            err.add_param(
                #cow_type,
                &self.#field_name);
            errors.add(#field_name_str, err);
        }
    }
}
