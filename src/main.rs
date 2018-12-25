#![warn(clippy::all)]

use std::{thread, time};

mod naive_life;

fn main() {
  let mut state = naive_life::random_state(40, 40);

  naive_life::print_state(&state);

  for _ in 1..100 {
    println!("");
    naive_life::tick(&mut state);
    naive_life::print_state(&state);

    thread::sleep(time::Duration::from_millis(500));
  }
}
