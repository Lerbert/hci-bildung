table! {
    sessions (session_id) {
        session_id -> Varchar,
        user_id -> Int4,
        expires -> Timestamp,
    }
}

table! {
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
    users (id) {
        id -> Int4,
        username -> Varchar,
        password_hash -> Varchar,
    }
}

joinable!(sessions -> users (user_id));
joinable!(sheets -> users (owner_id));

allow_tables_to_appear_in_same_query!(sessions, sheets, users,);
