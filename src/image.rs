use std::path::PathBuf;

use crate::{error::Lc3Error, memory::Address};

struct Image {
    origin: Address,
    words: Vec<u16>,
}

impl Image {
    /// Reads bytes from an LC-3 image file.
    ///
    /// # Errors
    ///
    /// - Returns [`Lc3Error::Io`] if the undelying file read operation fails.
    pub fn read_image(file_path: PathBuf) -> Result<Vec<u8>, Lc3Error> {
        Ok(std::fs::read(file_path)?)
    }

    /// Parses an LC-3 compiled image file to be loaded by the [`VirtualMachine`].
    ///
    /// # Errors
    ///
    /// - Returns [`Lc3Error::EmptyImageFile`] if the file has zero bytes.
    /// - Returns [`Lc3Error::OddImageLength`] if the image has an uneven number of bytes.
    #[allow(clippy::indexing_slicing)]
    pub fn parse_image(data: &[u8]) -> Result<Self, Lc3Error> {
        let (chunks, remainder) = data.as_chunks::<2>();

        if chunks.is_empty() {
            return Err(Lc3Error::EmptyImageFile);
        }

        if !remainder.is_empty() {
            return Err(Lc3Error::OddImageLength);
        }

        let origin = Address::from(u16::from_be_bytes(chunks[0]));

        let words: Vec<u16> = chunks[1..]
            .iter()
            .map(|chunk| u16::from_be_bytes(*chunk))
            .collect();

        Ok(Self { origin, words })
    }
}
