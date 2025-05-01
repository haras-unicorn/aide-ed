#![deny(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![deny(clippy::arithmetic_side_effects)]
#![deny(clippy::dbg_macro, clippy::print_stdout, clippy::print_stderr)]
#![deny(clippy::todo)]
#![deny(clippy::unreachable)]
#![deny(clippy::allow_attributes_without_reason)]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote; // The magic code generation macro!
use syn::{parse_macro_input, FieldsNamed, Ident, ItemStruct}; // Tools to parse Rust code structure

// This defines our attribute macro called `generate_response`
#[proc_macro_attribute]
pub fn generate_response(
  _attr: TokenStream, // We aren't using attributes on the macro itself for now
  item: TokenStream, // This is the token stream of the item below the attribute (our struct)
) -> TokenStream {
  // We return the generated code as a TokenStream

  // 1. Parse the input struct definition
  // `parse_macro_input!` handles errors nicely if the input isn't a struct
  let input_struct = parse_macro_input!(item as ItemStruct);

  // 2. Get the name of the input struct (e.g., "Organization")
  let input_name = &input_struct.ident;

  // 3. Create the name for the new response struct (e.g., "OrganizationResponse")
  // We use `format!` to create the new name string and `Ident::new` to make it a Rust identifier
  // `input_name.span()` helps the compiler give better error messages if needed later
  let response_name =
    Ident::new(&format!("{}Response", input_name), input_name.span());

  // 4. Extract the named fields from the input struct
  let fields = if let syn::Fields::Named(FieldsNamed { ref named, .. }) =
    input_struct.fields
  {
    named // This is an iterator over the fields
  } else {
    // For simplicity, we'll panic if it's not a struct with named fields.
    // A real-world macro would return a compile error.
    panic!("generate_response only works on structs with named fields.");
  };

  // 5. Filter for the specific fields we want in the response struct
  let desired_field_names = ["id", "title", "description"]; // The fields we want to copy
  let response_fields = fields.iter().filter(|field| {
    // Get the field's name (identifier)
    if let Some(ident) = &field.ident {
      // Check if the name is in our desired list
      desired_field_names.contains(&ident.to_string().as_str())
    } else {
      false // Should not happen with named fields, but good to be safe
    }
  });

  // 6. Generate the code for the new struct using the `quote!` macro
  let generated_code = quote! {
      // First, we output the original struct definition unchanged
      #input_struct

      // Now, we define our new response struct
      #[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
      pub struct #response_name {
          // We iterate through the filtered `response_fields`
          // For each field, we include its definition (e.g., `pub id: Uuid`)
          // The `#` is part of quote's syntax for repetition/interpolation.
          // The `*` means repeat for every item in the iterator.
          #(pub #response_fields),*
      }
  };

  // 7. Return the generated code as a TokenStream
  TokenStream::from(generated_code)
}
