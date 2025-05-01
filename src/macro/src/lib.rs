#![deny(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![deny(clippy::arithmetic_side_effects)]
#![deny(clippy::dbg_macro, clippy::print_stdout, clippy::print_stderr)]
#![deny(clippy::todo)]
#![deny(clippy::unreachable)]
#![deny(clippy::allow_attributes_without_reason)]

extern crate proc_macro;

mod crud;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
  parse::{Parse, ParseStream},
  parse_macro_input, Meta, MetaNameValue, Token,
};

struct ModelArgs {
  server_feature_name: String,
  server_module_name: String,
  api_module_name: String,
}

impl Parse for ModelArgs {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    // Initialize with default values
    let mut api_module_name = "api".to_string();
    let mut server_module_name = "server".to_string();
    let mut server_feature_name = "server".to_string();

    while !input.is_empty() {
      let meta: Meta = input.parse()?;

      match meta {
        Meta::NameValue(MetaNameValue { path, value, .. }) => {
          if path.is_ident("api_module") {
            if let syn::Expr::Lit(syn::ExprLit {
              lit: syn::Lit::Str(lit_str),
              ..
            }) = value
            {
              api_module_name = lit_str.value();
            } else {
              return Err(syn::Error::new_spanned(
                value,
                "Expected a string literal for api_module",
              ));
            }
          } else if path.is_ident("server_module") {
            if let syn::Expr::Lit(syn::ExprLit {
              lit: syn::Lit::Str(lit_str),
              ..
            }) = value
            {
              server_module_name = lit_str.value();
            } else {
              return Err(syn::Error::new_spanned(
                value,
                "Expected a string literal for server_module",
              ));
            }
          } else if path.is_ident("server_feature") {
            if let syn::Expr::Lit(syn::ExprLit {
              lit: syn::Lit::Str(lit_str),
              ..
            }) = value
            {
              server_feature_name = lit_str.value();
            } else {
              return Err(syn::Error::new_spanned(
                value,
                "Expected a string literal for server_feature",
              ));
            }
          } else {
            return Err(syn::Error::new_spanned(
                            path,
                            "Unknown attribute argument, expected 'api_module', 'server_module', or 'server_feature'",
                        ));
          }
        }
        _ => {
          return Err(syn::Error::new_spanned(
            meta,
            "Expected attribute argument like key = \"value\"",
          ))
        }
      }

      if input.peek(Token![,]) {
        input.parse::<Token![,]>()?;
      } else if !input.is_empty() {
        return Err(
          input.error("Expected comma between arguments or end of arguments"),
        );
      }
    }

    Ok(ModelArgs {
      api_module_name,
      server_feature_name,
      server_module_name,
    })
  }
}

#[proc_macro_attribute]
pub fn models(attr: TokenStream, item: TokenStream) -> TokenStream {
  let args = parse_macro_input!(attr as ModelArgs);
  let api_module_ident = format_ident!("{}", args.api_module_name);
  let server_module_ident = format_ident!("{}", args.server_module_name);
  let server_feature_name = args.server_feature_name;

  let input_struct = parse_macro_input!(item as syn::ItemStruct);

  let response_code =
    crud::model_part(&input_struct, "", "Response", "response_field");
  let create_args_code =
    crud::model_part(&input_struct, "Create", "Args", "create_args_field");
  let update_args_code =
    crud::model_part(&input_struct, "Update", "Args", "update_args_field");

  let mut cleaned_input_struct = input_struct.clone();
  crud::clean_attributes(
    &mut cleaned_input_struct,
    &["response_field", "create_args_field", "update_args_field"],
  );

  let combined_output = quote! {
    #[cfg(feature = #server_feature_name)]
    pub mod #server_module_ident {
      use crate::schema::*;
      use diesel::prelude::*;
      use serde::{Deserialize, Serialize};
      use uuid::Uuid;
      use chrono::{DateTime, Utc};

      #cleaned_input_struct
    }

    pub mod #api_module_ident {
      use uuid::Uuid;
      use chrono::{DateTime, Utc};

      #response_code

      #create_args_code

      #update_args_code
    }
  };

  TokenStream::from(combined_output)
}
