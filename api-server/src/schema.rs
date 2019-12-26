table! {
    github_login_session_information (id) {
        id -> Uuid,
        session_id -> Uuid,
        csrf_token -> Text,
        pkce_verifier -> Text,
        return_url -> Text,
    }
}

table! {
    github_user_info (id) {
        id -> Int4,
        login -> Text,
        name -> Text,
        email -> Nullable<Text>,
        avatar_url -> Text,
        user_id -> Uuid,
    }
}

table! {
    organization_users (organization_id, user_id) {
        organization_id -> Uuid,
        user_id -> Uuid,
    }
}

table! {
    organizations (id) {
        id -> Uuid,
        name -> Text,
    }
}

table! {
    system_data (key) {
        key -> Text,
        content -> Bytea,
    }
}

table! {
    users (id) {
        id -> Uuid,
    }
}

joinable!(github_user_info -> users (user_id));
joinable!(organization_users -> organizations (organization_id));
joinable!(organization_users -> users (user_id));

allow_tables_to_appear_in_same_query!(
    github_login_session_information,
    github_user_info,
    organization_users,
    organizations,
    system_data,
    users,
);
