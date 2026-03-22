CREATE TYPE match_result_enum AS ENUM ('white_won', 'black_won', 'draw');

ALTER TABLE matches
ADD COLUMN ending_board JSON NOT NULL,
ADD COLUMN move_count INTEGER NOT NULL,
ADD COLUMN match_result match_result_enum NOT NULL,
DROP COLUMN match_status;

DROP TYPE match_status_enum;