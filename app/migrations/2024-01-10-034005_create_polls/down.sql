DROP TYPE IF EXISTS poll_kind CASCADE;

DROP TYPE IF EXISTS poll_state CASCADE;

DROP TABLE poll_votes;

DROP TABLE poll_choices;

DROP TABLE polls;

DROP INDEX IF EXISTS poll_votes_poll_id;

DROP INDEX IF EXISTS poll_choices_poll_id_value;

DROP FUNCTION update_poll_state();