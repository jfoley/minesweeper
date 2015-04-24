extern crate piston;
extern crate conrod;
extern crate graphics;
extern crate opengl_graphics;
extern crate glutin_window;
// extern crate vecmath;

use std::path::Path;

use conrod::{
    Colorable,
    Sizeable,
    Positionable,

    Theme,
    Ui,
    Background,

    WidgetMatrix,
    Toggle,
    RightToggle,
    Label,
};
use conrod::color::{black, red};

use opengl_graphics::{GlGraphics, OpenGL};
use opengl_graphics::glyph_cache::GlyphCache;
use glutin_window::GlutinWindow;

use piston::event::*;
use piston::window::{WindowSettings, Size};

fn main() {
    let opengl = OpenGL::_3_2;
    let window = GlutinWindow::new(
        opengl,
        WindowSettings::new(
            "Minesweeper".to_string(),
            Size { width: 1100, height: 550 }
        )
        .exit_on_esc(true)
        .samples(4)
    );

    let event_iter = window.events().ups(180).max_fps(60);
    let mut gl = GlGraphics::new(opengl);

    let font_path = Path::new("./assets/NotoSans/NotoSans-Regular.ttf");
    let theme = Theme::default();
    let glyph_cache = GlyphCache::new(&font_path).unwrap();
    let mut ui = Ui::new(glyph_cache, theme);
    // let mut demo = DemoApp::new();

    for event in event_iter {
        ui.handle_event(&event);
        if let Some(args) = event.render_args() {
            gl.draw(args.viewport(), |_, gl| {
                Background::new().color(black()).draw(&mut ui, gl);

                let (rows, cols) = (10, 10);

                WidgetMatrix::new(rows, cols)
                    .dimensions(32.0 * rows as f64, 32.0 * cols as f64)
                    .each_widget(&mut ui, |ui, num, col, row, pos, dim| {
                        RightToggle::new(false)
                            .dim([32.0, 32.0])
                            .point(pos)
                            .rgba(0.5, 0.5, 0.5, 1.0)
                            .left_callback(|new_val: bool| {
                                println!("yo");
                            })
                            .set(num, ui);
                    });

                ui.draw(gl)
            });

        }
    }
}
