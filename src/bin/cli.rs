use inquire::Select;
use lc_3_po::{error::Lc3Error, image::Image, terminal::TerminalGuard, vm::VirtualMachine};
use std::{fmt::Display, fs::read_dir, os::fd::RawFd, path::PathBuf};

const STDIN: RawFd = 0;

struct Example {
    name: String,
    path: PathBuf,
}

impl Display for Example {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn main() -> Result<(), Lc3Error> {
    let mut examples: Vec<Example> = Vec::new();
    for entry in read_dir("examples")? {
        let entry = entry?;
        examples.push(Example {
            name: entry.file_name().to_string_lossy().into(),
            path: entry.path(),
        });
    }

    let image_file = Select::new("Select image to load: ", examples).prompt()?;

    let _terminal_guard = TerminalGuard::new(STDIN)?;

    let data = Image::read_image(&image_file.path)?;
    let parsed_image = Image::parse_image(&data)?;
    let mut vm = VirtualMachine::default();
    vm.load_image(parsed_image);
    vm.run()?;

    Ok(())
}
