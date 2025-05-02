#![deny(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![deny(clippy::arithmetic_side_effects)]
#![deny(clippy::dbg_macro, clippy::print_stdout, clippy::print_stderr)]
#![deny(clippy::todo)]
#![deny(clippy::unreachable)]
#![deny(clippy::allow_attributes_without_reason)]

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
  let conn = PgConnection::establish(&database_url)?;

  Ok(conn)
}

use paste::paste;

macro_rules! define_crud_exports {
  ($lower:ident, $capital:ident) => {
    paste! {
        pub use crate::models::[<$lower _api>]::[<Create $capital Args>];
        pub use crate::models::[<$lower _api>]::[<$capital Response>];
        pub use crate::models::[<$lower _api>]::[<Update $capital Args>];
        pub use crate::models::[<$lower _server>]::[<create_ $lower>];
        pub use crate::models::[<$lower _server>]::[<delete_ $lower>];
        pub use crate::models::[<$lower _server>]::[<get_ $lower>];
        pub use crate::models::[<$lower _server>]::[<list_ $lower s>];
        pub use crate::models::[<$lower _server>]::[<update_ $lower>];
    }
  };
}

define_crud_exports!(subject, Subject);
