-- Create OTP records table
CREATE TABLE IF NOT EXISTS otp_records (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    otp_code VARCHAR(6) NOT NULL,
    otp_type VARCHAR(50) NOT NULL, -- 'login', 'password_reset', 'email_verification'
    is_used BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    CONSTRAINT valid_otp_type CHECK (otp_type IN ('login', 'password_reset', 'email_verification'))
);

-- Create indexes for faster queries
CREATE INDEX idx_otp_user_id ON otp_records(user_id);
CREATE INDEX idx_otp_type ON otp_records(otp_type);
CREATE INDEX idx_otp_expires_at ON otp_records(expires_at);
CREATE INDEX idx_otp_user_type ON otp_records(user_id, otp_type);
