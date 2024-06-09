use image;
use image::io::Reader as ImageReader;
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use std::error::Error;

#[derive(Serialize, Deserialize)]
struct Input {
    image_url: String,
}

#[derive(Serialize, Deserialize)]
struct Output {
    code: String,
}

pub async fn reading_qr(input: String) -> Result<String, Box<dyn Error>> {
    let input = serde_json::from_str::<Input>(&input)?;
    let image = reqwest::get(input.image_url).await?.bytes().await?;

    let img = ImageReader::new(Cursor::new(image))
        .with_guessed_format()?
        .decode()?
        .to_luma8();

    let mut img = rqrr::PreparedImage::prepare(img);

    let grids = img.detect_grids();

    let (_, content) = grids[0].decode()?;

    let output = serde_json::to_string(&Output { code: content })?;

    Ok(output)
}
