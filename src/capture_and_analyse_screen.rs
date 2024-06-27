use std::error::Error;
use screenshots::Screen;
use rusty_tesseract::{Args, Image};
use tempfile::TempDir;
use std::io::Cursor;
use screenshots::image::ImageOutputFormat;

pub fn capture_and_analyze_screen() -> Result<String, Box<dyn Error>> {
    let screen = Screen::all()?
        .first()
        .ok_or("No screens found")?
        .clone();

    let image = screen.capture()?;

    // Convert the image to PNG format in memory
    let mut png_data = Vec::new();
    let mut cursor = Cursor::new(&mut png_data);
    image.write_to(&mut cursor, ImageOutputFormat::Png)?;

    let temp_dir = TempDir::new()?;
    let temp_path = temp_dir.path().join("screenshot.png");
    std::fs::write(&temp_path, &png_data)?;

    let args = Args::default();
    let tesseract_image = Image::from_path(&temp_path)?;
    let text = rusty_tesseract::image_to_string(&tesseract_image, &args)?;

    println!("OCR Result: {}", text);

    Ok(text)
}