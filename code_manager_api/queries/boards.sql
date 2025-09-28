--! create_board
INSERT INTO boards (name, account_id) VALUES (:name, :account_id) returning id;

--! get_boards_by_account_id : Board()
SELECT id, name FROM boards WHERE account_id = :account_id;

--! edit_board
UPDATE boards SET name = :name WHERE id = :id AND account_id = :account_id;

--! delete_board
DELETE FROM boards WHERE id = :id AND account_id = :account_id;

--! create_column
INSERT INTO columns (name, account_id) VALUES (:name, :account_id) returning id;

--! get_columns_by_board_id : Column()
SELECT id, name FROM columns WHERE board_id = :board_id AND account_id = :account_id;

--! edit_column
UPDATE columns SET name = :name WHERE id = :id AND account_id = :account_id AND board_id = :board_id;