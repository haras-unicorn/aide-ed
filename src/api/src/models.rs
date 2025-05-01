use crate::schema::*;
use aide_ed_macro::*;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(
  Queryable, Selectable, Insertable, Debug, Clone, Serialize, Deserialize,
)]
#[diesel(table_name = changes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Change {
  pub id: Uuid,
  pub teacher_id: Uuid,
  pub content_table: String,
  pub content_id: Uuid,
  pub issue_id: Option<Uuid>,
  pub title: String,
  pub description: String,
  pub proposed_changes: Value,
  pub created_at: DateTime<Utc>,
  pub merged_at: Option<DateTime<Utc>>,
}

#[derive(
  Queryable,
  Selectable,
  Insertable,
  Associations,
  Identifiable,
  Debug,
  Clone,
  Serialize,
  Deserialize,
)]
#[diesel(table_name = class_courses)]
#[diesel(belongs_to(Class))]
#[diesel(belongs_to(Course))]
#[diesel(primary_key(class_id, course_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ClassCourse {
  pub class_id: Uuid,
  pub course_id: Uuid,
}

#[derive(
  Queryable,
  Selectable,
  Insertable,
  Identifiable,
  Associations,
  Debug,
  Clone,
  Serialize,
  Deserialize,
)]
#[diesel(table_name = classes)]
#[diesel(belongs_to(Organization))]
#[diesel(belongs_to(Teacher))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Class {
  pub id: Uuid,
  pub title: String,
  pub description: String,
  pub year: i32,
  pub letter: String,
  pub organization_id: Uuid,
  pub teacher_id: Uuid,
}

#[derive(
  Queryable,
  Selectable,
  Insertable,
  Identifiable,
  Associations,
  Debug,
  Clone,
  Serialize,
  Deserialize,
)]
#[diesel(table_name = courses)]
#[diesel(belongs_to(Subject))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Course {
  pub id: Uuid,
  pub title: String,
  pub description: String,
  pub subject_id: Uuid,
}

#[derive(
  Queryable,
  Selectable,
  Insertable,
  Identifiable,
  Associations,
  Debug,
  Clone,
  Serialize,
  Deserialize,
)]
#[diesel(table_name = guides)]
#[diesel(belongs_to(Lecture))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Guide {
  pub id: Uuid,
  pub title: String,
  pub description: String,
  pub lecture_id: Uuid,
  pub content: Value,
}

#[derive(
  Queryable,
  Selectable,
  Insertable,
  Identifiable,
  Associations,
  Debug,
  Clone,
  Serialize,
  Deserialize,
)]
#[diesel(table_name = issues)]
#[diesel(belongs_to(Teacher))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Issue {
  pub id: Uuid,
  pub title: String,
  pub description: String,
  pub teacher_id: Option<Uuid>,
  pub content_table: String,
  pub content_id: Uuid,
  pub created_at: DateTime<Utc>,
  pub resolved: Option<bool>,
}

#[derive(
  Queryable,
  Selectable,
  Insertable,
  Identifiable,
  Associations,
  Debug,
  Clone,
  Serialize,
  Deserialize,
)]
#[diesel(table_name = lectures)]
#[diesel(belongs_to(Course))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Lecture {
  pub id: Uuid,
  pub title: String,
  pub description: String,
  pub course_id: Uuid,
}

#[derive(
  Queryable,
  Selectable,
  Insertable,
  Identifiable,
  Associations,
  Debug,
  Clone,
  Serialize,
  Deserialize,
)]
#[diesel(table_name = messages)]
#[diesel(belongs_to(Teacher))] // Assuming relation based on teacher_id
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Message {
  pub id: Uuid,
  pub teacher_id: Uuid,
  pub parent_table: String,
  pub parent_id: Uuid,
  pub content: String,
  pub created_at: DateTime<Utc>,
}

#[derive(
  Queryable,
  Selectable,
  Insertable,
  Identifiable,
  Associations,
  Debug,
  Clone,
  Serialize,
  Deserialize,
)]
#[diesel(table_name = organizations)]
#[diesel(belongs_to(Organization, foreign_key = organization_id))] // Self-reference needs explicit fk name
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Organization {
  pub id: Uuid,
  pub title: String,
  pub description: String,
  pub organization_id: Option<Uuid>,
}

#[derive(
  Queryable,
  Selectable,
  Insertable,
  Identifiable,
  Associations,
  Debug,
  Clone,
  Serialize,
  Deserialize,
)]
#[diesel(table_name = reviews)]
#[diesel(belongs_to(Change))]
#[diesel(belongs_to(Teacher))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Review {
  pub id: Uuid,
  pub change_id: Uuid,
  pub teacher_id: Uuid,
  pub comment: Option<String>,
  pub approved: Option<bool>,
  pub created_at: DateTime<Utc>,
}

#[derive(
  Queryable,
  Selectable,
  Insertable,
  Associations,
  Identifiable,
  Debug,
  Clone,
  Serialize,
  Deserialize,
)]
#[diesel(table_name = student_classes)]
#[diesel(belongs_to(Student))]
#[diesel(belongs_to(Class))]
#[diesel(primary_key(student_id, class_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StudentClass {
  pub student_id: Uuid,
  pub class_id: Uuid,
}

#[derive(
  Queryable,
  Selectable,
  Insertable,
  Identifiable,
  Associations,
  Debug,
  Clone,
  Serialize,
  Deserialize,
)]
#[diesel(table_name = students)]
#[diesel(belongs_to(Organization))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Student {
  pub id: Uuid,
  pub name: String,
  pub organization_id: Uuid,
}

#[models]
#[derive(
  Queryable,
  Selectable,
  Insertable,
  Identifiable,
  Debug,
  Clone,
  Serialize,
  Deserialize,
)]
#[diesel(table_name = subjects)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Subject {
  #[response_field]
  #[update_args_field]
  pub id: Uuid,

  #[response_field]
  #[create_args_field]
  #[update_args_field]
  pub title: String,

  #[response_field]
  #[create_args_field]
  #[update_args_field]
  pub description: String,
}

#[derive(
  Queryable,
  Selectable,
  Insertable,
  Identifiable,
  Associations,
  Debug,
  Clone,
  Serialize,
  Deserialize,
)]
#[diesel(table_name = teachers)]
#[diesel(belongs_to(Organization))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Teacher {
  pub id: Uuid,
  pub name: String,
  pub organization_id: Uuid,
}
