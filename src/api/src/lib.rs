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

macro_rules! export_crud_for {
  ($lower:ident, $plural:ident, $capital:ident) => {
    paste! {
        pub use models::[<$lower _api>]::[<Create $capital Args>];
        pub use models::[<$lower _api>]::[<$capital Response>];
        pub use models::[<$lower _api>]::[<Update $capital Args>];
        pub use models::[<$lower _server>]::[<create_ $lower>];
        pub use models::[<$lower _server>]::[<delete_ $lower>];
        pub use models::[<$lower _server>]::[<get_ $lower>];
        pub use models::[<$lower _server>]::[<list_ $plural>];
        pub use models::[<$lower _server>]::[<update_ $lower>];
    }
  };
  ($lower:ident, $plural:ident, $capital:ident, join) => {
    paste! {
        pub use models::[<$lower _api>]::[<Create $capital Args>];
        pub use models::[<$lower _api>]::[<$capital Response>];
        pub use models::[<$lower _server>]::[<create_ $lower>];
        pub use models::[<$lower _server>]::[<delete_ $lower>];
        pub use models::[<$lower _server>]::[<list_ $plural>];
    }
  };
}

export_crud_for!(subject, subjects, Subject);
export_crud_for!(change, changes, Change);
export_crud_for!(class_course, class_courses, ClassCourse, join);
export_crud_for!(class, classes, Class);
export_crud_for!(course, courses, Course);
export_crud_for!(guide, guides, Guide);
export_crud_for!(issue, issues, Issue);
export_crud_for!(lecture, lectures, Lecture);
export_crud_for!(message, messages, Message);
export_crud_for!(organization, organizations, Organization);
export_crud_for!(review, reviews, Review);
export_crud_for!(student_class, student_classes, StudentClass, join);
export_crud_for!(student, students, Student);
export_crud_for!(teacher, teachers, Teacher);
