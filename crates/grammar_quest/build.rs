use image::imageops::FilterType;
use std::{env, fs, path::Path};

fn icon_bytes(image: &image::RgbaImage, size: u32) -> Vec<u8> {
    image::imageops::resize(image, size, size, FilterType::Nearest).into_raw()
}

fn rust_array(name: &str, bytes: &[u8]) -> String {
    let values = bytes
        .iter()
        .map(u8::to_string)
        .collect::<Vec<_>>()
        .join(",");
    format!("pub const {name}: [u8; {}] = [{values}];\n", bytes.len())
}

fn main() {
    let logo_path = Path::new("assets/brand/grammar-quest-logo.png");
    println!("cargo:rerun-if-changed={}", logo_path.display());

    let logo = image::open(logo_path)
        .expect("Grammar Quest logo must be a readable image")
        .to_rgba8();
    let generated = [
        ("WINDOW_ICON_SMALL", icon_bytes(&logo, 16)),
        ("WINDOW_ICON_MEDIUM", icon_bytes(&logo, 32)),
        ("WINDOW_ICON_BIG", icon_bytes(&logo, 64)),
    ]
    .into_iter()
    .map(|(name, bytes)| rust_array(name, &bytes))
    .collect::<String>();

    let output =
        Path::new(&env::var("OUT_DIR").expect("Cargo must set OUT_DIR")).join("window_icon.rs");
    fs::write(output, generated).expect("Generated window icon must be writable");
}
