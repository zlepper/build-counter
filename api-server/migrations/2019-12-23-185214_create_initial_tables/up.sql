create table organizations
(
    id   uuid primary key not null,
    name text             not null
);

create table users
(
    id uuid primary key not null
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
    pkce_verifier text not null,
    return_url    text not null
);

create index github_login_session_information_idx on github_login_session_information (session_id, csrf_token);

create table github_user_info
(
    id         integer not null primary key,
    login      text    not null,
    name       text    not null,
    email      text,
    avatar_url text    not null,
    user_id    uuid    not null references users (id)
);


create index github_user_info_idx_user_id on github_user_info (user_id);
create index github_user_info_idx_for_finding_users on github_user_info (login, name, email);

create table system_data (
    key text not null primary key,
    content bytea not null
);