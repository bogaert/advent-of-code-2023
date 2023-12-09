mod d1;
mod d2;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> { // run using `cargo run d1`
  let args: Vec<String> = std::env::args().collect();
  let day = &args[1];

  match day.as_str() {
    "d1" => d1::solve(),
    _ => {
      println!("Unknown day");
      Ok(())
    }
  }
}