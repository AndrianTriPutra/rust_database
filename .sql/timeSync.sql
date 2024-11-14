-- Step 1: create atau replace Function
CREATE OR REPLACE FUNCTION update_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    -- when INSERT, fill created_at and updated_at
    IF TG_OP = 'INSERT' THEN
        NEW.created_at := NOW() AT TIME ZONE 'UTC';
        NEW.updated_at := NOW() AT TIME ZONE 'UTC';

    -- when UPDATE, just update updated_at
    ELSIF TG_OP = 'UPDATE' THEN
        NEW.updated_at := NOW() AT TIME ZONE 'UTC';

        -- if deleted_at update from NULL to time, give value deleted_at with NOW()
        IF NEW.deleted_at IS NOT NULL AND OLD.deleted_at IS NULL THEN
            NEW.deleted_at := NOW() AT TIME ZONE 'UTC';
        END IF;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Step 2: Membuat Trigger
CREATE TRIGGER set_timestamp
BEFORE INSERT OR UPDATE ON books
FOR EACH ROW
EXECUTE FUNCTION update_timestamp();
