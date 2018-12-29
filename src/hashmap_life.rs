use rand::Rng;
use std::collections::HashMap;

enum ActiveBuffer {
  Front,
  Back,
}

#[derive(PartialEq, Eq, Hash)]
struct Point {
  x: i32,
  y: i32,
}

pub struct WorldState {
  active: ActiveBuffer,
  front: HashMap<Point, bool>,
  back: HashMap<Point, bool>,
}

pub fn random_state(height: i32, width: i32) -> WorldState {
  let mut rng = rand::thread_rng();

  let mut front = HashMap::new();
  let back = HashMap::new();

  for y in 0..height {
    for x in 0..width {
      if rng.gen() {
        birth(&mut front, Point { x, y });
      }
    }
  }

  WorldState {
    active: ActiveBuffer::Front,
    front,
    back,
  }
}

pub fn print_state(state: &WorldState, from: (i32, i32), to: (i32, i32)) {
  let hashmap = match state.active {
    ActiveBuffer::Front => &state.front,
    ActiveBuffer::Back => &state.back,
  };

  let (from_x, from_y) = from;
  let (to_x, to_y) = to;

  for y in from_y..=to_y {
    for x in from_x..=to_x {
      match hashmap.get(&Point { x, y }) {
        Some(true) => print!("◻"),
        Some(false) => print!("◼"),
        None => print!("◼"),
      }
    }
    println!("");
  }
}

pub fn tick(state: &mut WorldState) {
  let (read, write) = match state.active {
    ActiveBuffer::Front => (&state.front, &mut state.back),
    ActiveBuffer::Back => (&state.back, &mut state.front),
  };

  write.clear();

  for (point, alive) in read {
    let num_neighbors = count_neighbors(read, &point);

    if num_neighbors == 3 || (*alive && num_neighbors == 2) {
      birth(
        write,
        Point {
          x: point.x,
          y: point.y,
        },
      )
    }
  }

  state.active = match state.active {
    ActiveBuffer::Front => ActiveBuffer::Back,
    ActiveBuffer::Back => ActiveBuffer::Front,
  };
}

fn count_neighbors(hashmap: &HashMap<Point, bool>, point: &Point) -> u8 {
  let mut count = 0u8;

  for dy in -1..=1 {
    for dx in -1..=1 {
      if dy == 0 && dx == 0 {
        continue;
      }

      if let Some(true) = hashmap.get(&Point {
        x: point.x + dx,
        y: point.y + dy,
      }) {
        count += 1;
      }
    }
  }

  count
}

fn birth(hashmap: &mut HashMap<Point, bool>, point: Point) {
  for dy in -1..=1 {
    for dx in -1..=1 {
      hashmap
        .entry(Point {
          x: point.x + dx,
          y: point.y + dy,
        })
        .or_insert(false);
    }
  }
  hashmap.insert(point, true);
}
