# Online Chess Platform

A full-stack, real-time multiplayer chess platform built with a high-performance Rust backend and a modern React frontend.

## 🚀 Features

- **Real-time Gameplay**: Low-latency multiplayer chess matches powered by WebSockets.
- **Matchmaking**: Find opponents quickly with our built-in matchmaking system.
- **User Accounts**: Secure authentication system with persistent profiles.
- **Interactive Board**: Smooth drag-and-drop chess board experience.
- **Responsive Design**: Play on various screen sizes with a polished UI.
- **Match Persistence**: Historical match data stored for later review.

## 🛠 Tech Stack

### Backend (Rust)
- **Framework**: [Axum](https://github.com/tokio-rs/axum) for high-performance async routing.
- **Database**: [PostgreSQL](https://www.postgresql.org/) with [SQLx](https://github.com/launchbadge/sqlx) for type-safe queries.
- **In-Memory Store**: [Redis](https://redis.io/) for matchmaking and real-time state management.
- **Authentication**: JWT-based auth with Argon2 password hashing.
- **Game Logic**: Custom rust-based chess engine.

### Frontend (TypeScript/React)
- **Framework**: React 19.
- **Build System**: [Nx](https://nx.dev/) monorepo management.
- **State Management**: [TanStack Query](https://tanstack.com/query/latest) (React Query) and Context API.
- **Drag & Drop**: [@dnd-kit](https://dnd-kit.com/) for a fluid board interface.
- **Real-time**: [react-use-websocket](https://github.com/robtaussig/react-use-websocket) for reliable communication.
- **Styling**: SCSS Modules for scoped and maintainable styles.

## 📂 Project Structure

```text
├── apps/
│   ├── backend/          # Rust Axum server
│   └── online-chess/     # React frontend application
├── libs/                 # (Potential for shared logic)
├── docker-compose.yml    # Orchestration for local development
└── nx.json               # Monorepo configuration
```

## 🚦 Getting Started

### Prerequisites

- [Docker](https://www.docker.com/) & [Docker Compose](https://docs.docker.com/compose/)
- [Node.js](https://nodejs.org/) (for local frontend development)
- [Rust](https://www.rust-lang.org/) (for local backend development)

### Quick Start with Docker

The easiest way to get the entire stack running is using Docker Compose:

```bash
docker-compose up --build
```

This will spin up:
- **Frontend**: http://localhost:8080
- **Backend**: http://localhost:3000
- **PostgreSQL**: Internal port 5432
- **Redis**: Internal port 6379

### Local Development

If you prefer to run services individually for faster development cycles:

1. **Setup Infrastructure**:
   ```bash
   docker-compose up db redis -d
   ```

2. **Backend**:
   ```bash
   cd apps/backend
   # Ensure you have the database migrated
   # (Requires sqlx-cli: cargo install sqlx-cli)
   sqlx migrate run
   cargo run
   ```

3. **Frontend**:
   ```bash
   npm install
   npx nx serve online-chess
   ```

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
