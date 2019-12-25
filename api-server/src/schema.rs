table! {
    github_login_session_information (id) {
        id -> Uuid,
        session_id -> Uuid,
        csrf_token -> Text,
        pkce_verifier -> Text,
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
    users (id) {
        id -> Uuid,
        name -> Text,
    }
}

joinable!(organization_users -> organizations (organization_id));
joinable!(organization_users -> users (user_id));

allow_tables_to_appear_in_same_query!(
    github_login_session_information,
    organization_users,
    organizations,
    users,
);
