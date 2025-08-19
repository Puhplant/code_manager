create table tickets (
    id serial not null,
    title varchar(255) not null,
    description text not null,
    column_id int,
    board_id int not null,
    position float null,
    account_id int not null,
    user_id int,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp
);

alter table tickets add constraint pk_tickets primary key (account_id, id);
create index idx_tickets_account_id_column_id_position on tickets (account_id, column_id, position);
create index idx_tickets_account_id_board_id on tickets (account_id, board_id);
create index idx_tickets_user_id on tickets (user_id);

alter table tickets add constraint fk_tickets_accounts foreign key (account_id) references accounts (id);
alter table tickets add constraint fk_tickets_boards foreign key (account_id, board_id) references boards (account_id, id);
alter table tickets add constraint fk_tickets_columns foreign key (account_id, column_id) references columns (account_id, id);

create table ticket_comments (
    id serial not null,
    ticket_id int not null,
    content text not null,
    account_id int not null,
    user_id int not null,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp
);

alter table ticket_comments add constraint pk_ticket_comments primary key (account_id, id);
create index idx_ticket_comments_user_id on ticket_comments (user_id);
create index idx_ticket_comments_account_id_ticket_id on ticket_comments (account_id, ticket_id);

alter table ticket_comments add constraint fk_ticket_comments_tickets foreign key (account_id, ticket_id) references tickets (account_id, id);
alter table ticket_comments add constraint fk_ticket_comments_users foreign key (user_id) references users (id);
alter table ticket_comments add constraint fk_ticket_comments_accounts foreign key (account_id) references accounts (id);
