create table subjects (
  id uuid primary key,
  title text not null,
  description text not null
);

create table organizations (
  id uuid primary key,
  title text not null,
  description text not null,
  organization_id uuid references organizations(id)
);

create table courses (
  id uuid primary key,
  title text not null,
  description text not null,
  subject_id uuid not null references subjects(id) on delete cascade
);

create table lectures (
  id uuid primary key,
  title text not null,
  description text not null,
  course_id uuid not null references courses(id) on delete cascade
);

create table guides (
  id uuid primary key,
  title text not null,
  description text not null,
  lecture_id uuid not null references lectures(id) on delete cascade,
  content jsonb not null
);

create table teachers (
  id uuid primary key,
  name text not null,
  organization_id uuid not null references organizations(id) on delete cascade
);

create table students (
  id uuid primary key,
  name text not null,
  organization_id uuid not null references organizations(id) on delete cascade
);

create table classes (
  id uuid primary key,
  title text not null,
  description text not null,
  year integer not null,
  letter text not null,
  organization_id uuid not null references organizations(id) on delete cascade,
  teacher_id uuid not null references teachers(id) on delete restrict
);

create table student_classes (
  student_id uuid not null references students(id) on delete cascade,
  class_id uuid not null references classes(id) on delete cascade,
  primary key (student_id, class_id)
);

create table class_courses (
  class_id uuid not null references classes(id) on delete cascade,
  course_id uuid not null references courses(id) on delete cascade,
  primary key (class_id, course_id)
);

create table issues (
  id uuid primary key default gen_random_uuid(),
  title text not null,
  description text not null,
  teacher_id uuid references teachers(id) on delete set null,
  content_table text not null,
  content_id uuid not null,
  created_at timestamp with time zone not null default now(),
  resolved boolean
);

create table changes (
  id uuid primary key default gen_random_uuid(),
  teacher_id uuid not null references teachers(id) on delete set null,
  content_table text not null,
  content_id uuid not null,
  issue_id uuid null references issues(id) on delete set null,
  title text not null,
  description text not null,
  proposed_changes jsonb not null,
  created_at timestamp with time zone not null default now(),
  merged_at timestamp with time zone null
);

create table reviews (
  id uuid primary key default gen_random_uuid(),
  change_id uuid not null references changes(id) on delete cascade,
  teacher_id uuid not null references teachers(id) on delete cascade,
  comment text null,
  approved boolean,
  created_at timestamp with time zone not null default now(),
  unique (change_id, teacher_id) -- Fixed column names in unique constraint
);

create table messages (
  id uuid primary key default gen_random_uuid(),
  teacher_id uuid not null references teachers(id) on delete set null,
  parent_table text not null,
  parent_id uuid not null,
  content text not null,
  created_at timestamp with time zone not null default now()
);
