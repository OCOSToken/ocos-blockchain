//! # wallet::qr
//!
//! Generates QR codes for OCOS wallet elements (addresses, public keys, private keys).
//! Output formats include:
//! - UTF-8 terminal view (for dev tools)
//! - SVG data (for UI rendering)
//!
//! ⚠️ QR codes for private keys should only be used in secure development environments.

use qrcode::{QrCode, EcLevel};
use qrcode::render::svg;
use image::Luma;

/// Generates a QR code as a UTF-8 string (for terminal or CLI output).
pub fn to_terminal_qr(data: &str) -> String {
    let code = QrCode::with_error_correction_level(data.as_bytes(), EcLevel::Q).unwrap();
    code.render::<char>()
        .quiet_zone(false)
        .module_dimensions(2, 1)
        .build()
}

/// Generates a QR code as an SVG string for web UI or frontends.
pub fn to_svg_qr(data: &str, size: usize) -> String {
    let code = QrCode::with_error_correction_level(data.as_bytes(), EcLevel::Q).unwrap();
    code.render()
        .min_dimensions(size as u32, size as u32)
        .dark_color(svg::Color("#000000"))
        .light_color(svg::Color("#ffffff"))
        .build()
}

/// Optionally: Save QR code as PNG image (e.g., to `/tmp/address.png`)
pub fn save_qr_to_png(path: &str, data: &str, scale: u32) -> Result<(), Box<dyn std::error::Error>> {
    let code = QrCode::new(data.as_bytes())?;
    let image = code.render::<Luma<u8>>().max_dimensions(256 * scale, 256 * scale).build();
    image.save(path)?;
    Ok(())
}
