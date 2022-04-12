# rocket-jwt

for rocket@0.4


```rust
use rocket_jwt::jwt;

#[jwt("secret_string")]
pub struct UserClaim {
  id: String,
}

```