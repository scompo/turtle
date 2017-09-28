use std::f64;
use std::f64::consts::PI;

use radians::Radians;
use {Distance};

/// Represents the various supported speeds that the turtle can move at
///
/// See [`Turtle::set_speed` method](struct.Turtle.html#method.set_speed) for more information.
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub enum Speed {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Instant,
}

impl Speed {
    /// Converts a speed to its value as pixels per second
    pub fn to_absolute(self) -> Distance {
        use self::Speed::*;
        // Arbitrary values that can be tweaked
        // Just make sure to keep invariants like Five > Three, etc.
        match self {
            One => 10.,
            Two => 50.,
            Three => 100.,
            Four => 150.,
            Five => 200.,
            Six => 250.,
            Seven => 300.,
            Eight => 350.,
            Nine => 400.,
            Ten => 500.,
            Instant => f64::INFINITY,
        }
    }

    /// Converts a speed to its value as radians per second
    pub fn to_rotation(self) -> Radians {
        use self::Speed::*;
        // Arbitrary values that can be tweaked
        // Just make sure to keep invariants like Five > Three, etc.
        Radians::from_radians_value(match self {
            One => 0.3 * PI,
            Two => 0.6 * PI,
            Three => 0.9 * PI,
            Four => 1.2 * PI,
            Five =>  1.5 * PI,
            Six => 1.8 * PI,
            Seven => 2.1 * PI,
            Eight => 2.4 * PI,
            Nine => 2.7 * PI,
            Ten => 3.0 * PI,
            Instant => f64::INFINITY,
        })
    }
}

impl<'a> From<&'a str> for Speed {
    fn from(s: &'a str) -> Self {
        use Speed::*;

        match s {
            "slowest" => One,
            "slow" => Three,
            "normal" => Six,
            "fast" => Ten,
            "fastest" => Instant,
            _ => panic!("Invalid speed specified, use one of the words: 'slowest', 'slow', 'normal', 'fast', 'fastest'"),
        }
    }
}

impl From<i32> for Speed {
    fn from(n: i32) -> Self {
        use Speed::*;

        match n {
            1 => One,
            2 => Two,
            3 => Three,
            4 => Four,
            5 => Five,
            6 => Six,
            7 => Seven,
            8 => Eight,
            9 => Nine,
            10 => Ten,
            _ => Instant,
        }
    }
}