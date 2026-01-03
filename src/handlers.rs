use crate::types::CreateUser;
use crate::bindings::exports::mik::core::handler::{self, Response};
use mik_sdk::prelude::*;

pub fn home(_req: &Request) -> Response {
    ok!({
        "name": "my-api",
        "version": "0.1.0"
    })
}

pub fn list_users(_req: &Request) -> Response {
    // Fake data - replace with real database call
    ok!({
        "users": [
            { "id": "1", "name": "Alice", "email": "alice@example.com" },
            { "id": "2", "name": "Bob", "email": "bob@example.com" }
        ],
        "total": 2
    })
}

pub fn get_user(path: Id, _req: &Request) -> Response {
    // Fake lookup - replace with real database call
    match path.as_str() {
        "1" => ok!({ "id": "1", "name": "Alice", "email": "alice@example.com" }),
        "2" => ok!({ "id": "2", "name": "Bob", "email": "bob@example.com" }),
        _ => not_found!("User not found"),
    }
}

pub fn create_user(body: CreateUser, _req: &Request) -> Response {
    let id = random::uuid();
    ok!({
        "id": id,
        "name": body.name,
        "email": body.email
    })
}
