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
  UserClaim::fairing();

  let claim = UserClaim {
    id: String::from("123"),
  };

  UserClaim::decode(UserClaim::sign(claim));
}