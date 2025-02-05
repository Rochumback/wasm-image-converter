use std::fmt::Display;

use image::{load_from_memory_with_format, ImageEncoder, ImageFormat};
use wasm_bindgen::prelude::*;

pub struct WasmImageFormat(ImageFormat);

#[derive(Debug)]
pub enum WasmImageError {
    UnknownFormatError(&'static str),
    BinaryDataError(&'static str),
}

impl Display for WasmImageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::UnknownFormatError(error) => write!(f, "{}", error),
            Self::BinaryDataError(error) => write!(f, "{}", error),
        }
    }
}

pub fn get_image_format(image_bin: &[u8]) -> Result<ImageFormat, WasmImageError> {
    let first_bytes = match image_bin.get(0..4) {
        Some(value) => value,
        None => return Err(WasmImageError::BinaryDataError("Binary data corrupted")),
    };
    let avif_bytes = match image_bin.get(4..12) {
        Some(value) => value,
        None => return Err(WasmImageError::BinaryDataError("Binary data corrupted")),
    };

    match *avif_bytes {
        [0x66, 0x74, 0x79, 0x70, 0x61, 0x76, 0x69, 0x66] => return Ok(ImageFormat::Avif),
        [0x66, 0x74, 0x79, 0x70, 0x6d, 0x69, 0x66, 0x31] => return Ok(ImageFormat::Avif),
        _ => (),
    }
    match first_bytes {
        [0x89, 0x50, 0x4e, 0x47] => Ok(ImageFormat::Png),
        [0xff, 0xd8, 0xff, 0xe0] => Ok(ImageFormat::Jpeg),
        [0x47, 0x49, 0x46, 0x38] => Ok(ImageFormat::Gif),
        [0x52, 0x49, 0x46, 0x46] => Ok(ImageFormat::WebP),
        [0x4e, 0x4e, 0x00, 0xa2] => Ok(ImageFormat::Tiff),
        [0x49, 0x49, 0x2a, 0x00] => Ok(ImageFormat::Tiff),
        [0x42, 0x4d] => Ok(ImageFormat::Bmp),
        [0x00, 0x00, 0x01, 0x00] => Ok(ImageFormat::Ico),

        _ => Err(WasmImageError::UnknownFormatError(
            "Unable to determine image format",
        )),
    }
}

impl From<&WasmImageFormat> for ImageFormat {
    fn from(value: &WasmImageFormat) -> Self {
        value.0
    }
}

impl From<WasmImageFormat> for ImageFormat {
    fn from(value: WasmImageFormat) -> Self {
        value.0
    }
}

pub fn image_to(image: Vec<u8>, encoder: impl ImageEncoder) -> Result<(), JsError> {
    let image_format = match get_image_format(&image) {
        Ok(format) => format,
        Err(message) => return Err(JsError::new(&message.to_string())),
    };
    _ = load_from_memory_with_format(&image, image_format)?.write_with_encoder(encoder);
    Ok(())
}
