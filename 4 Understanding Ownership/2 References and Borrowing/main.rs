fn main() {
  let mut s1 = String::from("hello");

  let len = calculate_length(&s1);

  println!("The length of {s1} is {len}");

  modify(&mut s1);
}

fn calculate_length(s: &String) -> usize {
  s.len()
}

fn modify(s: &mut String) {
  s.push_str("world");
}