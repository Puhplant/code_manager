--! insert_refresh_token
INSERT INTO refresh_tokens (user_id, token, expires_at) VALUES (:user_id, :token, :expires_at);


--! create_user
INSERT INTO users (name, email, normalized_email, password_hash, account_id) VALUES (:name, :email, :normalized_email, :password_hash, :account_id);

--! get_user_by_email : UserPassword()
SELECT id, password_hash, account_id FROM users WHERE normalized_email = :email limit 1;