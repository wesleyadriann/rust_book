struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}

fn main() {
  let mut user1 = User {
    username: String::from("foobaa"),
    email: String::from("foo@baa.baz"),
    sign_in_count: 1,
    active: true,
  };

  user1.email = String::from("baz@foo.baa");
}

fn create_user(email: String, username: String) -> User {
  User {
    email,
    username,
    active: true,
    sign_in_count: 1,
  }
}