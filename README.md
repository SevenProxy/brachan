# brachan

A simple imageboard/chan built in Rust, focused on simplicity, performance and ease of maintenance.

## Goal

An imageboard-style platform with:

- Board creation
- Thread creation
- Thread replies
- Image uploads
- Pagination and bump limit
- Moderation and post deletion
- Spam and abuse protection

The project starts simple and evolves as needed. No overengineering.

## Stack

**Backend**

- Rust
- [Axum](https://github.com/tokio-rs/axum) — HTTP framework
- Tokio — async runtime
- SQLx — database access
- PostgreSQL — database

**Frontend**

- HTML, CSS and vanilla JavaScript
- Server-side rendering (SSR)
- Templates via Askama or Maud

**Infrastructure**

- Docker and Docker Compose (development)
- Local storage initially, S3 in the future

## Layout

```
zahard/
├── src/
│   ├── main.rs        # server bootstrap
│   ├── routes/        # route registration
│   └── infra/         # controllers and external access
│       └── controlller/
├── Cargo.toml
└── Cargo.lock
```

Flow:

```
HTTP Request → Axum → Handler → Service → DB / Storage
```

Handlers deal with HTTP. Business rules live in services.

## Routes

```
GET  /                     # Home
GET  /:board               # Board
GET  /:board/thread/:id    # Thread

POST /:board/thread        # Create thread
POST /:board/thread/:id    # Create reply

POST /mod/delete/:id       # Delete post
```

The structure may change as the project evolves.

## Development

```bash
# Run
cargo run

# Tests
cargo test

# Formatting
cargo fmt

# Linter
cargo clippy

# Build
cargo build --release
```

All commands above run inside `zahard/`.

CI runs `cargo fmt --check`, `cargo clippy --deny warnings`, `cargo test` and `cargo doc` on every push.

## Database

PostgreSQL is the primary source of data. Schema changes use migrations in `migrations/`. Never change the database in production without a matching migration.

## Security

Uploads are treated as untrusted content. As needed:

- Rate limiting and flood control
- Upload size limits and MIME type validation
- Content sanitization/escaping
- IP hashing for moderation mechanisms
- Validation of all inputs
- Security headers and CSRF

## Principles

- Simple code, no premature abstractions
- Idiomatic Rust
- No unnecessary dependencies
- SSR before adding a frontend framework
- Every new feature considers security and abuse

## License

MIT
