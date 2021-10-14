use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImageView, GrayImage, ImageBuffer, Luma, Pixel};
use qr_reader_phone::process_payload::{process_decoded_payload, InProgress, Ready};
use std::env;

fn grayify(a: DynamicImage) -> GrayImage {
    let mut gray_img: GrayImage = ImageBuffer::new(a.width(), a.height());
    for y in 0..a.height() {
        for x in 0..a.width() {
            let new_pixel = a.get_pixel(x, y).to_luma();
            gray_img.put_pixel(x, y, new_pixel);
        }
    }
    gray_img
}

fn process_qr_image(img: &ImageBuffer<Luma<u8>, Vec<u8>>) -> Result<Vec<u8>, String> {
    let mut qr_decoder = quircs::Quirc::new();
    let codes = qr_decoder.identify(img.width() as usize, img.height() as usize, img);
    match codes.last() {
        Some(Ok(code)) => match code.decode() {
            Ok(decoded) => match process_decoded_payload(decoded.payload, InProgress::None) {
                Ok(Ready::Yes(d)) => Ok(d),
                Ok(Ready::NotYet(_)) => {
                    Err("process_decoded_payload did not find an answer".to_string())
                }
                Err(e) => Err(format!("process_decoded_payload failed: {:?}", e)),
            },
            Err(e) => Err(format!("QR decode error: {:?}", e)),
        },
        Some(Err(e)) => Err(format!("QR extraction error: {:?}", e)),
        None => Err("No QR code found".to_string()),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let img = ImageReader::open(args[1].clone())
        .expect("file not found")
        .decode()
        .expect("parse fail");
    let gray = grayify(img);
    println!(
        "{:?}",
        process_qr_image(&gray).expect("QR processing failed")
    );
}
