--! create_ticket (column_id?, position?)
INSERT INTO tickets (title, description, column_id, position, board_id, account_id, user_id) VALUES (:title, :description, :column_id, :position, :board_id, :account_id, :user_id) returning id;

--! get_ticket_by_id : Ticket(column_id?, position?)
SELECT id, title, description, column_id, position, account_id, user_id FROM tickets WHERE id = :id AND account_id = :account_id;

--! get_tickets_by_column_id : Ticket(column_id?, position?)
SELECT id, title, description, column_id, position, account_id, user_id FROM tickets WHERE column_id = :column_id AND account_id = :account_id ORDER BY position ASC;

--! get_board_tickets_by_board_id : Ticket(column_id?, position?)
SELECT id, title, description, column_id, position, account_id, user_id FROM tickets WHERE board_id = :board_id AND account_id = :account_id and column_id != null ORDER BY column_id ASC, position ASC;

--! get_backlog_tickets_by_board_id : Ticket(column_id?, position?)
SELECT id, title, description, column_id, position, account_id, user_id FROM tickets WHERE board_id = :board_id AND account_id = :account_id and column_id is null ORDER BY position ASC;

--! edit_ticket (column_id?)
UPDATE tickets SET title = :title, description = :description, column_id = :column_id WHERE id = :id AND account_id = :account_id;

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