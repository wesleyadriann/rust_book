enum IpAddrKind {
  V4,
  V6
}

struct IpAddr {
  kind: IpAddrKind,
  address: String,
}

enum IpAddr2 {
  V4(u8, u8, u8, u8),
  V6(String)
}

fn main() {
  let four = IpAddrKind::V4;
  let six = IpAddrKind::V6;

  route(four);
  route(IpAddrKind::V6);

  let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
  };

  let loopback = IpAddr {
    kind: six,
    address: String::from("::1"),
  };

  let home2 = IpAddr2::V4(127, 0, 0, 1);
  let loopback2 = IpAddr2::V6(String::from("::1"));

  // 

  let some_number = Some(5);
  let some_char = Some('a');

  let none_number: Option<i32> = None;

  let x: i8 = 5;
  let y: Option<i8> = Some(5);

  // let sum = x + y;
  let sum = x + y.unwrap();
}

fn route(ip_kind: IpAddrKind) -> IpAddrKind {
  ip_kind
}