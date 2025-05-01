use proc_macro::TokenStream;
use quote::quote;
use syn::{FieldsNamed, Ident, ItemStruct};

pub fn implement_generate_response(
  _attr: TokenStream,
  mut input_struct: ItemStruct,
) -> TokenStream {
  let input_name = &input_struct.ident;
  let response_name =
    Ident::new(&format!("{}Response", input_name), input_name.span());

  // --- Get fields and prepare to modify the original struct ---
  let fields =
    if let syn::Fields::Named(FieldsNamed { ref mut named, .. }) =
      input_struct.fields
    {
      named
    } else {
      // Using panic here is okay within a proc macro as it provides compile-time errors
      panic!("generate_response only works on structs with named fields.");
    };

  // --- Identify fields for the response struct AND clean the original ---
  let mut response_fields_data = Vec::new();

  for field in fields.iter_mut() {
    let mut is_response_field = false;
    // Keep only attributes that are NOT response_field
    field.attrs.retain(|attr| {
      if attr.path().is_ident("response_field") {
        is_response_field = true;
        false // Remove this attribute
      } else {
        true // Keep other attributes
      }
    });

    if is_response_field {
      // Clone necessary data for the response struct
      // We need Option<&Ident> because field.ident might be None (though unlikely for named fields)
      if let Some(ident) = field.ident.clone() {
        response_fields_data.push((ident, field.ty.clone()));
      }
    }
  }

  // --- Extract idents and types for quoting the response struct ---
  let response_field_idents =
    response_fields_data.iter().map(|(ident, _)| ident);
  let response_field_types = response_fields_data.iter().map(|(_, ty)| ty);

  // --- Generate the code ---
  let generated_code = quote! {
      // Output the MODIFIED input_struct (without #[response_field])
      #input_struct

      // Output the new response struct
      #[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
      pub struct #response_name {
          #(pub #response_field_idents: #response_field_types),*
      }
  };

  TokenStream::from(generated_code)
}
