#![allow(warnings)]

mod bindings;

use bindings::exports::mik::core::handler::{self, Guest, Response};
use mik_sdk::prelude::*;

// --- Types ---

#[derive(Type)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[derive(Type)]
pub struct CreateUser {
    #[field(min = 1)]
    pub name: String,
    #[field(min = 5)]
    pub email: String,
}

#[derive(Type)]
pub struct UserList {
    pub users: Vec<User>,
    pub total: i64,
}

// --- Routes ---

routes! {
    GET "/" => home,
    GET "/users" => list_users -> UserList,
    GET "/users/{id}" => get_user(path: Id) -> User,
    POST "/users" => create_user(body: CreateUser) -> User,
}

fn home(_req: &Request) -> Response {
    ok!({
        "name": "my-api",
        "version": "0.1.0"
    })
}

fn list_users(_req: &Request) -> Response {
    // Fake data - replace with real database call
    ok!({
        "users": [
            { "id": "1", "name": "Alice", "email": "alice@example.com" },
            { "id": "2", "name": "Bob", "email": "bob@example.com" }
        ],
        "total": 2
    })
}

fn get_user(path: Id, _req: &Request) -> Response {
    // Fake lookup - replace with real database call
    match path.as_str() {
        "1" => ok!({ "id": "1", "name": "Alice", "email": "alice@example.com" }),
        "2" => ok!({ "id": "2", "name": "Bob", "email": "bob@example.com" }),
        _ => not_found!("User not found"),
    }
}

fn create_user(body: CreateUser, _req: &Request) -> Response {
    let id = random::uuid();
    ok!({
        "id": id,
        "name": body.name,
        "email": body.email
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "generates openapi.json"]
    fn write_openapi_json() {
        std::fs::create_dir_all("dist").expect("Failed to create dist/");
        std::fs::write("dist/openapi.json", OPENAPI_JSON).expect("Failed to write openapi.json");
        println!("Generated dist/openapi.json ({} bytes)", OPENAPI_JSON.len());
    }
}
