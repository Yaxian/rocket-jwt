#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_jwt::jwt;

static SECRET_KEY: &str = "secret_key";

#[jwt(SECRET_KEY)]
pub struct UserClaim {
    id: String,
}

#[jwt("secret_2", exp = 10)]
pub struct UserClaim2 {
    id: String,
}

#[get("/")]
fn index() -> String {
    let user_claim = UserClaim {
        id: format!("hello_rocket_jwt"),
    };
    let token = UserClaim::sign(user_claim);
    println!("{:#?}", UserClaim::decode(token.clone()));
    token
}

#[get("/user_id")]
fn get_uer_id_from_jwt(user: UserClaim) -> String {
    format!("user id is {}", user.id)
}

fn main() {
    rocket::ignite()
        .attach(UserClaim::fairing())
        .mount("/", routes![index, get_uer_id_from_jwt])
        .launch();
}
