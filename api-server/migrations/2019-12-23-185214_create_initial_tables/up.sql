create table organizations
(
    id   uuid primary key not null,
    name text             not null
);

create table users
(
    id       uuid primary key not null,
    name     text             not null
);

create table organization_users
(
    organization_id uuid not null references organizations (id),
    user_id   uuid not null references users (id),
    primary key (organization_id, user_id)
);
