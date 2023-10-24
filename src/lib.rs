mod utils;

use wasm_bindgen::prelude::*;

#[allow(unused_imports)]
use reqwest::{Client, Response};
use image::io::Reader as ImageReader;
use image::GenericImageView;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[wasm_bindgen]
extern "C" {}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct ImageRgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[wasm_bindgen(js_name = getImagePixel)]
pub async fn get_image_pixel(url: &str, x: u32, y: u32) -> JsValue {
    // 创建一个 HTTP 客户端
    let client = Client::new();

    // 请求远程图像
    let response = client.get(url).send().await.unwrap();

    // 将二进制数据转换为图片
    let img = ImageReader::new(Cursor::new(response.bytes().await.unwrap()))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();

    // 读取指定位置的像素
    let pixel = img.get_pixel(x, y);

    // 返回像素
    let data = ImageRgba {
        r: pixel.0[0],
        g: pixel.0[1],
        b: pixel.0[2],
        a: pixel.0[3],
    };

    serde_wasm_bindgen::to_value(&data).unwrap()
}
