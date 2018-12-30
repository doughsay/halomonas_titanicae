#![warn(clippy::all)]

use std::{thread, time};

mod counting_life;
mod hashmap_life;
mod naive_life;

fn main() {
  // naive life test
  let mut naive_state = naive_life::random_state(40, 40);

  println!("Naive Life:");
  naive_life::print_state(&naive_state);
  naive_life::tick(&mut naive_state);
  println!("---");
  naive_life::print_state(&naive_state);

  // hashmap life test
  let mut hashmap_state = hashmap_life::random_state(40, 40);

  println!("HashMap Life:");
  hashmap_life::print_state(&hashmap_state, (0, 0), (39, 39));
  hashmap_life::tick(&mut hashmap_state);
  println!("---");
  hashmap_life::print_state(&hashmap_state, (0, 0), (39, 39));

  // counting life test
  let mut counting_state = counting_life::random_state(40, 40);

  println!("Naive Life:");
  counting_life::print_state(&counting_state);
  counting_life::tick(&mut counting_state);
  println!("---");
  counting_life::print_state(&counting_state);

  // simple animation using counting life
  for _ in 1..100 {
    println!("");
    counting_life::tick(&mut counting_state);
    counting_life::print_state(&counting_state);

    thread::sleep(time::Duration::from_millis(500));
  }
}
