/*
Like naive_life in that we use a fixed size contiguous array to store cell states, but instead of just storing
alive/dead state as a boolean, we store that and the number of live neighbors the cell has. Anytime a new cell is born,
we add one to the neighbors count for each of its neighbors. This avoids having to count neighbors for cells in large
dead zones. Still limited to a fixed size world though...
*/

use rand::Rng;

enum ActiveBuffer {
  Front,
  Back,
}

struct WorldSize {
  height: usize,
  width: usize,
}

struct CellState {
  state: bool,
  neighbors: u8,
}

pub struct WorldState {
  active: ActiveBuffer,
  size: WorldSize,
  front: Vec<CellState>,
  back: Vec<CellState>,
}

pub fn random_state(height: usize, width: usize) -> WorldState {
  let mut rng = rand::thread_rng();

  let size = WorldSize {
    height: height,
    width: width,
  };

  let mut front: Vec<CellState> = Vec::new();
  let mut back: Vec<CellState> = Vec::new();

  for _ in 0..(width * height) {
    front.push(CellState {
      state: false,
      neighbors: 0,
    });
    back.push(CellState {
      state: false,
      neighbors: 0,
    });
  }

  for y in 0..size.height {
    for x in 0..size.width {
      if rng.gen() {
        birth(&mut front, &size, x, y);
      }
    }
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

      if vec[i].state {
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

pub fn coords(state: &WorldState) -> Vec<(usize, usize)> {
  let vec = match state.active {
    ActiveBuffer::Front => &state.front,
    ActiveBuffer::Back => &state.back,
  };

  let mut coords: Vec<(usize, usize)> = Vec::new();

  for y in 0..state.size.height {
    for x in 0..state.size.width {
      let i = y * state.size.height + x;

      if vec[i].state {
        coords.push((x, y))
      }
    }
  }

  coords
}

pub fn tick(state: &mut WorldState) {
  let (read, write) = match state.active {
    ActiveBuffer::Front => (&mut state.front, &mut state.back),
    ActiveBuffer::Back => (&mut state.back, &mut state.front),
  };

  for y in 0..state.size.height {
    for x in 0..state.size.width {
      let i = y * state.size.height + x;
      let cell_state = &read[i];
      let alive = cell_state.state;
      let num_neighbors = cell_state.neighbors;

      if num_neighbors == 3 || (alive && num_neighbors == 2) {
        birth(write, &state.size, x, y)
      }

      read[i].state = false;
      read[i].neighbors = 0;
    }
  }

  state.active = match state.active {
    ActiveBuffer::Front => ActiveBuffer::Back,
    ActiveBuffer::Back => ActiveBuffer::Front,
  };
}

fn birth(vec: &mut [CellState], size: &WorldSize, x: usize, y: usize) {
  // add one to each neighboring cell state's neighbors count.
  for dy in -1..=1 {
    for dx in -1..=1 {
      if dy == 0 && dx == 0 {
        continue;
      }
      let i = (y as i32 + dy) * size.height as i32 + (x as i32 + dx);
      if i < 0 || i >= (size.width * size.height) as i32 {
        continue;
      }

      vec[i as usize].neighbors += 1;
    }
  }

  // set the actual cell state
  let i = y * size.height + x;
  vec[i].state = true;
}
