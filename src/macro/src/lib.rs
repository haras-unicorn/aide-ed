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
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn models(_attr: TokenStream, item: TokenStream) -> TokenStream {
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
      #cleaned_input_struct // The original struct definition (now cleaned)

      #response_code       // The generated Response struct

      #create_args_code    // The generated CreateArgs struct

      #update_args_code    // The generated UpdateArgs struct
  };

  TokenStream::from(combined_output)
}
