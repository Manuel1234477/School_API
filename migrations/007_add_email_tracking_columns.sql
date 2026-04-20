ALTER TABLE applications
ADD COLUMN IF NOT EXISTS welcome_email_sent   BOOLEAN NOT NULL DEFAULT FALSE,
ADD COLUMN IF NOT EXISTS acceptance_email_sent BOOLEAN NOT NULL DEFAULT FALSE,
ADD COLUMN IF NOT EXISTS class_reminder_sent  BOOLEAN NOT NULL DEFAULT FALSE,
ADD COLUMN IF NOT EXISTS class_start_date     TIMESTAMPTZ;

-- Backfill NULLs on any pre-existing rows (safe no-op on a fresh DB)
UPDATE applications SET
    welcome_email_sent    = FALSE,
    acceptance_email_sent = FALSE,
    class_reminder_sent   = FALSE
WHERE welcome_email_sent IS NULL
   OR acceptance_email_sent IS NULL
   OR class_reminder_sent IS NULL;

-- Indexes for scheduler queries
CREATE INDEX IF NOT EXISTS idx_applications_welcome_email_sent
    ON applications(welcome_email_sent) WHERE welcome_email_sent = FALSE;

CREATE INDEX IF NOT EXISTS idx_applications_acceptance_email_sent
    ON applications(acceptance_email_sent) WHERE acceptance_email_sent = FALSE;

CREATE INDEX IF NOT EXISTS idx_applications_class_reminder_sent
    ON applications(class_reminder_sent) WHERE class_reminder_sent = FALSE;
