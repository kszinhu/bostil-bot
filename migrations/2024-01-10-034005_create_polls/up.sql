CREATE TYPE poll_kind AS ENUM ('single_choice', 'multiple_choice');

CREATE TABLE polls (
  id UUID PRIMARY KEY,
  name VARCHAR(50) NOT NULL,
  description TEXT,
  kind poll_kind NOT NULL,
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
  value VARCHAR(50) NOT NULL,
  label VARCHAR(25) NOT NULL,
  description TEXT,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  poll_id UUID NOT NULL REFERENCES polls(id) ON
  DELETE CASCADE,
    PRIMARY KEY (poll_id, value)
);

CREATE TABLE poll_votes (
  user_id BIGINT NOT NULL,
  choice_value VARCHAR(50) NOT NULL,
  poll_id UUID NOT NULL,
  voted_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  FOREIGN KEY (choice_value, poll_id) REFERENCES poll_choices(value, poll_id) ON
  DELETE CASCADE,
    PRIMARY KEY (user_id, choice_value, poll_id)
);