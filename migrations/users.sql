create table users (
    id serial primary key,
    name varchar(255) not null,
    email varchar(320) not null unique,
    normalized_email varchar(320) not null unique,
    password_hash varchar(255) not null,
    account_id int not null,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp
);

create index idx_users_normalized_email on users (normalized_email);
create index idx_users_account_id on users (account_id);


create table refresh_tokens (
    id serial primary key,
    user_id int not null,
    token varchar(255) not null,
    expires_at timestamp not null,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp,
    constraint fk_refresh_tokens_users foreign key (user_id) references users (id)
);

create index idx_refresh_tokens_user_id_expires_at on refresh_tokens (user_id, expires_at);

create table accounts (
    id serial primary key,
    name varchar(255) not null,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp
);

alter table users add constraint fk_users_accounts foreign key (account_id) references accounts (id);