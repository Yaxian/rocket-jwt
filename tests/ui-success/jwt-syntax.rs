#[macro_use] extern crate rocket_jwt;

#[jwt("secret")]
struct UserClaim {
  id: String,
}

// #[jwt("secret", exp = 1024)]
// struct UserClaim2 {
//   id: String,
// }

fn main() {
  let claim = UserClaim {
    id: String::from("123"),
  };

  let jwt_claim = UserClaim::decode(UserClaim::sign(claim));
  if let Ok(jwt_claim) = jwt_claim {
    assert_eq!(jwt_claim.user.id, String::from("123"));
  }
}