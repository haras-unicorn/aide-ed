#![deny(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![deny(clippy::arithmetic_side_effects)]
#![deny(clippy::dbg_macro, clippy::print_stdout, clippy::print_stderr)]
#![deny(clippy::todo)]
#![deny(clippy::unreachable)]
#![deny(clippy::allow_attributes_without_reason)]

pub mod fns;
pub mod models;
#[cfg(feature = "server")]
pub mod schema;

#[cfg(feature = "server")]
pub(crate) fn establish_connection(
) -> Result<diesel::pg::PgConnection, diesel::prelude::ConnectionError> {
  use diesel::prelude::*;
  use std::env;

  let database_url =
    env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  PgConnection::establish(&database_url)
}
