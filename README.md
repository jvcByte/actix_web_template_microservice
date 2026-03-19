# Actix Web Template Microservice

A production-ready REST API microservice template built with Rust's Actix Web framework. This template provides a solid foundation for building scalable web applications with authentication, user management, and database integration.

## Features

- **JWT Authentication**: Secure token-based authentication with access and refresh tokens
- **User Management**: Complete user registration, login, and profile management
- **PostgreSQL Integration**: Robust database layer using SeaORM
- **Password Security**: Argon2 hashing for secure password storage
- **Database Migrations**: Automated schema management with SeaORM migrations
- **Structured API**: Well-organized REST endpoints with proper routing
- **Environment Configuration**: Secure configuration management via environment variables
- **Logging**: Comprehensive logging with configurable levels

## Tech Stack

- **Actix Web 4.13.0**: High-performance web framework
- **SeaORM 1.1.19**: Type-safe ORM for Rust
- **PostgreSQL**: Reliable database system
- **JWT 10.3.0**: JSON Web Token authentication
- **Argon2 0.5.3**: Modern password hashing
- **Serde**: Serialization framework
- **Chrono**: Date/time handling
- **UUID**: Unique identifier generation

## Project Structure

```
actix_web_template_microservice/
├── src/
│   ├── api/
│   │   ├── auth/          # Authentication endpoints
│   │   ├── users/         # User management endpoints
│   │   ├── refresh_tokens/ # Token refresh endpoints
│   │   ├── home/          # Home/health endpoints
│   │   ├── routes.rs      # Main API routing
│   │   └── mod.rs
│   ├── shared/
│   │   ├── config/        # Configuration management
│   │   └── ...
│   └── main.rs            # Application entry point
├── entity/                # Database entity definitions
├── migration/             # Database migrations
├── Cargo.toml             # Rust dependencies
├── .env                   # Environment variables (not in repo)
└── README.md
```

## Getting Started

### Prerequisites

- Rust 1.70+ (edition 2021)
- PostgreSQL 12+
- Cargo (comes with Rust)

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/jvcByte/actix_web_template_microservice.git
   cd actix_web_template_microservice
   ```

2. **Set up environment variables**
   ```bash
   cp .env.example .env  # If you have an example file
   ```

   Configure your `.env` file:
   ```env
   DATABASE_URL=postgresql://username:password@localhost:5432/your_database
   JWT_SECRET=your_super_secret_jwt_key_here
   JWT_ACCESS_TOKEN_EXPIRATION_MINUTES=15
   JWT_REFRESH_TOKEN_EXPIRATION_DAYS=30
   ADDRESS=127.0.0.1
   PORT=8080
   RUST_LOG=debug
   ```

3. **Install dependencies**
   ```bash
   cargo build
   ```

4. **Run database migrations**
   ```bash
   # Navigate to migration directory
   cd migration
   cargo run

   # Or from root directory
   cargo run --bin migration
   ```

5. **Start the server**
   ```bash
   cargo run
   ```

The server will start on `http://127.0.0.1:8080` by default.

## API Endpoints

### Authentication
- `POST /auth/register` - User registration
- `POST /auth/login` - User login

### Users
- `GET /users/me` - Get current user profile (requires auth)
- `PUT /users/me` - Update current user profile (requires auth)

### Tokens
- `POST /refresh-tokens` - Refresh access token

### Health Check
- `GET /` - Service health check

## API Usage Examples

### User Registration
```bash
curl -X POST http://localhost:8080/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "johndoe",
    "email": "john@example.com",
    "password": "securepassword123"
  }'
```

### User Login
```bash
curl -X POST http://localhost:8080/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "john@example.com",
    "password": "securepassword123"
  }'
```

Response:
```json
{
  "access_token": "<JWT_ACCESS_TOKEN>",
  "token_type": "Bearer",
  "expires_in": 900,
  "refresh_token": "<REFRESH_TOKEN>",
  "user": {
    "id": "uuid",
    "name": "Alice",
    "email": "alice@example.com"
  }
}
```

### Access Protected Endpoint
```bash
curl -X GET http://localhost:8080/users/me \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN"
```

## Development

### Database Management

**Run migrations:**
```bash
cd migration
cargo run
```

**Generate new migration:**
```bash
cd migration
cargo run -- generate your_migration_name
```

**Rollback migrations:**
```bash
cd migration
cargo run -- down
```

### Testing

Run tests:
```bash
cargo test
```

### Logging

Configure log level via `RUST_LOG` environment variable:
- `error` - Only errors
- `warn` - Warnings and errors
- `info` - Info, warnings, and errors
- `debug` - Debug info and above
- `trace` - All logs

## Configuration

All configuration is managed through environment variables:

| Variable | Description | Default |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection string | Required |
| `JWT_SECRET` | Secret key for JWT signing | Required |
| `JWT_ACCESS_TOKEN_EXPIRATION_MINUTES` | Access token lifetime | 15 |
| `JWT_REFRESH_TOKEN_EXPIRATION_DAYS` | Refresh token lifetime | 30 |
| `ADDRESS` | Server bind address | 127.0.0.1 |
| `PORT` | Server port | 8080 |
| `RUST_LOG` | Log level | debug |

## Security Features

- **Password Hashing**: Argon2 algorithm for secure password storage
- **JWT Tokens**: Stateless authentication with configurable expiration
- **Input Validation**: Proper validation of user inputs
- **CORS**: Configurable Cross-Origin Resource Sharing (if needed)
- **Rate Limiting**: Can be added via middleware

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License or Apache License 2.0. See the [LICENSE-MIT](LICENSE-MIT) or [LICENSE-APACHE](LICENSE-APACHE) files for details.

## Author

**jvcByte** - [jvc8463@gmail.com](mailto:jvc8463@gmail.com)

## Acknowledgments

- [Actix Web](https://actix.rs/) - The web framework
- [SeaORM](https://www.sea-ql.org/SeaORM/) - The ORM
- [JWT](https://jwt.io/) - Token standard
