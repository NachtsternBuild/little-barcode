//! # Barcode Detector
//! 
//! This library provides tools for automatically detecting and categorizing 
//! content typically stored in barcodes or QR codes.
//!
//! ## Core Concepts
//!
//! The heart of the library is the [`Detector`]. It uses optimized regular 
//! expressions to analyze input strings and classify them into the [`BarcodeContent`] enum 
//!
//!
//! ## Supported Formats
//!
//! The detector can currently distinguish the following formats:
//! * **Web URLs**: Recognizes `http` and `https` links.
//! * **EAN-13**: Standard barcode for retail products.
//! * **ISBN**: Book identification numbers (ISBN-10 and ISBN-13).
//! * **WIFI**: QR code configurations for wireless networks (`WIFI:S:...`).
//! * **vCard**: Business card information.
//!
//! ## Example
//!
//! ```rust
//! let detector = Detector::new();
//! let input = "[https://www.rust-lang.org](https://www.rust-lang.org)";
//!
//! match detector.detect(input) {
//!     BarcodeContent::Url(url) => println!("URL detected: {}", url),
//!     BarcodeContent::Unknown(text) => println!("Unknown format: {}", text),
//!     _ => println!("Other format detected"),
//! }
//! ```
//!
//! ## Error Handling and Validation
//! 
//! Please note that this detector is primarily based on **structure recognition**. 
//! For example, successful recognition as `Ean13` means that the format 
//! is correct, but does not necessarily validate the barcode's check digit.

use regex::Regex;

/// Represents the various structured data formats 
/// that may be contained in barcodes or QR codes.
#[derive(Debug, PartialEq, Clone)]
pub enum BarcodeContent {
    /// A standard web URL (e.g., `https://example.com`).
    Url(String),
    /// A 13-digit European Article Number (EAN).
    Ean13(String),
    /// An International Standard Book Number (ISBN), supports 10- and 13-digit formats.
    Isbn(String),
    /// Wi-Fi configuration data in the common QR code format.
    Wifi { 
        /// The name of the network (Service Set Identifier).
        ssid: String, 
        /// The network password.
        pass: String 
    },
    /// A business card in vCard format.
    VCard,
    /// Unknown or unstructured text format.
    Unknown(String),
}

/// A detector for identifying content in scanned barcodes.
///
/// `Detector` uses precompiled regular expressions to maximize efficiency 
/// during multiple scan operations.
///
/// # Performance
/// Since initializing the regex patterns is computationally intensive, an 
/// instance of `Detector` should be reused whenever possible, rather than 
/// creating a new one for each scan.
pub struct Detector {
    re_url: Regex,
    re_ean: Regex,
    re_isbn: Regex,
    re_wifi: Regex,
    re_vcard: Regex,
}

impl Detector {

	/// Creates a new instance of the detector and compiles all regex patterns.
    ///
    /// # Panics
    /// This method triggers a panic if the internal regular expressions are invalid.
    /// Since these are hardcoded, this only occurs in the event of software errors in the library.
    pub fn new() -> Self {
        Self {
            re_url: Regex::new(r"^(https?://[^\s/$.?#].[^\s]*)$").unwrap(),
            re_ean: Regex::new(r"^[0-9]{13}$").unwrap(),
            re_isbn: Regex::new(r"^(?:97[89][- ]?)?(?:[0-9][- ]?){9}[0-9xX]$").unwrap(),
            re_wifi: Regex::new(r"^WIFI:S:(?P<ssid>[^;]+);P:(?P<pass>[^;]+);;?$").unwrap(),
            re_vcard: Regex::new(r"(?s)^BEGIN:VCARD.*END:VCARD$").unwrap(),
        }
    }
	
	/// Analyzes an input string and attempts to match it to a known format.
    ///
    /// Before analysis, the input string is trimmed (leading and trailing whitespace is removed).
    ///
    /// # Examples
    ///
    /// ```
    /// use your_crate_name::{Detector, BarcodeContent};
    ///
    /// let detector = Detector::new();
    /// 
    /// // WIFI Example
    /// let wifi_data = "WIFI:S:MyHouse;P:secret123;;";
    /// let result = detector.detect(wifi_data);
    /// 
    /// if let BarcodeContent::Wifi { ssid, pass } = result {
    ///     assert_eq!(ssid, "MyHouse");
    ///     assert_eq!(pass, "secret123");
    /// }
    ///
    /// // URL Example
    /// let result = detector.detect("[https://rust-lang.org](https://rust-lang.org)");
    /// assert_eq!(result, BarcodeContent::Url("[https://rust-lang.org](https://rust-lang.org)".to_string()));
    /// ```
    pub fn detect(&self, input: &str) -> BarcodeContent {
        let input = input.trim();

        if self.re_url.is_match(input) {
            return BarcodeContent::Url(input.to_string());
        }

        if self.re_ean.is_match(input) {
            return BarcodeContent::Ean13(input.to_string());
        }

        if self.re_isbn.is_match(input) {
            return BarcodeContent::Isbn(input.to_string());
        }

        if self.re_vcard.is_match(input) {
            return BarcodeContent::VCard;
        }

        // special case for wifi with SSID and password
        if let Some(caps) = self.re_wifi.captures(input) {
            return BarcodeContent::Wifi {
                ssid: caps["ssid"].to_string(),
                pass: caps["pass"].to_string(),
            };
        }

        BarcodeContent::Unknown(input.to_string())
    }
}
