## Day 1: Basic Server Running

### Goal: Get a minimal Axum server running with one text route and one JSON route.

What I built today

Setup Axum project with Tokio runtime.

Wrote everything inside main() to stay fast and avoid early abstraction.

## Added two routes:

GET / returns static text.

GET /health returns JSON { "status": "ok" }.

## Current code entrypoint

Server listens on 127.0.0.1:3000.

Using tokio::net::TcpListener and axum::serve.

## Commands

```bash
cargo run
curl http://127.0.0.1:3000/
curl http://127.0.0.1:3000/health
```