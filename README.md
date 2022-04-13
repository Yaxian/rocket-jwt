# rocket-jwt

for rocket@0.4


```rust
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_jwt::jwt;

#[jwt("secret")]
pub struct UserClaim {
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

#[get("/uer_id")]
fn get_uer_id_from_jwt(user: UserClaim) -> String {
    format!("user id is {}", user.id)
}

fn main() {
    rocket::ignite()
        .attach(UserClaim::fairing())
        .mount("/", routes![index, get_uer_id_from_jwt])
        .launch();
}

```


### API

| attribute | type | description | default |
|----------|------|-------------|---|
|  | String | jwt secret key, required | |
| exp | Int | token expire in seconds | 2663280 *(one month)* |



### Run example

```
cargo run --example rocket-jwt-demo
```

1. get `jwt` token

```
curl http://localhost:8000
```

2. use `jwt` token

```
curl -H "Authorization: Bearer YOUR_JWT_TOKEN" http://localhost:8000/uer_id
```

## License

[MIT](LICENSE-MIT)