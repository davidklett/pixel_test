extern crate raster;

use raster::Image;

use raster::Color;

fn main(){

 use raster::editor;

// Create an image from file
let mut image = raster::open("C:/Users/DavidKlett.DESKTOP-HV02S6B/OneDrive - The University of Akron/Rust_Projects/bresenham/zippy.png").unwrap();

image.set_pixel(4,4, Color::red()).unwrap();
// Save it
raster::save(&image, "C:/Users/DavidKlett.DESKTOP-HV02S6B/OneDrive - The University of Akron/Rust_Projects/bresenham/zippy2.png");
}