# mik-handler-template

Starter template for portable WASI HTTP handlers with [mik-sdk](https://github.com/dufeut/mik-sdk).

Build once, deploy to wasmtime, Spin, or wasmCloud.

## Quick Start

```bash
# 1. Use this template (click "Use this template" on GitHub, or:)
gh repo create my-api --template dufeut/mik-handler-template --clone
cd my-api

# 2. Build & run
./scripts/build.sh
mik run dist/service.wasm

# 3. Test it
curl http://localhost:8080/
curl http://localhost:8080/users
curl -X POST http://localhost:8080/users -d '{"name":"Alice","email":"alice@example.com"}'
```

## Project Structure

```
├── .github/workflows/
│   └── deploy.yml      # CI/CD → ghcr.io
├── src/
│   ├── lib.rs          # Routes + module wiring
│   ├── types.rs        # Request/response types
│   └── handlers.rs     # Route handlers
├── wit/world.wit       # WIT world definition
├── tests/
│   ├── api.test.mjs    # E2E tests
│   └── helpers.mjs     # Test utilities
├── scripts/
│   ├── build.sh        # Build + compose + OpenAPI
│   └── test.sh         # Run e2e tests
├── Cargo.toml          # Dependencies
└── README.md
```

## Writing Handlers

**`src/types.rs`** - Define your request/response types:

```rust
use mik_sdk::prelude::*;

#[derive(Type)]
pub struct CreateUser {
    #[field(min = 1)]
    pub name: String,
    #[field(min = 5)]
    pub email: String,
}
```

**`src/handlers.rs`** - Implement route handlers:

```rust
use crate::bindings::exports::mik::core::handler::{self, Response};
use crate::types::CreateUser;
use mik_sdk::prelude::*;

pub fn home(_req: &Request) -> Response {
    ok!({ "status": "healthy" })
}

pub fn get_user(path: Id, _req: &Request) -> Response {
    ok!({ "id": path, "name": "Alice" })
}

pub fn create_user(body: CreateUser, _req: &Request) -> Response {
    ok!({ "id": random::uuid(), "name": body.name })
}
```

**`src/lib.rs`** - Wire routes to handlers:

```rust
mod bindings;
mod handlers;
mod types;

use handlers::*;
use types::*;

routes! {
    GET "/" => home,
    GET "/users/{id}" => get_user(path: Id),
    POST "/users" => create_user(body: CreateUser),
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
./scripts/build.sh    # Build → dist/service.wasm + dist/openapi.json

# Run with any runtime
mik run dist/service.wasm
wasmtime serve -S cli=y dist/service.wasm
spin up --from dist/service.wasm
```

## Testing

```bash
./scripts/test.sh     # Builds, starts server, runs e2e tests
```

Tests use Node.js built-in test runner (Node 18+):

```
tests/api.test.mjs
  GET /
    ✔ returns api info
  GET /users
    ✔ returns user list
  GET /users/:id
    ✔ returns user by id
    ✔ returns 404 for unknown user
  POST /users
    ✔ creates a new user
```

Add tests in `tests/` using the helpers:

```js
import { get, post, assertOk, assertHas } from './helpers.mjs'

it('my test', async () => {
  const res = await get('/my-endpoint')
  assertOk(res)
  assertHas(res, 'key', 'value')
})
```

## Prerequisites

- Rust 1.89+

Tools (`cargo-component`, `wac`, `wasm-tools`) are auto-installed by `scripts/build.sh` if missing.

## Configuration

| Variable      | Location                       | Description                      |
| ------------- | ------------------------------ | -------------------------------- |
| `SDK_VERSION` | `.github/workflows/deploy.yml` | mik-sdk version for bridge + WIT |
| `version`     | `Cargo.toml`                   | Your handler version             |

## Learn More

- [mik-sdk documentation](https://github.com/dufeut/mik-sdk)
- [WASI HTTP specification](https://github.com/WebAssembly/wasi-http)
