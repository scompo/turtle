extern crate piston_window;
extern crate interpolation;
extern crate fps_clock;

mod turtle_window;

mod turtle;
mod speed;
mod color;
mod radians;
mod animation;
mod extensions;
mod renderer;
mod state;

pub use turtle::{Turtle, Point, Distance, Angle};
pub use speed::{Speed};
pub use color::{Color};