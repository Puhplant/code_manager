--! insert_refresh_token
INSERT INTO refresh_tokens (id, user_id, token, expires_at) VALUES (:id, :user_id, :token, :expires_at) RETURNING id;

--! create_user
INSERT INTO users (name, email, normalized_email, password_hash, account_id) VALUES (:name, :email, :normalized_email, :password_hash, :account_id);

--! get_user_by_email : UserPassword()
SELECT id, password_hash, account_id FROM users WHERE normalized_email = :email limit 1;

--! get_user_by_id : User()
SELECT id, name, email, normalized_email, account_id FROM users WHERE id = :id;

--! get_refresh_token_by_id : RefreshToken()
SELECT id, user_id, token, expires_at FROM refresh_tokens WHERE id = :id;

--! expire_refresh_token
UPDATE refresh_tokens SET expires_at = CASE WHEN expires_at < now() + interval '10 seconds' THEN expires_at ELSE now() + interval '10 seconds' END WHERE user_id = :user_id and id = :id;

--! delete_expired_refresh_tokens
DELETE FROM refresh_tokens WHERE expires_at < now() and user_id = :user_id;

