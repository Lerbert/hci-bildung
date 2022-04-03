table! {
    use diesel::sql_types::*;
    use crate::db::sql_types::*;

    roles (user_id, role) {
        user_id -> Int4,
        role -> Role,
    }
}

table! {
    use diesel::sql_types::*;

    sessions (session_id) {
        session_id -> Varchar,
        user_id -> Int4,
        expires -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;

    sheets (id) {
        id -> Uuid,
        title -> Varchar,
        owner_id -> Int4,
        created -> Timestamptz,
        changed -> Timestamptz,
        tiptap -> Jsonb,
        trashed -> Nullable<Timestamptz>,
    }
}

table! {
    use diesel::sql_types::*;

    users (id) {
        id -> Int4,
        username -> Varchar,
        password_hash -> Varchar,
    }
}

joinable!(roles -> users (user_id));
joinable!(sessions -> users (user_id));
joinable!(sheets -> users (owner_id));

allow_tables_to_appear_in_same_query!(roles, sessions, sheets, users,);
