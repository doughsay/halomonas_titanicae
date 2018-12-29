use rand::Rng;

enum ActiveBuffer {
  Front,
  Back,
}

struct WorldSize {
  height: usize,
  width: usize,
}

pub struct WorldState {
  active: ActiveBuffer,
  size: WorldSize,
  front: Vec<bool>,
  back: Vec<bool>,
}

pub fn random_state(height: usize, width: usize) -> WorldState {
  let mut rng = rand::thread_rng();

  let size = WorldSize {
    height: height,
    width: width,
  };

  let mut front: Vec<bool> = Vec::new();
  let mut back: Vec<bool> = Vec::new();

  for _ in 0..(width * height) {
    front.push(rng.gen());
    back.push(false);
  }

  WorldState {
    active: ActiveBuffer::Front,
    size,
    front,
    back,
  }
}

pub fn print_state(state: &WorldState) {
  let vec = match state.active {
    ActiveBuffer::Front => &state.front,
    ActiveBuffer::Back => &state.back,
  };

  for y in 0..state.size.height {
    for x in 0..state.size.width {
      let i = y * state.size.height + x;

      if vec[i] {
        print!("◻");
      } else {
        print!("◼");
      }

      if x == (state.size.width - 1) {
        println!("");
      }
    }
  }
}

pub fn tick(state: &mut WorldState) {
  let (read, write) = match state.active {
    ActiveBuffer::Front => (&state.front, &mut state.back),
    ActiveBuffer::Back => (&state.back, &mut state.front),
  };

  for y in 0..state.size.height {
    for x in 0..state.size.width {
      let i = y * state.size.height + x;
      let num_neighbors = count_neighbors(read, state.size.width, state.size.height, x, y);

      if read[i] {
        // cell is alive, should it survive?
        write[i] = num_neighbors == 2 || num_neighbors == 3;
      } else {
        // cell is empty, should it be born?
        write[i] = num_neighbors == 3;
      }
    }
  }

  state.active = match state.active {
    ActiveBuffer::Front => ActiveBuffer::Back,
    ActiveBuffer::Back => ActiveBuffer::Front,
  };
}

fn count_neighbors(vec: &[bool], width: usize, height: usize, x: usize, y: usize) -> u8 {
  let mut count = 0u8;

  for dy in -1..=1 {
    for dx in -1..=1 {
      if dy == 0 && dx == 0 {
        continue;
      }
      let i = (y as i32 + dy) * height as i32 + (x as i32 + dx);
      if i < 0 || i >= (width * height) as i32 {
        continue;
      }

      if vec[i as usize] {
        count += 1;
      }
    }
  }

  count
}
