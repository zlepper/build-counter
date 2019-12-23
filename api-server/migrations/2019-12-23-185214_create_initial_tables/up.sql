create table tenants
(
    id   uuid primary key not null,
    name text             not null
);

create table users
(
    id        uuid primary key not null,
    name      text             not null,
    email     text             not null,
    password  text             not null,
    tenant_id uuid             not null references tenants (id),
    unique (email, tenant_id)
);