// SPDX-License-Identifier: GPL-3.0-or-later
use base64::prelude::*;
use std::path::PathBuf;

pub fn create_thumbnail_base64(
    input_path: PathBuf,
    thumbnail_width: u32,
    thumbnail_height: u32,
) -> Result<String, image::ImageError> {
    // Load the image from the specified path
    let img = image::open(input_path)?;

    // Resize the image to the specified thumbnail dimensions
    let thumbnail = img.thumbnail(thumbnail_width, thumbnail_height);

    // Encode the thumbnail to PNG format in memory
    let mut buf = std::io::Cursor::new(Vec::new());
    thumbnail.write_to(&mut buf, image::ImageFormat::Png)?;

    // Convert the PNG bytes to a base64 string
    let base64_string = BASE64_STANDARD.encode(buf.get_ref());

    // Return the base64 string
    Ok(base64_string)
}
