table! {
    tenants (id) {
        id -> Uuid,
        name -> Text,
    }
}

table! {
    users (id) {
        id -> Uuid,
        name -> Text,
        email -> Text,
        password -> Text,
        tenant_id -> Uuid,
    }
}

joinable!(users -> tenants (tenant_id));

allow_tables_to_appear_in_same_query!(
    tenants,
    users,
);
