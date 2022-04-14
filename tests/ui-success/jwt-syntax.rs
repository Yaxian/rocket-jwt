#[macro_use]
extern crate rocket_jwt;

use jsonwebtoken::{decode, DecodingKey, Validation};

const SECRET_KEY: &'static str = "secret_key";

#[jwt(SECRET_KEY)]
struct UserClaim {
    id: String,
}

#[jwt("secret", exp = 1024)]
struct UserClaim2 {
  id: String,
}

fn main() {
    let claim = UserClaim {
        id: String::from("123"),
    };

    let token = UserClaim::sign(claim);
    let jwt_claim = UserClaim::decode(token.clone());

    let decode_claim = decode::<UserClaimJwtClaim>(
        &token,
        &DecodingKey::from_secret((SECRET_KEY).as_bytes()),
        &Validation::default(),
    );

    if let Ok(decode_claim) = decode_claim {
        assert_eq!(decode_claim.claims.user.id, String::from("123"));
    }

    if let Ok(jwt_claim) = jwt_claim {
        assert_eq!(jwt_claim.user.id, String::from("123"));
    }
}
