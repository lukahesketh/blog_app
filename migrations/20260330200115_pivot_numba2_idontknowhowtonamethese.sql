CREATE TYPE bg_color AS ENUM ('red', 'blue', 'black', 'white', 'orange');

ALTER TABLE posts
ADD background_colour bg_color;

ALTER TABLE users
ADD bio VARCHAR(160);

ALTER TABLE users
ALTER COLUMN username TYPE VARCHAR(20);

CREATE TABLE comments (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    post_id INT NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    content VARCHAR(500) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX comments_post_created_idx
ON comments(post_id, created_at);
