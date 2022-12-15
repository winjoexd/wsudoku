#![deny(missing_docs)]

//! A Wsudoku game.
use glutin_window::GlutinWindow;
use opengl_graphics::{OpenGL, GlGraphics};
use piston::event_loop::{EventSettings, Events};
use piston::{EventLoop, RenderEvent, WindowSettings};

pub use crate::gameboard::Gameboard;
mod gameboard;

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("WSudoku", (640, 480))
        .exit_on_esc(true)
        .graphics_api(opengl)
        .vsync(true);
    let mut window: GlutinWindow =
        settings.build().expect("Could not create window");
    let mut events = Events::new(EventSettings::new().lazy(true));
    let mut gl = GlGraphics::new(opengl);

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::{clear};

                clear([1.0; 4], g);
            });
        }
    }

    println!("{}", settings.get_exit_on_esc());
}
