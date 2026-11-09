# Ryosuke Kawashima's Personal Website

## Contents

- **Curated LinkedIn Feed**: Daily LinkedIn posts rendered via validated LinkedIn embeds with personal commentary and featured highlights
- **Recommendations**: Curated collection of books, YouTube videos, and websites worth checking out, organized by category and tags
- **Blog & Insights**: Articles and notes on AI/ML, software engineering, and daily life in India

## Techstacks

- **Frontend**: Next.js 16 (App Router), React 19, TypeScript 5, Tailwind CSS 4
- **Backend**: Rust (Edition 2024), Axum 0.8, Tokio, SQLx, Serde, Tower HTTP (`cors`, `trace`), Argon2 (admin authentication)
- **Database**: PostgreSQL 17
- **Containerization & Proxy**: Docker, Docker Compose, Caddy (HTTPS & reverse proxy)
- **CI/CD**: GitHub Actions (automated Rust `fmt`/`clippy`/`test`, Next.js `lint`/`tsc`/build, and Docker image builds)
- **Cloud & IaC**: AWS (Lightsail / EC2, optional ECS Fargate + RDS), Terraform

## Local development

```bash
cp .env.example .env   # then set POSTGRES_PASSWORD
docker compose up --build
```

| Service | URL on your machine |
|---|---|
| Frontend | http://localhost:13000 |
| Backend health | http://localhost:18080/api/health |
| PostgreSQL | `localhost:15432` |

Ports are only bound to `127.0.0.1` and can be changed in `.env`
(`FRONTEND_HOST_PORT`, `BACKEND_HOST_PORT`, `POSTGRES_HOST_PORT`).
