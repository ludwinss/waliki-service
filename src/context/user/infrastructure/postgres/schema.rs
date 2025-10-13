diesel::table! {
    use diesel::sql_types::*;

    users (id) {
        id -> Int8,
        uuid -> Uuid,
        email -> Varchar,
        name -> Nullable<Varchar>,
        status -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        email_verified_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    users_identities (id) {
        id -> Int8,
        uuid -> Uuid,
        subject -> Text,
        provider -> Text,
        user_id -> Int8,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(users_identities -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(users, users_identities);
