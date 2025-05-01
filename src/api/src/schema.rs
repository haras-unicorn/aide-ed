// @generated automatically by Diesel CLI.

diesel::table! {
    changes (id) {
        id -> Uuid,
        teacher_id -> Uuid,
        content_table -> Text,
        content_id -> Uuid,
        issue_id -> Nullable<Uuid>,
        title -> Text,
        description -> Text,
        proposed_changes -> Jsonb,
        created_at -> Timestamptz,
        merged_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    class_courses (class_id, course_id) {
        class_id -> Uuid,
        course_id -> Uuid,
    }
}

diesel::table! {
    classes (id) {
        id -> Uuid,
        title -> Text,
        description -> Text,
        year -> Int4,
        letter -> Text,
        organization_id -> Uuid,
        teacher_id -> Uuid,
    }
}

diesel::table! {
    courses (id) {
        id -> Uuid,
        title -> Text,
        description -> Text,
        subject_id -> Uuid,
    }
}

diesel::table! {
    guides (id) {
        id -> Uuid,
        title -> Text,
        description -> Text,
        lecture_id -> Uuid,
        content -> Jsonb,
    }
}

diesel::table! {
    issues (id) {
        id -> Uuid,
        title -> Text,
        description -> Text,
        teacher_id -> Nullable<Uuid>,
        content_table -> Text,
        content_id -> Uuid,
        created_at -> Timestamptz,
        resolved -> Nullable<Bool>,
    }
}

diesel::table! {
    lectures (id) {
        id -> Uuid,
        title -> Text,
        description -> Text,
        course_id -> Uuid,
    }
}

diesel::table! {
    messages (id) {
        id -> Uuid,
        teacher_id -> Uuid,
        parent_table -> Text,
        parent_id -> Uuid,
        content -> Text,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    organizations (id) {
        id -> Uuid,
        title -> Text,
        description -> Text,
        organization_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    reviews (id) {
        id -> Uuid,
        change_id -> Uuid,
        teacher_id -> Uuid,
        comment -> Nullable<Text>,
        approved -> Nullable<Bool>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    student_classes (student_id, class_id) {
        student_id -> Uuid,
        class_id -> Uuid,
    }
}

diesel::table! {
    students (id) {
        id -> Uuid,
        name -> Text,
        organization_id -> Uuid,
    }
}

diesel::table! {
    subjects (id) {
        id -> Uuid,
        title -> Text,
        description -> Text,
    }
}

diesel::table! {
    teachers (id) {
        id -> Uuid,
        name -> Text,
        organization_id -> Uuid,
    }
}

diesel::joinable!(changes -> issues (issue_id));
diesel::joinable!(changes -> teachers (teacher_id));
diesel::joinable!(class_courses -> classes (class_id));
diesel::joinable!(class_courses -> courses (course_id));
diesel::joinable!(classes -> organizations (organization_id));
diesel::joinable!(classes -> teachers (teacher_id));
diesel::joinable!(courses -> subjects (subject_id));
diesel::joinable!(guides -> lectures (lecture_id));
diesel::joinable!(issues -> teachers (teacher_id));
diesel::joinable!(lectures -> courses (course_id));
diesel::joinable!(messages -> teachers (teacher_id));
diesel::joinable!(reviews -> changes (change_id));
diesel::joinable!(reviews -> teachers (teacher_id));
diesel::joinable!(student_classes -> classes (class_id));
diesel::joinable!(student_classes -> students (student_id));
diesel::joinable!(students -> organizations (organization_id));
diesel::joinable!(teachers -> organizations (organization_id));

diesel::allow_tables_to_appear_in_same_query!(
    changes,
    class_courses,
    classes,
    courses,
    guides,
    issues,
    lectures,
    messages,
    organizations,
    reviews,
    student_classes,
    students,
    subjects,
    teachers,
);
