#![warn(clippy::all)]

use std::{thread, time};

mod hashmap_life;
mod naive_life;

fn main() {
  let mut naive_state = naive_life::random_state(40, 40);

  println!("Naive Life:");
  naive_life::print_state(&naive_state);
  naive_life::tick(&mut naive_state);
  println!("---");
  naive_life::print_state(&naive_state);

  let mut hashmap_state = hashmap_life::random_state(40, 40);

  println!("HashMap Life:");
  hashmap_life::print_state(&hashmap_state, (0, 0), (39, 39));
  hashmap_life::tick(&mut hashmap_state);
  println!("---");
  hashmap_life::print_state(&hashmap_state, (0, 0), (39, 39));

  for _ in 1..100 {
    println!("");
    hashmap_life::tick(&mut hashmap_state);
    hashmap_life::print_state(&hashmap_state, (0, 0), (39, 39));

    thread::sleep(time::Duration::from_millis(500));
  }
}
