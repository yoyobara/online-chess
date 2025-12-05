CREATE TYPE match_status_enum AS ENUM ('Matchmaking', 'Ongoing', 'White_won', 'Black_won', 'Draw');

ALTER TABLE matches
ADD COLUMN match_status match_status_enum NOT NULL,
ALTER COLUMN white_player_id DROP NOT NULL,
ALTER COLUMN black_player_id DROP NOT NULL;