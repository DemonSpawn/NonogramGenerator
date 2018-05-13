use std::path::Path;

extern crate image;

pub fn read_image(image_path : &str)
{
    let path: &Path = Path::new(image_path);
    let res_image = image::open(&path).expect("Opening image failed");

    let kernel = [-1.0f32, -1.0, -1.0,
            -1.0, 8.0, -1.0,
            -1.0, -1.0, -1.0];

    let out_image = res_image.adjust_contrast(1000.00)
        .resize(30u32, 30u32, image::FilterType::Nearest)
        .filter3x3(&kernel)
        .to_luma_alpha();



    let mut out = Path::new("images_for_test/out.jpg");

    out_image.save(&mut out).expect("Saving image failed");
}
