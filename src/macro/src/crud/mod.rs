use proc_macro2::TokenStream as TokenStream2; // Use proc_macro2 for quote
use quote::quote;
use syn::{FieldsNamed, Ident, ItemStruct, Type};

pub fn model_part(
  original_struct: &ItemStruct,
  prefix: &str,
  suffix: &str,
  target_attr_name: &str,
) -> TokenStream2 {
  let input_name = &original_struct.ident;
  let derived_name = Ident::new(
    &format!("{}{}{}", prefix, input_name, suffix),
    input_name.span(),
  );

  let fields = match &original_struct.fields {
    syn::Fields::Named(FieldsNamed { named, .. }) => named,
    _ => {
      panic!(
        "Macro helper only works on structs with named fields. Applied to '{}'",
        input_name
      );
    }
  };

  let mut derived_fields_data: Vec<(&Ident, &Type)> = Vec::new();

  for field in fields.iter() {
    let mut is_target_field = false;
    for attr in &field.attrs {
      if attr.path().is_ident(target_attr_name) {
        is_target_field = true;
        break; // Found the attribute, no need to check others for this field
      }
    }

    if is_target_field {
      if let Some(ident) = field.ident.as_ref() {
        derived_fields_data.push((ident, &field.ty));
      }
    }
  }

  let derived_field_idents = derived_fields_data.iter().map(|(ident, _)| ident);
  let derived_field_types = derived_fields_data.iter().map(|(_, ty)| ty);

  quote! {
      #[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
      pub struct #derived_name {
          #(pub #derived_field_idents: #derived_field_types),*
      }
  }
}

pub fn clean_attributes(
  input_struct: &mut ItemStruct,
  attrs_to_remove: &[&str],
) {
  if let syn::Fields::Named(FieldsNamed { ref mut named, .. }) =
    input_struct.fields
  {
    for field in named.iter_mut() {
      field.attrs.retain(|attr| {
        !attrs_to_remove
          .iter()
          .any(|name_to_remove| attr.path().is_ident(name_to_remove))
      });
    }
  }
}
