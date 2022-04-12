#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

#[macro_use]
extern crate rocket;

extern crate rocket_jwt;

use rocket_jwt::jwt;

#[jwt("secret")]
pub struct UserClaim {
    id: String,
}

fn main() {
    rocket::ignite().attach(UserClaim::fairing()).launch();
}
