#![deny(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![deny(clippy::arithmetic_side_effects)]
#![deny(clippy::dbg_macro, clippy::print_stdout, clippy::print_stderr)]
#![deny(clippy::todo)]
#![deny(clippy::unreachable)]
#![deny(clippy::allow_attributes_without_reason)]

extern crate proc_macro;

mod crud;
mod fns;

use inflector::Inflector;
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
  join: bool,
}

impl Parse for ModelArgs {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let mut api_module_name = "api".to_string();
    let mut server_module_name = "server".to_string();
    let mut server_feature_name = "server".to_string();
    let mut join = false;

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
          } else if path.is_ident("join") {
            if let syn::Expr::Lit(syn::ExprLit {
              lit: syn::Lit::Bool(lit_bool),
              ..
            }) = value
            {
              join = lit_bool.value;
            } else {
              return Err(syn::Error::new_spanned(
                value,
                "Expected a boolean literal (true or false) for join",
              ));
            }
          } else {
            return Err(syn::Error::new_spanned(
                            path,
                            "Unknown attribute argument, expected 'api_module', 'server_module', 'server_feature', or 'join'",
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
      join,
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
  let struct_name = &input_struct.ident;

  let response_code =
    crud::model_part(&input_struct, "", "Response", "response_field");
  let create_args_code =
    crud::model_part(&input_struct, "Create", "Args", "create_args_field");
  let update_args_code = if !args.join {
    crud::model_part(&input_struct, "Update", "Args", "update_args_field")
  } else {
    quote! {}
  };

  let mut cleaned_input_struct = input_struct.clone();
  crud::clean_attributes(
    &mut cleaned_input_struct,
    &["response_field", "create_args_field", "update_args_field"],
  );

  let struct_name_str = struct_name.to_string();
  let table_name_str = struct_name_str.to_snake_case().to_plural();
  let table_name_ident = format_ident!("{}", table_name_str);

  let create_args_ident = format_ident!("Create{}Args", struct_name_str);
  let update_args_ident = format_ident!("Update{}Args", struct_name_str);
  let response_ident = format_ident!("{}Response", struct_name_str);
  let server_struct_ident = format_ident!("{}", struct_name_str);

  let create_fn_code = fns::generate_create_fn(
    &input_struct, // Pass original struct to check attributes
    &table_name_ident,
    &create_args_ident,
    &response_ident,
    &server_struct_ident,
    args.join,
  );

  let get_fn_code = if !args.join {
    let pk_ident = format_ident!("id");
    let pk_type = quote! { uuid::Uuid };
    fns::generate_get_fn(
      &input_struct,
      &table_name_ident,
      &pk_ident,
      &pk_type,
      &response_ident,
      &server_struct_ident,
    )
  } else {
    quote! {}
  };
  let list_fn_code = fns::generate_list_fn(
    &input_struct,
    &table_name_ident,
    &response_ident,
    &server_struct_ident,
  );
  let update_fn_code = if !args.join {
    let pk_ident = format_ident!("id");
    let pk_type = quote! { uuid::Uuid };
    fns::generate_update_fn(
      &input_struct,
      &table_name_ident,
      &pk_ident,
      &pk_type,
      &update_args_ident,
      &response_ident,
      &server_struct_ident,
    )
  } else {
    quote! {}
  };

  let delete_fn_code = {
    let pk_ident = format_ident!("id");
    let pk_type = quote! { uuid::Uuid };
    fns::generate_delete_fn(
      &input_struct,
      &table_name_ident,
      &pk_ident,
      &pk_type,
      &create_args_ident,
      args.join,
    )
  };

  let combined_output = quote! {
      pub mod #api_module_ident {
          use serde::{Deserialize, Serialize};
          use uuid::Uuid;
          use chrono::{DateTime, Utc};
          use serde_json::Value;

          #response_code
          #create_args_code
          #update_args_code
      }

      pub mod #server_module_ident {
          #[cfg(feature = #server_feature_name)]
          use crate::schema::*;
          #[cfg(feature = #server_feature_name)]
          use diesel::prelude::*;

          use server_fn::codec::Json;
          use dioxus::prelude::*;
          use serde::{Deserialize, Serialize};
          use uuid::Uuid;
          use chrono::{DateTime, Utc};
          use serde_json::Value;
          use crate::models::#api_module_ident::*;

          #[cfg(feature = #server_feature_name)]
          #cleaned_input_struct

          #create_fn_code
          #get_fn_code
          #list_fn_code
          #update_fn_code
          #delete_fn_code
      }

      pub use #api_module_ident::*;

      #[cfg(feature = #server_feature_name)]
      pub use #server_module_ident::*;
  };

  TokenStream::from(combined_output)
}
