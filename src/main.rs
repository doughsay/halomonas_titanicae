#![warn(clippy::all)]

// use std::{thread, time};

mod counting_life;
mod hashmap_life;
mod naive_life;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

pub struct App {
  gl: GlGraphics,
}

impl App {
  fn render(&mut self, args: &RenderArgs, coords: Vec<(usize, usize)>) {
    use graphics::*;

    const BACKGROUND: [f32; 4] = [0.1, 0.1, 0.25, 1.0];
    const FOREGROUND: [f32; 4] = [1.0, 1.0, 0.0, 1.0];

    let square = rectangle::square(0.0, 0.0, 9.5);
    // let (x, y) = (args.width / 2.0, args.height / 2.0);

    self.gl.draw(args.viewport(), |c, gl| {
      // Clear the screen.
      clear(BACKGROUND, gl);

      for (x, y) in coords {
        let transform = c
          .transform
          .trans((x as f64) * 10.0, (y as f64) * 10.0)
          .trans(-4.25, -4.25);

        // Draw a box rotating around the middle of the screen.
        rectangle(FOREGROUND, square, transform, gl);
      }
    });
  }

  fn update<'a>(&'a mut self, _args: &UpdateArgs) {}
}

fn main() {
  // naive life test
  // let mut naive_state = naive_life::random_state(40, 40);

  // println!("Naive Life:");
  // naive_life::print_state(&naive_state);
  // naive_life::tick(&mut naive_state);
  // println!("---");
  // naive_life::print_state(&naive_state);

  // hashmap life test
  // let mut hashmap_state = hashmap_life::random_state(40, 40);

  // println!("HashMap Life:");
  // hashmap_life::print_state(&hashmap_state, (0, 0), (39, 39));
  // hashmap_life::tick(&mut hashmap_state);
  // println!("---");
  // hashmap_life::print_state(&hashmap_state, (0, 0), (39, 39));

  // counting life test
  let mut counting_state = counting_life::random_state(80, 80);

  // println!("Counting Life:");
  // counting_life::print_state(&counting_state);
  // counting_life::tick(&mut counting_state);
  // println!("---");
  // counting_life::print_state(&counting_state);

  // // simple animation using counting life
  // for _ in 1..100 {
  //   println!("");
  //   counting_life::tick(&mut counting_state);
  //   counting_life::print_state(&counting_state);

  //   thread::sleep(time::Duration::from_millis(500));
  // }

  let opengl = OpenGL::V3_2;

  // Create an Glutin window.
  let mut window: Window = WindowSettings::new("spinning-square", [800, 800])
    .opengl(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();

  // Create a new game and run it.
  let mut app = App {
    gl: GlGraphics::new(opengl),
  };

  let mut events = Events::new(EventSettings::new());
  while let Some(e) = events.next(&mut window) {
    if let Some(r) = e.render_args() {
      counting_life::tick(&mut counting_state);
      let coords = counting_life::coords(&counting_state);
      app.render(&r, coords);
    }

    if let Some(u) = e.update_args() {
      app.update(&u);
    }
  }
}
