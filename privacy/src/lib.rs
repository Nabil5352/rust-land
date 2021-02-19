// Privacy Example

mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {}
        fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function(); // will work
    outermost::middle_secret_function(); // will not work, as it is not public
    outermost::inside::inner_function(); // will not work, as module inside is private
    outermost::inside::secret_function(); // will not work, as module inside is private
}

// Scopes

// First example: use keyword

pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

use a::series::of::nested_modules;

fn main() {
    nested_modules();
}

// Second example: use keyword

enum TrafficLight {
    Red, Yelllow, Green
}

use TrafficLight::{Red, Yelllow};

fn second_main() {
    let red = Red;
    let yellow = Yelllow;
    let green = TrafficLight::Green;
}

// Second example update: glob

use TrafficLight::*;

fn second_update_main() {
    let red = Red;
    let yellow = Yelllow;
    let green = TrafficLight::Green;
}
