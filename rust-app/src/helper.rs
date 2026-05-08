//! # Barcode Toolkit
//!
//! A comprehensive library for recognizing, generating, and processing barcodes and QR codes.
//! 
//! This crate combines the power of the [`rxing`] engine with convenient 
//! logic for type detection and file management.
//!
//! ## Modules
//!
//! * [`handler`]: Responsible for generating image files (PNG) and reading files.
//! * [`regex`]: Provides intelligent content recognition (URLs, Wi-Fi, EAN, ISBN) using regular expressions.
//!
//! ## Quick Start
//!
//! The typical workflow involves creating a code and then analyzing its content:
//! ```rust
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let handler = BarcodeHandler::new();
//!     let detector = Detector::new();
//!
//!     // Generate a QR code
//!     let content = "WIFI:S:MyNetwork;P:secret123;;";
//!     let file_name = handler.create_code(content, true)?;
//!
//!     // Read a code from a file
//!     let (raw_text, format) = handler.read_any_code(&file_name)?;
//!
//!     // Classify the content intelligently
//!     match detector.detect(&raw_text) {
//!         BarcodeContent::Wifi { ssid, .. } => println!("Wi-Fi found: {}", ssid),
//!         BarcodeContent::Url(url) => println!("URL found: {}", url),
//!         _ => println!("Raw text: {}", raw_text),
//!     }
//!
//!     Ok(())
//! }
//! ```
pub mod regex_check;
pub mod handler;

pub use regex_check::*;
pub use handler::*;
