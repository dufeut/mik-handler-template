import { describe, it } from "node:test";
import { get, post, assertOk, assertNotFound, assertHas } from "./helpers.mjs";

describe("GET /", () => {
  it("returns api info", async () => {
    const res = await get("/");
    assertOk(res);
    assertHas(res, "name", "my-api");
  });
});

describe("GET /users", () => {
  it("returns user list", async () => {
    const res = await get("/users");
    assertOk(res);
    assertHas(res, "total", 2);
  });
});

describe("GET /users/{id}", () => {
  it("returns user by id", async () => {
    const res = await get("/users/1");
    assertOk(res);
    assertHas(res, "name", "Alice");
    assertHas(res, "email", "alice@example.com");
  });

  it("returns 404 for unknown user", async () => {
    const res = await get("/users/999");
    assertNotFound(res);
  });
});

describe("POST /users", () => {
  it("creates a new user", async () => {
    const res = await post("/users", {
      name: "Test",
      email: "test@example.com",
    });
    assertOk(res);
    assertHas(res, "name", "Test");
    assertHas(res, "email", "test@example.com");
  });
});
