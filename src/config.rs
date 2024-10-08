use eframe::egui;

pub const WORKING_LOCALE: &str = "en";

pub fn default_viewport_dimensions() -> eframe::emath::Vec2 {
    eframe::emath::Vec2::new(1024.0, 800.0)
}

pub fn default_working_folder() -> std::path::PathBuf {
    match dirs::home_dir() {
        Some(x) => x,
        None => std::path::PathBuf::new(),
    }
    // A list of available folders is in: https://docs.rs/dirs/latest/dirs/#functions
}

pub const DEFAULT_OVERAL_MOSAIC_DIMENSIONS_HORIZONTAL_MM: usize = 500;
pub const DEFAULT_OVERAL_MOSAIC_DIMENSIONS_VERTICAL_MM: usize = 300;
pub const DEFAULT_BASE_TESSERA_SIZE_HORIZONTAL_MM: usize = 10;
pub const DEFAULT_BASE_TESSERA_SIZE_VERTICAL_MM: usize = 10;
pub const DEFAULT_GAP_BETWEEN_TESSSELAE: usize = 1;

pub const COLOR_FOR_GRID: egui::Color32 = egui::Color32::LIGHT_RED;

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
