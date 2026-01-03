# mik-handler-template

Starter template for portable WASI HTTP handlers with [mik-sdk](https://github.com/dufeut/mik-sdk).

Build once, deploy to wasmtime, Spin, or wasmCloud.

## Quick Start

```bash
# 1. Use this template (click "Use this template" on GitHub, or:)
gh repo create my-api --template dufeut/mik-handler-template --clone
cd my-api

# 2. Setup dependencies
./setup.sh

# 3. Build & run locally
./build.sh
wasmtime serve -S cli=y service.wasm

# 4. Test it
curl http://localhost:8080/
curl http://localhost:8080/users
curl -X POST http://localhost:8080/users -d '{"name":"Alice","email":"alice@example.com"}'
```

## Project Structure

```
├── .github/workflows/
│   └── deploy.yml      # CI/CD → ghcr.io
├── src/lib.rs          # Your handler code
├── wit/world.wit       # WIT world definition
├── setup.sh            # Fetch WIT dependencies
├── build.sh            # Build + compose locally
├── Cargo.toml          # Dependencies
└── README.md
```

## Writing Handlers

Edit `src/lib.rs`:

```rust
use mik_sdk::prelude::*;

// Define types with validation
#[derive(Type)]
pub struct CreateUser {
    #[field(min = 1)]
    pub name: String,
    #[field(min = 5)]
    pub email: String,
}

// Define routes
routes! {
    GET "/" => home,
    GET "/users/{id}" => get_user(path: Id),
    POST "/users" => create_user(body: CreateUser),
}

fn home(_req: &Request) -> Response {
    ok!({ "status": "healthy" })
}

fn get_user(path: Id, _req: &Request) -> Response {
    ok!({ "id": path.id, "name": "Alice" })
}

fn create_user(body: CreateUser, _req: &Request) -> Response {
    ok!({ "id": random::uuid(), "name": body.name })
}
```

## Deployment

Deploy by pushing a version tag or manually triggering the workflow:

```bash
# Tag and push to trigger deploy
git tag v0.1.0
git push origin v0.1.0

# Or manually: Actions → Deploy → Run workflow
```

The workflow:
1. Builds your handler
2. Composes with the mik-bridge
3. Generates OpenAPI schema
4. Publishes to `ghcr.io/{owner}/{repo}:{version}`

Pull your component:

```bash
oras pull ghcr.io/your-org/my-api:0.1.0
```

## Local Development

```bash
./build.sh            # Build → dist/service.wasm + dist/openapi.json

# Run with any runtime
mik run dist/service.wasm
wasmtime serve -S cli=y dist/service.wasm
spin up --from dist/service.wasm

# Test
curl http://localhost:8080/
```

## Prerequisites

- Rust 1.89+

Tools (`cargo-component`, `wac`, `wasm-tools`) are auto-installed by `build.sh` if missing.

## Configuration

| Variable      | Location                       | Description                      |
| ------------- | ------------------------------ | -------------------------------- |
| `SDK_VERSION` | `.github/workflows/deploy.yml` | mik-sdk version for bridge + WIT |
| `version`     | `Cargo.toml`                   | Your handler version             |

## Learn More

- [mik-sdk documentation](https://github.com/dufeut/mik-sdk)
- [WASI HTTP specification](https://github.com/WebAssembly/wasi-http)
