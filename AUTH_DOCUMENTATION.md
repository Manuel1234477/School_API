# Authentication System Documentation

## Overview

This School API implements a comprehensive role-based authentication system with support for three user roles:
- **Admin** - Full system access and management capabilities
- **Student** - Student-specific features and data access
- **Mentor** - Mentor-specific features and student guidance

## Architecture

### Layered Design

```
Controllers (Auth_controllers.rs)
    ↓
Services (auth_services.rs)
    ↓
Database (PostgreSQL with SQLx)
    ↓
Utils (JWT, Error Handling)
```

### Key Components

1. **Models** (`src/models/user.rs`)
   - User struct with role-based fields
   - Request/Response DTOs
   - JWT Claims structure

2. **Services** (`src/services/auth_services.rs`)
   - User registration with role validation
   - Login with password verification
   - Token refresh logic
   - Role-based authorization

3. **Controllers** (`src/controllers/Auth_controllers.rs`)
   - 6 authentication endpoints (2 per role)
   - Token management endpoints
   - User profile endpoints

4. **Utils** (`src/utils/`)
   - JWT token generation and verification
   - Error handling with proper HTTP status codes
   - Token extraction from headers

## API Endpoints

### Admin Authentication

#### Register Admin
```
POST /auth/admin/register
Content-Type: application/json

{
  "email": "admin@example.com",
  "password": "secure_password",
  "first_name": "John",
  "last_name": "Doe"
}

Response: 201 Created
{
  "user": {
    "id": "uuid",
    "email": "admin@example.com",
    "first_name": "John",
    "last_name": "Doe",
    "role": "admin",
    "is_active": true,
    "created_at": "2024-01-01T00:00:00Z"
  },
  "access_token": "eyJ0eXAiOiJKV1QiLCJhbGc...",
  "refresh_token": "eyJ0eXAiOiJKV1QiLCJhbGc...",
  "token_type": "Bearer",
  "expires_in": 3600
}
```

#### Login Admin
```
POST /auth/admin/login
Content-Type: application/json

{
  "email": "admin@example.com",
  "password": "secure_password"
}

Response: 200 OK
(Same as register response)
```

### Student Authentication

#### Register Student
```
POST /auth/student/register
Content-Type: application/json

{
  "email": "student@example.com",
  "password": "secure_password",
  "first_name": "Jane",
  "last_name": "Smith"
}

Response: 201 Created
(Same structure as admin)
```

#### Login Student
```
POST /auth/student/login
Content-Type: application/json

{
  "email": "student@example.com",
  "password": "secure_password"
}

Response: 200 OK
(Same structure as admin)
```

### Mentor Authentication

#### Register Mentor
```
POST /auth/mentor/register
Content-Type: application/json

{
  "email": "mentor@example.com",
  "password": "secure_password",
  "first_name": "Bob",
  "last_name": "Johnson"
}

Response: 201 Created
(Same structure as admin)
```

#### Login Mentor
```
POST /auth/mentor/login
Content-Type: application/json

{
  "email": "mentor@example.com",
  "password": "secure_password"
}

Response: 200 OK
(Same structure as admin)
```

### Common Endpoints

#### Refresh Access Token
```
POST /auth/refresh
Content-Type: application/json

{
  "refresh_token": "eyJ0eXAiOiJKV1QiLCJhbGc..."
}

Response: 200 OK
{
  "access_token": "eyJ0eXAiOiJKV1QiLCJhbGc...",
  "token_type": "Bearer",
  "expires_in": 3600
}
```

#### Get Current User Profile
```
GET /auth/me
Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGc...

Response: 200 OK
{
  "id": "uuid",
  "email": "user@example.com",
  "first_name": "John",
  "last_name": "Doe",
  "role": "admin",
  "is_active": true,
  "created_at": "2024-01-01T00:00:00Z"
}
```

#### Verify Token
```
POST /auth/verify
Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGc...

Response: 200 OK
{
  "valid": true,
  "user_id": "uuid",
  "email": "user@example.com",
  "role": "admin",
  "token_type": "access"
}
```

#### Logout
```
POST /auth/logout

Response: 200 OK
{
  "message": "Logged out successfully. Please discard the tokens on client side."
}
```

## Security Features

### Password Security
- Passwords are hashed using bcrypt with DEFAULT_COST (12 rounds)
- Passwords are never stored in plain text
- Password verification uses constant-time comparison

### JWT Tokens
- Access tokens expire in 1 hour (3600 seconds)
- Refresh tokens expire in 7 days (604800 seconds)
- Tokens include user ID, email, role, and token type
- Tokens are signed with a secret key (configurable via JWT_SECRET env var)

### Role-Based Access Control
- Each endpoint validates the user's role
- Attempting to login with wrong role returns 403 Forbidden
- Role validation happens at both controller and service layers

### Database Security
- Passwords stored as bcrypt hashes
- User email is unique (enforced by database constraint)
- Indexes on frequently queried fields (email, role, is_active)

## Error Handling

All errors return appropriate HTTP status codes:

| Error | Status Code | Description |
|-------|-------------|-------------|
| InvalidCredentials | 401 | Wrong email or password |
| UserNotFound | 404 | User doesn't exist |
| UserAlreadyExists | 409 | Email already registered |
| InvalidToken | 401 | Token is malformed or invalid |
| TokenExpired | 401 | Token has expired |
| Unauthorized | 401 | Missing or invalid authorization |
| Forbidden | 403 | User lacks required permissions |
| InvalidRole | 400 | Invalid role specified |
| DatabaseError | 500 | Database operation failed |
| InternalServerError | 500 | Unexpected server error |

## Configuration

### Environment Variables

Create a `.env` file in the project root:

```env
# Database Configuration
DATABASE_URL=postgres://user:password@localhost:5432/school_db

# Server Configuration
PORT=3000
HOST=127.0.0.1

# JWT Configuration
JWT_SECRET=your-super-secret-jwt-key-change-this-in-production
JWT_ACCESS_TOKEN_EXPIRY=3600
JWT_REFRESH_TOKEN_EXPIRY=604800

# Logging
RUST_LOG=info
```

## Database Schema

The `users` table structure:

```sql
CREATE TABLE users (
    id UUID PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    role VARCHAR(50) NOT NULL CHECK (role IN ('admin', 'student', 'mentor')),
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL
);
```

Indexes:
- `idx_users_email` - Fast email lookups
- `idx_users_role` - Filter by role
- `idx_users_is_active` - Filter active users

## Usage Examples

### Register a New Admin
```bash
curl -X POST http://localhost:3000/auth/admin/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "admin@school.com",
    "password": "AdminPass123!",
    "first_name": "Admin",
    "last_name": "User"
  }'
```

### Login as Student
```bash
curl -X POST http://localhost:3000/auth/student/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "student@school.com",
    "password": "StudentPass123!"
  }'
```

### Get Current User (with token)
```bash
curl -X GET http://localhost:3000/auth/me \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN"
```

### Refresh Token
```bash
curl -X POST http://localhost:3000/auth/refresh \
  -H "Content-Type: application/json" \
  -d '{
    "refresh_token": "YOUR_REFRESH_TOKEN"
  }'
```

## Middleware Integration

The authentication middleware is available in `src/middlewares/auth_middleware.rs`:

- `auth_middleware` - Extracts and validates JWT tokens
- `role_middleware` - Validates user has required role

These can be applied to protected routes to enforce authentication and authorization.

## Best Practices

1. **Always use HTTPS** in production
2. **Rotate JWT_SECRET** periodically
3. **Store tokens securely** on the client (HttpOnly cookies recommended)
4. **Implement token blacklisting** for logout functionality
5. **Use strong passwords** (enforce in client validation)
6. **Monitor failed login attempts** for security
7. **Implement rate limiting** on auth endpoints
8. **Log authentication events** for audit trails

## Future Enhancements

- [ ] Token blacklisting for logout
- [ ] Rate limiting on auth endpoints
- [ ] Email verification for new accounts
- [ ] Password reset functionality
- [ ] Two-factor authentication (2FA)
- [ ] OAuth2/OpenID Connect integration
- [ ] Session management
- [ ] Audit logging
