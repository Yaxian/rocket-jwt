# rocket-jwt

jwt authorization for rocket@0.5.

rocket@0.4 see [v0.4](https://github.com/Yaxian/rocket-jwt/tree/v0.4).


```rust
#[macro_use]
extern crate rocket;

use rocket_jwt::jwt;

static SECRET_KEY: &str = "secret_key";
#[jwt(SECRET_KEY)]
pub struct UserClaim {
    id: String,
}

#[jwt("secret", exp = 100)]
pub struct UserClaimExp {
    id: String
}

#[jwt("secret", leeway = 10)]
pub struct UserClaimLeeway {
    id: String
}

// get token from cookie, key is `token`
#[jwt("secret", cookie = "token")]
pub struct UserClaimCookie {
    id: String
}

// get token from request query, key is `token`
#[jwt("secret", query = "token")]
pub struct UserClaimQuery {
    id: String
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

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, get_uer_id_from_jwt])
}

```


### API

| attribute | type | description | default |
|----------|------|-------------|---|
|  | String | jwt secret key, required | |
| exp | Int | token expire after seconds | 2592000 *(one month)* |
| leeway | Int | token expire leeway in seconds | 60 *(one minute)* |
| cookie | String | get token from cookie key, optional | |
| query | String | get token from query key, optional | |


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
curl -H "Authorization: Bearer YOUR_JWT_TOKEN" http://localhost:8000/user_id
```

## License

[MIT](LICENSE-MIT)