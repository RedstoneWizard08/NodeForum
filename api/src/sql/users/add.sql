INSERT INTO nodeforum.users(first_name, last_name, email, user_password, username)
VALUES ($1, $2, $3, $4, $5)
RETURNING *;
