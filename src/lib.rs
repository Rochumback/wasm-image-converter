use image::codecs::ico::IcoEncoder;
use image::codecs::{
    avif::AvifEncoder,
    bmp::BmpEncoder,
    jpeg::JpegEncoder,
    png::{CompressionType, FilterType, PngEncoder},
    webp::WebPEncoder,
};
use wasm_bindgen::prelude::*;

mod wasm_image;
use wasm_image::image_to;

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
