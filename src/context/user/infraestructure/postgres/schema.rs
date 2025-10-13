// @generated automatically by Diesel CLI.

pub mod user {
    diesel::table! {
        user.users (id) {
            created_at -> Timestamptz,
            updated_at -> Timestamptz,
            status -> Text,
            id -> Int8,
            uuid -> Uuid,
            #[max_length = 255]
            email -> Varchar,
            #[max_length = 255]
            name -> Nullable<Varchar>,
            email_verified_at -> Nullable<Timestamptz>,
        }
    }

    diesel::table! {
        user.users_identities (id) {
            created_at -> Timestamptz,
            updated_at -> Timestamptz,
            id -> Int8,
            uuid -> Uuid,
            subject -> Text,
            provider -> Text,
            user_id -> Int8,
        }
    }

    diesel::joinable!(users_identities -> users (user_id));

    diesel::allow_tables_to_appear_in_same_query!(users, users_identities,);
}
