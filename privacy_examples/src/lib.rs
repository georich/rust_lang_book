mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {
            ::outermost::middle_secret_function();
        }

        fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function();
    outermost::middle_secret_function();
    outermost::inside::inner_function();
    outermost::inside::secret_function();
}

pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {

            }
        }
    }
}

fn main() {
    a::series::of::nested_modules();
}
// can be rewritten
use a::series::of;
// use a::series::of::nested_modules;
// allows nested_modules() instead of of::nested_modules()
fn main() {
    of::nested_modules();
}

// can do similar with enums
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::{ Red, Yellow };

fn main() {
    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;
}

// or user glob
use TrafficLight::*;

fn main() {
    let red = Red;
    let yellow = Yellow;
    let green = Green;
}
