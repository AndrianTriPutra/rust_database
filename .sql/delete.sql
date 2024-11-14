UPDATE books
SET deleted_at = NOW() AT TIME ZONE 'UTC'
WHERE uuid = 'abc';
