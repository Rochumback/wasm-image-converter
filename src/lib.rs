use std::io::Cursor;

use image::codecs::ico::IcoEncoder;
use image::{
    codecs::{
        avif::AvifEncoder,
        bmp::BmpEncoder,
        gif::GifEncoder,
        jpeg::JpegEncoder,
        png::{CompressionType, FilterType, PngEncoder},
        tiff::TiffEncoder,
        webp::WebPEncoder,
    },
    load_from_memory_with_format, AnimationDecoder, Frame, Frames,
};
use wasm_bindgen::prelude::*;

mod wasm_image;
use wasm_image::{get_image_format, image_to};

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn get_preview(image: &[u8]) -> Result<Vec<u8>, JsError> {
    let mut converted_image: Vec<u8> = Vec::new();
    let encoder = PngEncoder::new_with_quality(
        &mut converted_image,
        CompressionType::Fast,
        FilterType::NoFilter,
    );
    image_to(image.to_vec(), encoder)?;
    Ok(converted_image)
}

#[wasm_bindgen]
pub fn image_to_png(image: Vec<u8>) -> Result<Vec<u8>, JsError> {
    let mut converted_image: Vec<u8> = Vec::new();
    let encoder = PngEncoder::new_with_quality(
        &mut converted_image,
        CompressionType::Best,
        FilterType::NoFilter,
    );
    image_to(image, encoder)?;
    Ok(converted_image)
}

#[wasm_bindgen]
pub fn image_to_jpeg(image: Vec<u8>) -> Result<Vec<u8>, JsError> {
    let mut converted_image: Vec<u8> = Vec::new();
    let encoder = JpegEncoder::new_with_quality(&mut converted_image, 95);
    image_to(image, encoder)?;
    Ok(converted_image)
}

#[wasm_bindgen]
pub fn image_to_webp(image: Vec<u8>) -> Result<Vec<u8>, JsError> {
    let mut converted_image: Vec<u8> = Vec::new();
    let encoder = WebPEncoder::new_lossless(&mut converted_image);
    image_to(image, encoder)?;
    Ok(converted_image)
}

#[wasm_bindgen]
pub fn image_to_avif(image: Vec<u8>) -> Result<Vec<u8>, JsError> {
    let mut converted_image: Vec<u8> = Vec::new();
    let encoder = AvifEncoder::new_with_speed_quality(&mut converted_image, 10, 95);
    image_to(image, encoder)?;
    Ok(converted_image)
}

#[wasm_bindgen]
pub fn image_to_ico(image: Vec<u8>) -> Result<Vec<u8>, JsError> {
    let mut converted_image: Vec<u8> = Vec::new();
    let encoder = IcoEncoder::new(&mut converted_image);
    image_to(image, encoder)?;
    Ok(converted_image)
}

#[wasm_bindgen]
pub fn image_to_bmp(image: Vec<u8>) -> Result<Vec<u8>, JsError> {
    let mut converted_image: Vec<u8> = Vec::new();
    let encoder = BmpEncoder::new(&mut converted_image);
    image_to(image, encoder)?;
    Ok(converted_image)
}

#[wasm_bindgen]
pub fn images_to_gif(image: Vec<u8>) -> Result<Vec<u8>, JsError> {
    let mut converted_image: Vec<u8> = Vec::new();
    let mut encoder = GifEncoder::new_with_speed(&mut converted_image, 1);
    let image_format = match get_image_format(&image) {
        Ok(format) => format,
        Err(message) => return Err(JsError::new(&message.to_string())),
    };
    let image = load_from_memory_with_format(&image, image_format)?;
    _ = encoder.encode_frame(Frame::new(image.to_rgba8()));
    drop(encoder);
    Ok(converted_image)
}

#[wasm_bindgen]
pub fn log_object(object: JsValue) {
    log(&format!("{:?}", object));
}
