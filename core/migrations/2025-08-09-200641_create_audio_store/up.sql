CREATE TABLE audios (
  id BIGINT PRIMARY KEY,
  user_id BIGINT NOT NULL,
  content BYTEA NOT NULL,
  caption TEXT,
  added_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

SELECT diesel_manage_updated_at('audios');
