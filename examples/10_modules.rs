//! Exercise 10: modules, `use`, and visibility (`pub`).
//! Real projects split code across modules and files; this exercise shows the
//! mechanics inside one file for simplicity.
//! Run: cargo run --example 10_modules

// A module groups related items. Everything is private by default; `pub`
// makes an item visible outside the module.
mod geometry {
    // Fields are private too — callers use `Circle::new` instead of building
    // the struct literally.
    pub struct Circle {
        radius: f64,
    }

    impl Circle {
        pub fn new(radius: f64) -> Circle {
            Circle { radius }
        }

        pub fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }
    }

    // Modules can nest.
    pub mod constants {
        pub const PI: f64 = std::f64::consts::PI;
    }
}

mod conversion {
    pub fn area_to_side(area: f64) -> f64 {
        (area / std::f64::consts::PI).sqrt()
    }
}

// `use` brings a path into scope so it can be written without the full prefix.
use conversion::area_to_side;
use geometry::Circle;

fn main() {
    // Full path works without any `use`.
    println!("pi = {}", geometry::constants::PI);

    let c = Circle::new(2.0);
    let area = c.area();
    println!("circle radius 2.0 -> area = {area:.4}");

    let side = area_to_side(area);
    println!("equivalent square side = {side:.4}");

    // In a multi-file project, `mod geometry;` in lib.rs would load
    // src/geometry.rs (or src/geometry/mod.rs) instead of an inline block.
}
