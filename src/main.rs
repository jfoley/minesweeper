#![feature(core)]
#![feature(rand)]

extern crate piston;
extern crate graphics;
extern crate opengl_graphics;
extern crate sdl2_window;

use graphics::{ clear, rectangle, Image, character, text, Transformed };

use opengl_graphics::glyph_cache::GlyphCache;
use opengl_graphics::{Texture};

mod nu_text;
use std::boxed::FnBox;
use std::path::Path;

use std::option::Option;

const size: f64 = 50.0;

mod board;
use board::Cell;
use board::Point;
use board::Board;

impl Cell {
    fn new(x: usize, y: usize) -> Cell {
        Cell{flagged: false, mine: false, visible: false, score: 0, coords: Point{x: x, y: y}}
    }

    fn in_rect(&self, xy: &[f64; 2]) -> bool {
        let x = xy[0];
        let y = xy[1];

        let x_min = size * self.coords.x as f64;
        let y_min = size * self.coords.y as f64;

        let x_max = x_min + size;
        let y_max = y_min + size;

        x >= x_min && x <= x_max && y >= y_min && y <= y_max
    }
}

trait Renderable {
    fn render(&self,
        context: graphics::Context,
        graphics: &mut opengl_graphics::GlGraphics,
        ctx: &mut RenderCtx,
        );
}

impl Renderable for Cell {
    fn render(&self,
        context: graphics::Context,
        graphics: &mut opengl_graphics::GlGraphics,
        ctx: &mut RenderCtx) {

        let rect = [
            size * self.coords.x as f64 + 1.0,
            size * self.coords.y as f64 + 1.0,
            size - 2.0,
            size - 2.0,
        ];

        let color = if self.visible {
            if self.mine {
                [0.7, 0.0, 0.0, 1.0]
            } else {
                [0.7, 0.7, 0.7, 1.0]
            }
        } else {
            [0.5, 0.5, 0.5, 1.0]
        };

        rectangle(color,
                  rect,
                  context.transform, graphics);

        if self.flagged {
            Image::new()
                .rect(rect)
                .draw(&ctx.flag, &context.draw_state, context.transform, graphics);

        }

        if self.visible {
            if self.mine {
                Image::new()
                    .rect(rect)
                    .draw(&ctx.bomb, &context.draw_state, context.transform, graphics);
            } else if self.score != 0 {
                text::Text::colored([0.0, 0.0, 0.0, 1.0], size as u32)
                    .draw(
                        &self.score.to_string(),
                        &mut ctx.cache,
                        &context.draw_state,
                        context.transform.trans(rect[0], rect[1] + size),
                        graphics
                    );
            }
        }
    }
}

impl Renderable for Board {
    fn render(&self,
        context: graphics::Context,
        graphics: &mut opengl_graphics::GlGraphics,
        ctx: &mut RenderCtx,
        ) {
        for x in 0..self.size() {
            for y in 0..self.size() {
                self.cells[x][y].render(context, graphics, ctx);
            }
        }
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

struct GameState {
    won: bool,
    lost: bool,
}

struct RenderCtx<'a> {
    bomb: Texture,
    flag: Texture,
    cache: GlyphCache<'a>
}

fn main() {
  use piston::window::{ WindowSettings, Size };
  use piston::event::*;
  use sdl2_window::Sdl2Window;

  let mines = vec![
  Point{x: 0, y: 0},
  Point{x: 1, y: 1},
  Point{x: 2, y: 2},
  ];

  let mut game_state = GameState{won: false, lost: false};
  let mut board = Board::new(3, mines);

  let opengl = opengl_graphics::OpenGL::_3_2;
  let window = Sdl2Window::new(
    opengl,
    WindowSettings::new("Hello Piston".to_string(),
                        Size {
                            width: (board.size() as f64 * size) as u32,
                            height: (board.size() as f64 * size) as u32
                            })
                       .exit_on_esc(true)
  );
  let ref mut gl = opengl_graphics::GlGraphics::new(opengl);

  let mut mouse = Mouse{left: false, right: false, x: 0.0, y: 0.0, left_called: false, right_called: false};

  let font_path = Path::new("./assets/Ubuntu-R.ttf");
  let mut ctx = RenderCtx{
      bomb: Texture::from_path(&Path::new("./assets/bomb.png")).unwrap(),
      flag: Texture::from_path(&Path::new("./assets/flag.png")).unwrap(),
      cache: GlyphCache::new(&font_path).unwrap()
  };

  for e in window.events() {
    if let Some(args) = e.render_args() {
      gl.draw(args.viewport(),
              |c, g| {
                clear([1.0; 4], g);
                board.render(c, g, &mut ctx);

                if game_state.won {
                    text::Text::colored([0.0, 0.0, 0.0, 1.0], 30)
                        .draw(
                            &"you won!".to_string(),
                            &mut ctx.cache,
                            &c.draw_state,
                            c.transform.trans(0.0, (size * 3.0) / 2.0),
                            g
                        );
                }

                if game_state.lost {
                    text::Text::colored([0.0, 0.0, 0.0, 1.0], 30)
                        .draw(
                            &"you lost...".to_string(),
                            &mut ctx.cache,
                            &c.draw_state,
                            c.transform.trans(0.0, (size * 3.0) / 2.0),
                            g
                        );
                }
              }
      );
    }

    e.update(|dt| {
        let board_x = (mouse.x / size) as usize;
        let board_y = (mouse.y / size) as usize;

        for x in 0..board.size() {
            for y in 0..board.size() {
                let cell = board.cells[x][y];

                if cell.in_rect(&[mouse.x, mouse.y]) {
                    if mouse.left && !mouse.left_called {
                        if board.uncover(board_x, board_y) {
                            game_state.lost = true;
                        }
                        mouse.left_called = true;
                    }

                    if mouse.right && !mouse.right_called {
                        if board.flag(x, y) {
                            game_state.won = true;
                        }
                        mouse.right_called = true;
                    }
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
