CREATE TYPE poll_kind AS ENUM ('single_choice', 'multiple_choice');

CREATE TYPE poll_state AS ENUM ('created', 'started', 'stopped', 'ended');

-- Diesel don't support composite primary key yet, so we need to create a unique index

CREATE TABLE polls (
  id UUID PRIMARY KEY,
  name VARCHAR(50) NOT NULL,
  description TEXT,
  kind poll_kind NOT NULL,
  state poll_state NOT NULL DEFAULT 'created',
  timer BIGINT NOT NULL,
  thread_id BIGINT NOT NULL,
  embed_message_id BIGINT NOT NULL,
  poll_message_id BIGINT,
  started_at TIMESTAMP WITH TIME ZONE,
  ended_at TIMESTAMP WITH TIME ZONE,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  created_by BIGINT NOT NULL
);

CREATE TABLE poll_choices (
  poll_id UUID NOT NULL REFERENCES polls(id) ON DELETE CASCADE,
  value VARCHAR(50) NOT NULL,
  label VARCHAR(25) NOT NULL,
  description TEXT,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    PRIMARY KEY (poll_id, value)
);

CREATE UNIQUE INDEX poll_choices_poll_id_value ON poll_choices(poll_id, value);

CREATE TABLE poll_votes (
  user_id BIGINT NOT NULL,
  choice_value VARCHAR(50) NOT NULL,
  poll_id UUID NOT NULL REFERENCES polls(id) ON DELETE CASCADE,
  voted_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, choice_value, poll_id)
);

CREATE INDEX poll_votes_poll_id ON poll_votes(user_id, choice_value, poll_id);

-- Triggers
CREATE
OR REPLACE FUNCTION update_poll_state() RETURNS TRIGGER AS $$
BEGIN IF NEW .state = 'started' THEN NEW .started_at = NOW();

ELSIF NEW .state = 'ended' THEN NEW .ended_at = NOW();

END IF;

RETURN NEW;

END;

$$ LANGUAGE plpgsql;

CREATE TRIGGER update_poll_state BEFORE
UPDATE ON polls FOR EACH ROW EXECUTE PROCEDURE update_poll_state();