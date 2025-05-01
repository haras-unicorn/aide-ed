#![deny(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![deny(clippy::arithmetic_side_effects)]
#![deny(clippy::dbg_macro, clippy::print_stdout, clippy::print_stderr)]
#![deny(clippy::todo)]
#![deny(clippy::unreachable)]
#![deny(clippy::allow_attributes_without_reason)]

use proc_macro::TokenStream;
use syn::parse_macro_input;

extern crate proc_macro;

mod crud;

#[proc_macro_attribute]
pub fn generate_response(attr: TokenStream, item: TokenStream) -> TokenStream {
  let input_struct = parse_macro_input!(item as syn::ItemStruct);

  crud::response::implement_generate_response(attr, input_struct)
}
