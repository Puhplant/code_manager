--! create_ticket (column_id?, position?)
INSERT INTO tickets (title, description, column_id, position, board_id, account_id, user_id) VALUES (:title, :description, :column_id, :position, :board_id, :account_id, :user_id) returning id;

--! get_ticket_by_id : Ticket(column_id?, position?)
SELECT id, title, description, column_id, position, account_id, board_id, user_id FROM tickets WHERE id = :id AND account_id = :account_id;

--! get_tickets_by_column_id : Ticket(column_id?, position?)
SELECT id, title, description, column_id, position, account_id, board_id, user_id FROM tickets WHERE column_id = :column_id AND account_id = :account_id ORDER BY position ASC;

--! get_board_tickets_by_board_id : MinTicket(column_id?, position?)
SELECT id, title, column_id, position, board_id, account_id, user_id 
FROM (
    select id, title, column_id, position, board_id, account_id, user_id, row_number() over (partition by column_id order by position ASC) as row_number from tickets where board_id = :board_id AND account_id = :account_id and column_id != null
) sub WHERE row_number <= 10;

--! get_backlog_tickets_by_board_id : MinTicket(column_id?, position?)
SELECT id, title, column_id, position, account_id, board_id, user_id FROM tickets WHERE board_id = :board_id AND account_id = :account_id and column_id is null ORDER BY position ASC;

--! edit_ticket (column_id?)
UPDATE tickets SET title = :title, description = :description, column_id = :column_id WHERE id = :id AND account_id = :account_id;

--! move_ticket (column_id?, position?)
UPDATE tickets SET column_id = :column_id, position = :position WHERE id = :id AND account_id = :account_id;

--! delete_ticket
DELETE FROM tickets WHERE id = :id AND account_id = :account_id;

--! create_ticket_comment
INSERT INTO ticket_comments (ticket_id, content, account_id, user_id) VALUES (:ticket_id, :content, :account_id, :user_id) returning id;

--! get_ticket_comments_by_ticket_id : TicketComment()
SELECT id, ticket_id, content, account_id, user_id FROM ticket_comments WHERE ticket_id = :ticket_id AND account_id = :account_id ORDER BY created_at DESC;

--! edit_ticket_comment
UPDATE ticket_comments SET content = :content WHERE id = :id AND account_id = :account_id;

--! delete_ticket_comment
DELETE FROM ticket_comments WHERE id = :id AND account_id = :account_id;