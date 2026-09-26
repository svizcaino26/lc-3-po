use std::path::Path;

use lc_3_po::{error::Lc3Error, image::Image, vm::VirtualMachine};

fn main() -> Result<(), Lc3Error> {
    let image_path = Path::new("2048.obj");
    let data = Image::read_image(image_path)?;
    let parsed_image = Image::parse_image(&data)?;
    let mut vm = VirtualMachine::default();
    vm.load_image(parsed_image);
    vm.run()?;
    Ok(())
}
