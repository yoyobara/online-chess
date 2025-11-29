DROP TABLE IF EXISTS matches;

CREATE TABLE waiting_room (
    waiting_player_id INTEGER PRIMARY KEY NOT NULL REFERENCES users(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP
);

CREATE TRIGGER waiting_room_set_updated_at
BEFORE UPDATE ON waiting_room
FOR EACH ROW
EXECUTE FUNCTION set_updated_at();
