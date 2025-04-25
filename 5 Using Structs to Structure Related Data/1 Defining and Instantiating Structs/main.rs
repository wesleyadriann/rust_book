struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}

struct Color(u32, u32, u32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
  let mut user1 = User {
    username: String::from("user1"),
    email: String::from("user1@baa.baz"),
    sign_in_count: 1,
    active: true,
  };

  user1.email = String::from("baz@foo.baa");

  let user2 = User {
    email: String::from("user2@baa.baz"),
    ..user1
  };

  let user3 = create_user(
    "user3@baa.baz".to_string(),
    "user3".to_string()
  );

  let black = Color(0, 0, 0);
  let origin = Point(0, 0, 0);

  let subject = AlwaysEqual;
}

fn create_user(email: String, username: String) -> User {
  User {
    email,
    username,
    active: true,
    sign_in_count: 1,
  }
}

