#![allow(warnings)]

mod bindings;
mod handlers;
mod types;

use bindings::exports::mik::core::handler::{self, Guest, Response};
use handlers::*;
use mik_sdk::prelude::*;
use types::*;

routes! {
    GET "/" => home,
    GET "/users" => list_users -> UserList,
    GET "/users/{id}" => get_user(path: Id) -> User,
    POST "/users" => create_user(body: CreateUser) -> User,
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
