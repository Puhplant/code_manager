create table boards (
    id serial not null,
    name varchar(255) not null,
    account_id int not null,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp
);

alter table boards add constraint pk_boards primary key (account_id, id);
create index idx_boards_account_id on boards (account_id);
alter table boards add constraint fk_boards_accounts foreign key (account_id) references accounts (id);

create table columns (
    id serial not null,
    name varchar(255) not null,
    board_id int not null,
    account_id int not null,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp
);

alter table columns add constraint pk_columns primary key (account_id, id);
create index idx_columns_account_id_board_id on columns (account_id, board_id);
alter table columns add constraint fk_columns_boards foreign key (account_id, board_id) references boards (account_id, id);
alter table columns add constraint fk_columns_accounts foreign key (account_id) references accounts (id);
