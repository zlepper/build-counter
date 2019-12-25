create table organizations
(
    id   uuid primary key not null,
    name text             not null
);

create table users
(
    id   uuid primary key not null,
    name text             not null
);

create table organization_users
(
    organization_id uuid not null references organizations (id),
    user_id         uuid not null references users (id),
    primary key (organization_id, user_id)
);

create table github_login_session_information
(
    id            uuid not null primary key,
    session_id    uuid not null,
    csrf_token    text not null,
    pkce_verifier text not null
);

create index github_login_session_information_idx on github_login_session_information (session_id, csrf_token);