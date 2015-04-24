#![feature(core)]

extern crate piston;
extern crate graphics;
extern crate opengl_graphics;
extern crate sdl2_window;

use graphics::{ clear, rectangle };


use std::boxed::FnBox;

use std::option::Option;

const size: f64 = 100.0;

#[derive(Clone)]
struct Cell {
    x: usize,
    y: usize,
    id: usize,
}

impl Cell {
    fn in_rect(&self, xy: &[f64; 2]) -> bool {
        let x = xy[0];
        let y = xy[1];

        let x_min = size * self.x as f64;
        let y_min = size * self.y as f64;

        let x_max = x_min + size;
        let y_max = y_min + size;

        x >= x_min && x <= x_max && y >= y_min && y <= y_max
    }

    fn click(&self) {
        println!("click {}", self.id)
    }

    fn right_click(&self) {
        println!("right click {}", self.id)
    }
}

trait Renderable {
    fn render(&self, context: graphics::Context, graphics: &mut opengl_graphics::GlGraphics);
}

impl Renderable for Cell {
    fn render(&self, context: graphics::Context, graphics: &mut opengl_graphics::GlGraphics) {
        let rect = [
            size * self.x as f64,
            size * self.y as f64,
            size,
            size,
        ];

        rectangle([1.0, 0.0, 0.0, 1.0],
                  rect,
                  context.transform, graphics);
    }
}

struct Mouse {
    left: bool,
    right: bool,
    x: f64,
    y: f64,

    left_called: bool,
    right_called: bool,
}

fn main() {
  use piston::window::{ WindowSettings, Size };
  use piston::event::*;
  use sdl2_window::Sdl2Window;

  let opengl = opengl_graphics::OpenGL::_3_2;
  let window = Sdl2Window::new(
    opengl,
    WindowSettings::new("Hello Piston".to_string(),
                        Size { width: 300, height: 300 })
                       .exit_on_esc(true)
  );
  let ref mut gl = opengl_graphics::GlGraphics::new(opengl);

  let cell1 = Cell{x: 1, y: 0, id: 1};
  let cell2 = Cell{x: 0, y: 2, id: 2};
  let mut mouse = Mouse{left: false, right: false, x: 0.0, y: 0.0, left_called: false, right_called: false};

  for e in window.events() {
    if let Some(args) = e.render_args() {
      gl.draw(args.viewport(),
              |c, g| {
                clear([1.0; 4], g);
                cell1.render(c, g);
                cell2.render(c, g);
              }
      );
    }

    e.update(|dt| {
        for cell in [cell1.clone(), cell2.clone()].iter() {
            if cell.in_rect(&[mouse.x, mouse.y]) {
                if mouse.left && !mouse.left_called {
                    cell.click();
                    mouse.left_called = true;
                }

                if mouse.right && !mouse.right_called {
                    cell.right_click();
                    mouse.right_called = true;
                }
            }
        }
    });

    e.mouse_cursor(|x, y| {
        mouse.x = x;
        mouse.y = y;
    });


    e.press(|button_type| {
        if let piston::input::Button::Mouse(button) = button_type {
            match button {
                piston::input::mouse::MouseButton::Left => mouse.left = true,
                piston::input::mouse::MouseButton::Right => mouse.right = true,
                _ => println!("wat")
            }

        }

    });

    e.release(|button_type| {
        if let piston::input::Button::Mouse(button) = button_type {
            match button {
                piston::input::mouse::MouseButton::Left => {
                    mouse.left = false;
                    mouse.left_called = false;
                },
                piston::input::mouse::MouseButton::Right => {
                    mouse.right = false;
                    mouse.right_called = false;
                },
                _ => println!("wat")
            }

        }

    });
  }
}
