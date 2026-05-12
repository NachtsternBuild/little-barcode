//! # Barcode Management System
//!
//! This module provides a high-level interface for creating and reading barcodes
//! and QR codes. It acts as a wrapper around the `rxing` library and automates
//! file management in the local download directory.
//!
//! ## Features
//! - **Generation**: Creates PNG files for QR codes and EAN-13 barcodes.
//! - **Automation**: Generates timestamp-based filenames.
//! - **Recognition**: Reads various code formats from image files.
//!
//! ## Example
//!
//! ```rust
//! let handler = BarcodeHandler::new();
//!
//! // create a QR code
//! if let Ok(file_name) = handler.create_code("[https://rust-lang.org](https://rust-lang.org)", true) {
//!     println!("File saved as: {}", file_name);
//!     
//!     // read the code back
//!     if let Ok((text, format)) = handler.read_any_code(&file_name) {
//!         println!("Content: {}, Format: {}", text, format);
//!     }
//! }
//! ```
use rxing::{
	helpers,
	BarcodeFormat, 
};
use rxing::MultiFormatWriter; 
use rxing::Writer;
use std::path::PathBuf;
use chrono::Local;
use image::{
	DynamicImage, 
	GrayImage, 
	Luma
};

/// Manages the creation and reading of barcodes within the user's download directory.
pub struct BarcodeHandler {
    download_dir: PathBuf,
}

impl BarcodeHandler {
	/// Creates a new instance of `BarcodeHandler`.
    ///
    /// The destination directory is automatically set to the current user's default download folder.
    ///
    /// # Panics
    /// The method panics if the user's home directory cannot be determined.
    pub fn new() -> Self {
        let mut path = home::home_dir().expect("Home-Verzeichnis nicht gefunden");
        path.push("Downloads");
        Self { 
        	download_dir: path 
        }
    }

    /// Creates an image file of a QR code or an EAN-13 barcode.
    ///
    /// The file is saved in PNG format with a timestamp in the filename.
    ///
    /// # Arguments
    /// * `content` - The string content to be encoded.
    /// * `is_qr` - `true` for a square QR code, `false` for a linear EAN-13 barcode.
    ///
    /// # Error
    /// Returns an error if encoding fails or the file 
    /// cannot be saved to the file system.
    ///
    /// # Example
    /// ```rust
    /// let handler = BarcodeHandler::new();
    /// let filename = handler.create_code("4006381333634", false).unwrap();
    /// ```
    pub fn create_code(&self, content: &str, is_qr: bool) -> Result<String, Box<dyn std::error::Error>> {
        let format = if is_qr {
            BarcodeFormat::QR_CODE
        } 
        
        else {
            BarcodeFormat::EAN_13
        };

        // adjust dimensions of the output image
        let (width, height) = if is_qr {
            (300, 300)
        } 
        
        else {
            (400, 150)
        };

        // generate code
        let writer = MultiFormatWriter::default();
        let bit_matrix = writer.encode(content, &format, width, height)?;

        // convert BitMatrix to image
        let mut gray_img = GrayImage::new(width as u32, height as u32);
        for y in 0..height {
            for x in 0..width {
                let pixel = if bit_matrix.get(x as u32, y as u32) { 
                	Luma([0u8]) 
                } 
                
                else { 
                	Luma([255u8]) 
                };
                gray_img.put_pixel(x as u32, y as u32, pixel);
            }
        }
        let img = DynamicImage::ImageLuma8(gray_img);

        // generate time stamp
        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        let prefix = if is_qr {
            "qr"
        } 
        
        else {
            "barcode"
        };

        let file_name = format!("{}_{}.png", prefix, timestamp);

        let mut file_path = self.download_dir.clone();
        file_path.push(&file_name);

        img.save(&file_path)?;

        Ok(file_name) // return the file name
    }

	/// Reads and decodes a barcode from an image file in the download directory.
    ///
    /// The format (QR, Code128, EAN, etc.) is automatically detected by 
    /// the underlying engine.
    ///
    /// # Return Value
    /// Returns a tuple `(Content, Format)` on success.
    ///
    /// # Error
    /// Fails if the file cannot be found or no valid code 
    /// can be recognized in the image.
    pub fn read_any_code(&self, file_name: &str) -> Result<(String, String), Box<dyn std::error::Error>> {
        let mut file_path = self.download_dir.clone();
        file_path.push(file_name);

        // rxing automatic detect QR, EAN or Code128        
        let path_str = file_path.to_str().ok_or("Ungültiger Dateipfad")?;
		let result = helpers::detect_in_file(path_str, None)?;

        let content = result.getText().to_string();
        let format = format!("{:?}", result.getBarcodeFormat());

        Ok((content, format))
    }
}
