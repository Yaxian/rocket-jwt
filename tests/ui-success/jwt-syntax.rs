#[macro_use]
extern crate rocket_jwt;

use jsonwebtoken::{decode, DecodingKey, Validation};
use std::{
    thread,
    time::Duration,
};

fn main() {
    pass();
    invalid_signature();
    expired_signature()
}

const SECRET_KEY: &'static str = "secret_key";
#[jwt(SECRET_KEY)]
struct UserClaim {
    id: String,
}

fn pass() {
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

fn invalid_signature() {
    let claim = UserClaim {
        id: String::from("123"),
    };

    let token = UserClaim::sign(claim);

    let decode_claim = decode::<UserClaimJwtClaim>(
        &token,
        &DecodingKey::from_secret(("secret").as_bytes()),
        &Validation::default(),
    );

    if let Err(decode_claim) = decode_claim {
        assert_eq!(decode_claim.to_string(), "InvalidSignature");
    }
}

#[jwt("secret", exp = 3, leeway = 1)]
struct UserClaim2 {
    id: String,
}
fn expired_signature() {
    let claim2 = UserClaim2 {
        id: String::from("123"),
    };
    let token2 = UserClaim2::sign(claim2);
    thread::sleep(Duration::from_secs(5));
    let decode_claim2 = UserClaim2::decode(token2);

    assert_eq!(decode_claim2.is_err(), true);
    if let Err(decode_claim2) = decode_claim2 {
        assert_eq!(decode_claim2.to_string(), "ExpiredSignature");
    }
}