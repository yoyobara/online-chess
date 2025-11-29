DROP TRIGGER IF EXISTS users_set_updated_at ON users;
DROP FUNCTION IF EXISTS users_set_updated_at();

DROP TRIGGER IF EXISTS matches_set_updated_at ON matches;
DROP FUNCTION IF EXISTS matches_set_updated_at();

CREATE OR REPLACE FUNCTION set_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at := CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER users_set_updated_at
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE FUNCTION set_updated_at();

CREATE TRIGGER matches_set_updated_at
BEFORE UPDATE ON matches
FOR EACH ROW
EXECUTE FUNCTION set_updated_at();
