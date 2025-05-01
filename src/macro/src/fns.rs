use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{FieldsNamed, Ident, ItemStruct, Type};

fn get_target_fields<'a>(
  original_struct: &'a ItemStruct,
  target_attr_name: &str,
) -> Vec<(&'a Ident, &'a Type)> {
  let fields = match &original_struct.fields {
    syn::Fields::Named(FieldsNamed { named, .. }) => named,
    _ => return Vec::new(), // Or panic, depending on desired strictness
  };

  fields
    .iter()
    .filter_map(|field| {
      let is_target = field
        .attrs
        .iter()
        .any(|attr| attr.path().is_ident(target_attr_name));
      if is_target {
        field.ident.as_ref().map(|ident| (ident, &field.ty))
      } else {
        None
      }
    })
    .collect()
}

pub fn generate_create_fn(
  original_struct: &ItemStruct,
  table_name_ident: &Ident,
  create_args_ident: &Ident,
  response_ident: &Ident,
  server_struct_ident: &Ident,
  is_join_table: bool,
) -> TokenStream2 {
  let struct_name_str = original_struct.ident.to_string();
  let fn_name = format_ident!("create_{}", struct_name_str.to_lowercase());
  let error_entity_name = format!("{} {}", struct_name_str, "{e}"); // For error messages

  let create_fields = get_target_fields(original_struct, "create_args_field");
  let create_field_assigns = create_fields
    .iter()
    .map(|(ident, _)| quote! { #ident: args.#ident });

  let response_fields = get_target_fields(original_struct, "response_field");
  let response_field_assigns = response_fields
    .iter()
    .map(|(ident, _)| quote! { #ident: result.#ident });

  let pk_ident = format_ident!("id");
  let pk_type = quote! { uuid::Uuid };
  let id_assignment = if !is_join_table
    && original_struct
      .fields
      .iter()
      .any(|f| f.ident.as_ref().map_or(false, |i| i == &pk_ident))
  {
    quote! { #pk_ident: #pk_type::new_v4(), }
  } else {
    quote! {} // No ID generation for join tables or if no 'id' field
  };

  quote! {
      #[server(prefix = "/api", input = Json, output = Json)]
      pub async fn #fn_name(
          args: #create_args_ident,
      ) -> Result<#response_ident, ServerFnError> {
          use crate::schema::*;
          use crate::schema::#table_name_ident::dsl::*; // Use dsl for table name access
          use diesel::prelude::*;
          use super::#server_struct_ident; // Access the struct defined in this module
          use crate::establish_connection; // Assuming establish_connection is in crate root

          let mut conn = establish_connection().map_err(|e| {
              ServerFnError::<server_fn::error::NoCustomError>::ServerError(format!("Database connection error: {}", e))
          })?;

          let new_record = #server_struct_ident {
              #id_assignment
              #(#create_field_assigns),*
          };

          let result = diesel::insert_into(#table_name_ident)
              .values(&new_record)
              .get_result::<#server_struct_ident>(&mut conn)
              .map_err(|e| {
                  ServerFnError::<server_fn::error::NoCustomError>::ServerError(format!(
                      "Error creating {}: {}",
                      #error_entity_name, e
                  ))
              })?;

          Ok(#response_ident {
              #(#response_field_assigns),*
          })
      }
  }
}

pub fn generate_get_fn(
  original_struct: &ItemStruct,
  table_name_ident: &Ident,
  _pk_ident: &Ident,
  pk_type: &TokenStream2,
  response_ident: &Ident,
  server_struct_ident: &Ident,
) -> TokenStream2 {
  let struct_name_str = original_struct.ident.to_string();
  let fn_name = format_ident!("get_{}", struct_name_str.to_lowercase());
  let input_arg_name = format_ident!("{}_id", struct_name_str.to_lowercase());
  let error_entity_name =
    format!("{} with id {}", struct_name_str, "{#input_arg_name}");

  let response_fields = get_target_fields(original_struct, "response_field");
  let response_field_assigns = response_fields
    .iter()
    .map(|(ident, _)| quote! { #ident: result.#ident });

  quote! {
      #[server(prefix = "/api", input = Json, output = Json)]
      pub async fn #fn_name(
          #input_arg_name: #pk_type,
      ) -> Result<#response_ident, ServerFnError> {
          use crate::schema::*;
          use crate::schema::#table_name_ident::dsl::*;
          use diesel::prelude::*;
          use super::#server_struct_ident;
          use crate::establish_connection;

          let mut conn = establish_connection().map_err(|e| {
              ServerFnError::<server_fn::error::NoCustomError>::ServerError(format!("Database connection error: {}", e))
          })?;

          let result = #table_name_ident
              .find(#input_arg_name)
              .first::<#server_struct_ident>(&mut conn)
              .map_err(|e| match e {
                  diesel::result::Error::NotFound => ServerFnError::<server_fn::error::NoCustomError>::ServerError(format!(
                      "{} not found",
                      #error_entity_name
                  )),
                  _ => ServerFnError::<server_fn::error::NoCustomError>::ServerError(format!(
                      "Error getting {}: {}",
                      #error_entity_name, e
                  )),
              })?;

          Ok(#response_ident {
              #(#response_field_assigns),*
          })
      }
  }
}

pub fn generate_list_fn(
  original_struct: &ItemStruct,
  table_name_ident: &Ident,
  response_ident: &Ident,
  server_struct_ident: &Ident,
) -> TokenStream2 {
  let struct_name_str = original_struct.ident.to_string();
  let fn_name = format_ident!("list_{}s", struct_name_str.to_lowercase()); // Pluralize fn name
  let error_entity_name = format!("{}s", struct_name_str); // Pluralize entity name

  let response_fields = get_target_fields(original_struct, "response_field");
  let response_field_assigns = response_fields
    .iter()
    .map(|(ident, _)| quote! { #ident: item.#ident });

  quote! {
      #[server(prefix = "/api", input = Json, output = Json)]
      pub async fn #fn_name() -> Result<Vec<#response_ident>, ServerFnError> {
          use crate::schema::*;
          use crate::schema::#table_name_ident::dsl::*;
          use diesel::prelude::*;
          use super::#server_struct_ident;
          use crate::establish_connection;

          let mut conn = establish_connection().map_err(|e| {
              ServerFnError::<server_fn::error::NoCustomError>::ServerError(format!("Database connection error: {}", e))
          })?;

          let results = #table_name_ident
              .select(#server_struct_ident::as_select()) // Use the server struct here
              .load::<#server_struct_ident>(&mut conn)
              .map_err(|e| {
                  ServerFnError::<server_fn::error::NoCustomError>::ServerError(format!(
                      "Error listing {}: {}",
                      #error_entity_name, e
                  ))
              })?;

          Ok(
              results
                  .into_iter()
                  .map(|item| #response_ident {
                      #(#response_field_assigns),*
                  })
                  .collect::<Vec<_>>(),
          )
      }
  }
}

pub fn generate_update_fn(
  original_struct: &ItemStruct,
  table_name_ident: &Ident,
  pk_ident: &Ident,
  pk_type: &TokenStream2,
  update_args_ident: &Ident,
  response_ident: &Ident,
  server_struct_ident: &Ident,
) -> TokenStream2 {
  let struct_name_str = original_struct.ident.to_string();
  let fn_name = format_ident!("update_{}", struct_name_str.to_lowercase());
  let input_pk_arg_name = pk_ident.clone();
  let error_entity_name =
    format!("{} with id {}", struct_name_str, "{#input_pk_arg_name}");

  let update_fields = get_target_fields(original_struct, "update_args_field");
  let set_clauses = update_fields.iter().map(|(ident, _)| {
    quote! { #table_name_ident::#ident.eq(args.#ident.clone()) }
  });

  let response_fields = get_target_fields(original_struct, "response_field");
  let response_field_assigns = response_fields
    .iter()
    .map(|(ident, _)| quote! { #ident: result.#ident });

  quote! {
      #[server(prefix = "/api", input = Json, output = Json)]
      pub async fn #fn_name(
          #input_pk_arg_name: #pk_type,
          args: #update_args_ident,
      ) -> Result<#response_ident, ServerFnError> {
          use crate::schema::*;
          use crate::schema::#table_name_ident;
          use diesel::prelude::*;
          use super::#server_struct_ident;
          use crate::establish_connection;

          let mut conn = establish_connection().map_err(|e| {
              ServerFnError::<server_fn::error::NoCustomError>::ServerError(format!("Database connection error: {}", e))
          })?;

          let target = #table_name_ident::table.find(#input_pk_arg_name);

          let result = diesel::update(target)
              .set((
                  #(#set_clauses),*
              ))
              .get_result::<#server_struct_ident>(&mut conn)
              .map_err(|e| match e {
                  diesel::result::Error::NotFound => ServerFnError::<server_fn::error::NoCustomError>::ServerError(format!(
                      "{} not found for update",
                      #error_entity_name
                  )),
                  _ => ServerFnError::<server_fn::error::NoCustomError>::ServerError(format!(
                      "Error updating {}: {}",
                      #error_entity_name, e
                  )),
              })?;

          Ok(#response_ident {
              #(#response_field_assigns),*
          })
      }
  }
}

pub fn generate_delete_fn(
  original_struct: &ItemStruct,
  table_name_ident: &Ident,
  create_args_ident: &Ident,
  is_join_table: bool,
) -> TokenStream2 {
  let struct_name_str = original_struct.ident.to_string();
  let fn_name = format_ident!("delete_{}", struct_name_str.to_lowercase());

  if is_join_table {
    let key_fields = get_target_fields(original_struct, "create_args_field");
    let key_field_names = key_fields.iter().map(|(ident, _)| ident);
    let key_tuple = key_fields.iter().map(|(ident, _)| quote! { args.#ident });
    let error_entity_name = format!(
      "{} with keys {:?}",
      struct_name_str,
      key_field_names
        .clone()
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
    );

    quote! {
        #[server(prefix = "/api", input = Json, output = Json)]
        pub async fn #fn_name(args: #create_args_ident) -> Result<usize, server_fn::ServerFnError> {
            use crate::schema::*;
            use crate::schema::#table_name_ident::dsl::*;
            use diesel::prelude::*;
            use crate::establish_connection;

            let mut conn = establish_connection().map_err(|e| {
                server_fn::ServerFnError::<server_fn::error::NoCustomError>::ServerError(
                    format!("Database connection error: {}", e))
            })?;

            let target = #table_name_ident.find((#(#key_tuple),*));

            diesel::delete(target)
                .execute(&mut conn)
                .map_err(|e| match e {
                    _ => server_fn::ServerFnError::<server_fn::error::NoCustomError>::ServerError(format!(
                        "Error deleting {}: {}",
                        #error_entity_name, e
                    )),
                })
        }
    }
  } else {
    let pk_type = quote! { uuid::Uuid };
    let input_arg_name = format_ident!("{}_id", struct_name_str.to_lowercase());
    let error_entity_name =
      format!("{} with id {}", struct_name_str, "{#input_arg_name}");

    quote! {
        #[server(prefix = "/api", input = Json, output = Json)]
        pub async fn #fn_name(#input_arg_name: #pk_type) -> Result<usize, ServerFnError::<server_fn::error::NoCustomError>> {
            use crate::schema::*;
            use crate::schema::#table_name_ident::dsl::*;
            use diesel::prelude::*;
            use crate::establish_connection;

            let mut conn = establish_connection().map_err(|e| {
                server_fn::ServerFnError::<server_fn::error::NoCustomError>::ServerError(format!("Database connection error: {}", e))
            })?;

            diesel::delete(#table_name_ident.find(#input_arg_name))
                .execute(&mut conn)
                .map_err(|e| match e {
                    diesel::result::Error::NotFound => server_fn::ServerFnError::<server_fn::error::NoCustomError>::ServerError(format!(
                        "{} not found for deletion",
                        #error_entity_name
                    )),
                    _ => server_fn::ServerFnError::<server_fn::error::NoCustomError>::ServerError(format!(
                        "Error deleting {}: {}",
                        #error_entity_name, e
                    )),
                })
        }
    }
  }
}
