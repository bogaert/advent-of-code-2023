mod d1;
mod d2;
mod utils;

fn main() { // run using `cargo run d1`
  let args: Vec<String> = std::env::args().collect();
  let day = &args[1];

  match day.as_str() {
    "d1" => d1::solve(),
    _ => println!("Unknown day"),
  }
}