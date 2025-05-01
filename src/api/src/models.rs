use aide_ed_macro::models;

#[models(api_module = "change_api", server_module = "change_server")]
#[derive(
  Queryable, Selectable, Insertable, Debug, Clone, Serialize, Deserialize,
)]
#[diesel(table_name = changes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Change {
  #[response_field]
  #[update_args_field]
  pub id: Uuid,

  #[response_field]
  #[create_args_field]
  #[update_args_field]
  pub teacher_id: Uuid,

  #[response_field]
  #[create_args_field]
  #[update_args_field]
  pub content_table: String,

  #[response_field]
  #[create_args_field]
  #[update_args_field]
  pub content_id: Uuid,

  #[response_field]
  #[create_args_field]
  #[update_args_field]
  pub issue_id: Option<Uuid>,

  #[response_field]
  #[create_args_field]
  #[update_args_field]
  pub title: String,

  #[response_field]
  #[create_args_field]
  #[update_args_field]
  pub description: String,

  #[response_field]
  #[create_args_field]
  #[update_args_field]
  pub proposed_changes: serde_json::Value,

  #[response_field]
  #[create_args_field]
  #[update_args_field]
  pub created_at: DateTime<Utc>,

  #[response_field]
  #[create_args_field]
  #[update_args_field]
  pub merged_at: Option<DateTime<Utc>>,
}

#[models(
  api_module = "class_course_api",
  server_module = "class_course_server",
  join = true
)]
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
#[diesel(belongs_to(super::class_server::Class))]
#[diesel(belongs_to(super::course_server::Course))]
#[diesel(primary_key(class_id, course_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ClassCourse {
  #[response_field]
  #[create_args_field]
  pub class_id: Uuid,

  #[response_field]
  #[create_args_field]
  pub course_id: Uuid,
}

#[models(api_module = "class_api", server_module = "class_server")]
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
#[diesel(belongs_to(super::organization_server::Organization))]
#[diesel(belongs_to(super::teacher_server::Teacher))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Class {
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

  #[response_field]
  #[create_args_field]
  #[update_args_field]
  pub year: i32,

  #[response_field]
  #[create_args_field]
  #[update_args_field]
  pub letter: String,

  #[response_field]
  #[create_args_field]
  #[update_args_field]
  pub organization_id: Uuid,

  #[response_field]
  #[create_args_field]
  #[update_args_field]
  pub teacher_id: Uuid,
}

#[models(api_module = "course_api", server_module = "course_server")]
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
#[diesel(belongs_to(super::subject_server::Subject))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Course {
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

  #[response_field]
  #[create_args_field]
  #[update_args_field]
  pub subject_id: Uuid,
}

#[models(api_module = "guide_api", server_module = "guide_server")]
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
#[diesel(belongs_to(super::lecture_server::Lecture))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Guide {
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

  #[response_field]
  #[create_args_field]
  #[update_args_field]
  pub lecture_id: Uuid,

  #[response_field]
  #[create_args_field]
  #[update_args_field]
  pub content: serde_json::Value,
}

#[models(api_module = "issue_api", server_module = "issue_server")]
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
#[diesel(belongs_to(super::teacher_server::Teacher))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Issue {
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

  #[response_field]
  #[create_args_field]
  #[update_args_field]
  pub teacher_id: Option<Uuid>,

  #[response_field]
  #[create_args_field]
  #[update_args_field]
  pub content_table: String,

  #[response_field]
  #[create_args_field]
  #[update_args_field]
  pub content_id: Uuid,

  #[response_field]
  #[create_args_field]
  #[update_args_field]
  pub created_at: DateTime<Utc>,

  #[response_field]
  #[create_args_field]
  #[update_args_field]
  pub resolved: Option<bool>,
}

#[models(api_module = "lecture_api", server_module = "lecture_server")]
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
#[diesel(belongs_to(super::course_server::Course))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Lecture {
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

  #[response_field]
  #[create_args_field]
  #[update_args_field]
  pub course_id: Uuid,
}

#[models(api_module = "message_api", server_module = "message_server")]
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
#[diesel(belongs_to(super::teacher_server::Teacher))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Message {
  #[response_field]
  #[update_args_field]
  pub id: Uuid,

  #[response_field]
  #[create_args_field]
  #[update_args_field]
  pub teacher_id: Uuid,

  #[response_field]
  #[create_args_field]
  #[update_args_field]
  pub parent_table: String,

  #[response_field]
  #[create_args_field]
  #[update_args_field]
  pub parent_id: Uuid,

  #[response_field]
  #[create_args_field]
  #[update_args_field]
  pub content: String,

  #[response_field]
  #[create_args_field]
  #[update_args_field]
  pub created_at: DateTime<Utc>,
}

#[models(
  api_module = "organization_api",
  server_module = "organization_server"
)]
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
#[diesel(belongs_to(Organization, foreign_key = organization_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Organization {
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

  #[response_field]
  #[create_args_field]
  #[update_args_field]
  pub organization_id: Option<Uuid>,
}

#[models(api_module = "review_api", server_module = "review_server")]
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
#[diesel(belongs_to(super::change_server::Change))]
#[diesel(belongs_to(super::teacher_server::Teacher))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Review {
  #[response_field]
  #[update_args_field]
  pub id: Uuid,

  #[response_field]
  #[create_args_field]
  #[update_args_field]
  pub change_id: Uuid,

  #[response_field]
  #[create_args_field]
  #[update_args_field]
  pub teacher_id: Uuid,

  #[response_field]
  #[create_args_field]
  #[update_args_field]
  pub comment: Option<String>,

  #[response_field]
  #[create_args_field]
  #[update_args_field]
  pub approved: Option<bool>,

  #[response_field]
  #[create_args_field]
  #[update_args_field]
  pub created_at: DateTime<Utc>,
}

#[models(
  api_module = "student_class_api",
  server_module = "student_class_server",
  join = true
)]
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
#[diesel(belongs_to(super::student_server::Student))]
#[diesel(belongs_to(super::class_server::Class))]
#[diesel(primary_key(student_id, class_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StudentClass {
  #[response_field]
  #[create_args_field]
  pub student_id: Uuid,

  #[response_field]
  #[create_args_field]
  pub class_id: Uuid,
}

#[models(api_module = "student_api", server_module = "student_server")]
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
#[diesel(belongs_to(super::organization_server::Organization))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Student {
  #[response_field]
  #[update_args_field]
  pub id: Uuid,

  #[response_field]
  #[create_args_field]
  #[update_args_field]
  pub name: String,

  #[response_field]
  #[create_args_field]
  #[update_args_field]
  pub organization_id: Uuid,
}

#[models(api_module = "subject_api", server_module = "subject_server")]
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

#[models(api_module = "teacher_api", server_module = "teacher_server")]
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
#[diesel(belongs_to(super::organization_server::Organization))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Teacher {
  #[response_field]
  #[update_args_field]
  pub id: Uuid,

  #[response_field]
  #[create_args_field]
  #[update_args_field]
  pub name: String,

  #[response_field]
  #[create_args_field]
  #[update_args_field]
  pub organization_id: Uuid,
}
