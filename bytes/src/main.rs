use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  let n_str: &str = args.last().unwrap();
  let n: i32 = n_str.parse::<i32>().unwrap_or(0);

  let bytes: Vec<u8> = (0..n).map(|_| {
    rand::random::<u8>()
  }).collect();

  println!("{}", hex::encode(bytes));
}

