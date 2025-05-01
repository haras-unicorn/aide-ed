use diesel::pg::PgConnection;
use diesel::prelude::*;
use dioxus::prelude::*;
use serde_json::Value;
use server_fn::codec::Json;
use std::env;
use uuid::Uuid;

fn establish_connection() -> Result<PgConnection, ConnectionError> {
  let database_url =
    env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  PgConnection::establish(&database_url)
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct CreateSubjectArgs {
  pub title: String,
  pub description: String,
}

#[server(prefix = "/api", input = Json, output = Json)]
pub async fn create_subject(
  args: CreateSubjectArgs,
) -> Result<Value, ServerFnError> {
  let mut conn = establish_connection()
    .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

  let new_subject = Subject {
    id: Uuid::new_v8(),
    title: args.title,
    description: args.description,
  };

  diesel::insert_into(subjects::table)
    .values(&new_subject)
    .get_result(&mut conn)
    .map_err(|e| {
      ServerFnError::ServerError(format!("Error creating subject: {}", e))
    })
}

#[server(prefix = "/api", input = Json, output = Json)]
pub async fn get_subject(subject_id: Uuid) -> Result<Value, ServerFnError> {
  let mut conn = establish_connection()
    .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

  subjects::table
    .find(subject_id)
    .first(&mut conn)
    .map_err(|e| match e {
      diesel::result::Error::NotFound => ServerFnError::ServerError(format!(
        "Subject with id {} not found",
        subject_id
      )),
      _ => ServerFnError::ServerError(format!("Error getting subject: {}", e)),
    })
}

#[server(prefix = "/api", input = Json, output = Json)]
pub async fn list_subjects() -> Result<Value, ServerFnError> {
  let mut conn = establish_connection()
    .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

  subjects::table
    .select(Subject::as_select())
    .load(&mut conn)
    .map_err(|e| {
      ServerFnError::ServerError(format!("Error listing subjects: {}", e))
    })
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct UpdateSubjectArgs {
  pub id: Uuid,
  pub title: String,
  pub description: String,
}

#[server(prefix = "/api", input = Json, output = Json)]
pub async fn update_subject(
  args: UpdateSubjectArgs,
) -> Result<Value, ServerFnError> {
  let mut conn = establish_connection()
    .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

  diesel::update(subjects::table.find(args.id))
    .set((
      subjects::title.eq(args.title),
      subjects::description.eq(args.description),
    ))
    .get_result(&mut conn)
    .map_err(|e| match e {
      diesel::result::Error::NotFound => ServerFnError::ServerError(format!(
        "Subject with id {} not found for update",
        args.id
      )),
      _ => ServerFnError::ServerError(format!("Error updating subject: {}", e)),
    })
}

#[server(prefix = "/api", input = Json, output = Json)]
pub async fn delete_subject(subject_id: Uuid) -> Result<usize, ServerFnError> {
  let mut conn = establish_connection()
    .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

  diesel::delete(subjects::table.find(subject_id))
    .execute(&mut conn)
    .map_err(|e| match e {
      diesel::result::Error::NotFound => ServerFnError::ServerError(format!(
        "Subject with id {} not found for deletion",
        subject_id
      )),
      _ => ServerFnError::ServerError(format!("Error deleting subject: {}", e)),
    })
}
