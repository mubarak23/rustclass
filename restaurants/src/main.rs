use rand::Rng;
use std::collections::HashMap

fn main() {
  let mut map = HashMap::new();
  map.insert(1, 2);

  let secret_number = rand::threads_rng().gen_range(1..=100);
}