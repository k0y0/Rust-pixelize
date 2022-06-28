use std::fs::File;
mod image;
use image::Image;

mod error;
mod pixel;
fn main() {

    let file = File::open("test.ff").unwrap();

    let img = Image::decode(file).unwrap();
    let new_img = img.pixelize(20).unwrap();
    let new_file = File::create("new_test.ff").unwrap();
    new_img.encode(new_file).unwrap();
}
