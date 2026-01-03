use mik_sdk::prelude::*;

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
