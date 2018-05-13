mod generator;
extern crate piston_window;
use piston_window::*;


/***
This program shall be capable of reading in an imagefile, create a "contrast map", and from this map,
create a Nonogram.

1. Read in a picture
2. Where the contrast from the fields is above a limit, the pixel is set to black
3. From this map generate a Nonogram (maybe also the game?)
***/

fn main() {
 let image_path = "images_for_test/Lenna_(test_image).png";

    let mut window: PistonWindow =
        WindowSettings::new("High Contrast", [640, 480])
            .exit_on_esc(true).build().unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            clear([1.0; 4], graphics);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [0.0, 0.0, 100.0, 100.0],
                      context.transform,
                      graphics);
        });
    }

    generator::read_image(image_path);
}
