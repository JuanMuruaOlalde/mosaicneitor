use dirs::home_dir;
use eframe::emath::Vec2;
use std::path::PathBuf;

pub const WORKING_LOCALE: &str = "en";

pub fn default_viewport_dimensions() -> Vec2 {
    Vec2::new(1024.0, 800.0)
}

pub fn default_working_folder() -> PathBuf {
    match home_dir() {
        Some(x) => x,
        None => std::path::PathBuf::new(),
    }
    // A list of available folders is in: https://docs.rs/dirs/latest/dirs/#functions
}

pub const DEFAULT_OVERAL_MOSAIC_DIMENSIONS_HORIZONTAL_MM: i32 = 500;
pub const DEFAULT_OVERAL_MOSAIC_DIMENSIONS_VERTICAL_MM: i32 = 300;
pub const DEFAULT_BASE_TESSELA_SIZE_SIDE1_MM: i32 = 10;
pub const DEFAULT_BASE_TESSELA_SIZE_SIDE2_MM: i32 = 10;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn manualy_check_default_working_folder() {
        match default_working_folder().to_str() {
            Some(x) => assert_eq!(x, "C:\\Users\\jmurua"),
            None => panic!("Config cannot stablish the default working folder"),
        }
    }
}
