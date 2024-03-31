-- Add migration script here
-- 1. backfill null status to `confirmed` in `subscriptions` table
-- 2. and then mark status column as NOT NULL
-- Make 1 and 2 in a single transaction
BEGIN;
  UPDATE subscriptions
    SET status = 'confirmed'
    WHERE status IS NULL;
  ALTER TABLE subscriptions ALTER COLUMN status SET NOT NULL;
COMMIT;