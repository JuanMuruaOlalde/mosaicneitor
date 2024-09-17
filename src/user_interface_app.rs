use eframe::egui;
use egui_file_dialog::FileDialog;
use palette::convert::FromColor;

use crate::{config, mosaic, utils};

pub(crate) struct MosaicneitorApp {
    pub(crate) file_dialog: FileDialog,
    pub(crate) selected_file: Option<std::path::PathBuf>,
    base_image: image::Rgba32FImage,
    pub(crate) image_for_displaying: Option<egui::ColorImage>,
    pub(crate) mosaic: mosaic::Mosaic,
    pub(crate) dimensions_horizontal: String,
    pub(crate) dimensions_vertical: String,
    pub(crate) size_side_a: String,
    pub(crate) size_side_b: String,
    pub(crate) zoom_level: Zoom,
    pub(crate) show_image: bool,
    pub(crate) show_tesserae_grid: bool,
    pub(crate) show_actual_tesserae: bool,
}

impl Default for MosaicneitorApp {
    fn default() -> Self {
        Self {
            file_dialog: FileDialog::new()
                .show_new_folder_button(false)
                .default_pos([20.0, 30.0])
                .initial_directory(crate::config::default_working_folder())
                .add_file_filter(
                    "PNG",
                    std::sync::Arc::new(|path| path.extension().unwrap_or_default() == "png"),
                )
                .add_file_filter(
                    "JPEG",
                    std::sync::Arc::new(|path| path.extension().unwrap_or_default() == "jpg"),
                )
                .default_file_filter("JPEG"),
            selected_file: None,
            base_image: image::Rgba32FImage::default(),
            image_for_displaying: None,
            mosaic: mosaic::Mosaic::new(
                image::Rgba32FImage::default(),
                mosaic::RectangleInMm {
                    horizontal: config::DEFAULT_BASE_TESSERA_SIZE_HORIZONTAL_MM,
                    vertical: config::DEFAULT_BASE_TESSERA_SIZE_VERTICAL_MM,
                },
            ),
            dimensions_horizontal: config::DEFAULT_OVERAL_MOSAIC_DIMENSIONS_HORIZONTAL_MM
                .to_string(),
            dimensions_vertical: config::DEFAULT_OVERAL_MOSAIC_DIMENSIONS_VERTICAL_MM.to_string(),
            size_side_a: config::DEFAULT_BASE_TESSERA_SIZE_HORIZONTAL_MM.to_string(),
            size_side_b: config::DEFAULT_BASE_TESSERA_SIZE_VERTICAL_MM.to_string(),
            zoom_level: Zoom::X1,
            show_image: false,
            show_tesserae_grid: true,
            show_actual_tesserae: false,
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum Zoom {
    X1,
    X2,
    X3,
    X4,
    X5,
}

impl MosaicneitorApp {
    pub fn name() -> &'static str {
        "Mosaicneitor"
    }

    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        Default::default()
    }

    pub(crate) fn load_image_from_selected_file(&mut self) {
        match &self.selected_file {
            None => self.image_for_displaying = None,
            Some(path) => {
                let loaded_image = image::ImageReader::open(path);
                match loaded_image {
                    Err(_) => self.image_for_displaying = None,
                    Ok(img) => {
                        let decoded_image = img.decode();
                        match decoded_image {
                            Err(_) => self.image_for_displaying = None,
                            Ok(img) => {
                                self.base_image = img.to_rgba32f();
                                let buffered_image = img.to_rgb8();
                                let pixels = buffered_image.as_flat_samples();
                                let egui_color_image = egui::ColorImage::from_rgb(
                                    [img.width() as usize, img.height() as usize],
                                    pixels.as_slice(),
                                );
                                self.image_for_displaying = Some(egui_color_image);
                                self.adjust_mosaic_dimensions_to_image_aspect_ratio();
                                self.show_image = true;
                                self.show_tesserae_grid = true;
                            }
                        }
                    }
                }
            }
        }
    }

    pub(crate) fn adjust_mosaic_dimensions_to_image_aspect_ratio(&mut self) {
        let adjusted_dimensions = utils::round_preserving_aspect_ratio(
            self.get_mosaic_dimensions(),
            self.get_image_dimensions(),
        );
        self.dimensions_horizontal = adjusted_dimensions[0].to_string();
        self.dimensions_vertical = adjusted_dimensions[1].to_string();
    }

    pub fn get_image_dimensions(&self) -> [usize; 2] {
        match &self.image_for_displaying {
            Some(img) => img.size,
            None => [1, 1],
        }
    }

    pub fn get_mosaic_dimensions(&self) -> [usize; 2] {
        [
            match self.dimensions_horizontal.parse::<usize>() {
                Ok(dimension) => dimension,
                Err(_error) => 1,
            },
            match self.dimensions_vertical.parse::<usize>() {
                Ok(dimension) => dimension,
                Err(_error) => 1,
            },
        ]
    }

    pub fn get_tessera_size(&self) -> [usize; 2] {
        [
            match self.size_side_a.parse::<usize>() {
                Ok(size) => size,
                Err(_error) => 1,
            },
            match self.size_side_b.parse::<usize>() {
                Ok(size) => size,
                Err(_error) => 1,
            },
        ]
    }

    pub fn get_zoom_factor(&self) -> usize {
        match self.zoom_level {
            Zoom::X1 => 1,
            Zoom::X2 => 2,
            Zoom::X3 => 3,
            Zoom::X4 => 4,
            Zoom::X5 => 5,
        }
    }

    pub fn get_a_blank_mosaic(&self) -> mosaic::Mosaic {
        let general_tessera_size = mosaic::RectangleInMm {
            horizontal: self.get_tessera_size()[0],
            vertical: self.get_tessera_size()[1],
        };
        let mosaic_size = mosaic::RectangleInMm {
            horizontal: self.get_mosaic_dimensions()[0],
            vertical: self.get_mosaic_dimensions()[1],
        };
        let color_srgba: palette::Srgba<f32> =
            palette::Srgba::from(egui::Color32::LIGHT_GRAY.to_srgba_unmultiplied()).into();
        let color_oklch = palette::Oklch::from_color(color_srgba);
        let mut mosaic = mosaic::Mosaic::new(self.base_image.clone(), general_tessera_size);
        for _vertical_position in (1..mosaic_size.vertical)
            .step_by(general_tessera_size.vertical + config::GAP_BETWEEN_TESSSELAE)
        {
            let mut row: Vec<mosaic::Tessera> = Vec::new();
            for _horizontal_position in (1..mosaic_size.horizontal)
                .step_by(general_tessera_size.horizontal + config::GAP_BETWEEN_TESSSELAE)
            {
                row.push(mosaic::Tessera { color: color_oklch });
            }
            mosaic.add_a_row_of_tesserae(row);
        }
        mosaic
    }

    pub fn get_mosaic_from_base_image(&self) -> mosaic::Mosaic {
        let general_tessera_size = mosaic::RectangleInMm {
            horizontal: self.get_tessera_size()[0],
            vertical: self.get_tessera_size()[1],
        };
        let mosaic_size = mosaic::RectangleInMm {
            horizontal: self.get_mosaic_dimensions()[0],
            vertical: self.get_mosaic_dimensions()[1],
        };
        let mut mosaic = mosaic::Mosaic::new(self.base_image.clone(), general_tessera_size);
        for vertical_position in (1..mosaic_size.vertical)
            .step_by(general_tessera_size.vertical + config::GAP_BETWEEN_TESSSELAE)
        {
            let mut row: Vec<mosaic::Tessera> = Vec::new();
            for horizontal_position in (1..mosaic_size.horizontal)
                .step_by(general_tessera_size.horizontal + config::GAP_BETWEEN_TESSSELAE)
            {
                let sample_point = [
                    horizontal_position + general_tessera_size.horizontal / 2,
                    vertical_position + general_tessera_size.horizontal / 2,
                ];
                let pixel_on_the_image = self.get_pixel_position_on_the_image(sample_point);
                let color_srgba: palette::Srgba<f32> = match self
                    .get_pixel_color(pixel_on_the_image)
                {
                    Ok(color) => palette::Srgba::from(color.to_srgba_unmultiplied()).into(),
                    Err(_) => {
                        palette::Srgba::from(egui::Color32::YELLOW.to_srgba_unmultiplied()).into()
                    }
                };
                let color_oklch = palette::Oklch::from_color(color_srgba);
                row.push(mosaic::Tessera { color: color_oklch });
            }
            mosaic.add_a_row_of_tesserae(row);
        }
        mosaic
    }

    fn get_pixel_position_on_the_image(&self, point_on_the_mosaic: [usize; 2]) -> [usize; 2] {
        let mosaic_dimensions = self.get_mosaic_dimensions();
        let image_dimensions = self.get_image_dimensions();
        [
            image_dimensions[0] * point_on_the_mosaic[0] / mosaic_dimensions[0],
            image_dimensions[1] * point_on_the_mosaic[1] / mosaic_dimensions[1],
        ]
    }

    fn get_pixel_color(&self, pixel_on_the_image: [usize; 2]) -> Result<egui::Color32, String> {
        match &self.image_for_displaying {
            Some(img) => {
                let avance_rows_for_vertical_pixel =
                    pixel_on_the_image[1] * self.get_image_dimensions()[0];
                let avance_for_horizontal_pixel = pixel_on_the_image[0];
                match img
                    .pixels
                    .get(avance_rows_for_vertical_pixel + avance_for_horizontal_pixel)
                {
                    Some(color) => Ok(*color),
                    None => Err(String::from("no pixel")),
                }
            }
            None => Err(String::from("no image")),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_mosaic_dimensions_yields_the_correct_values_or_defaults() {
        let mut app = MosaicneitorApp::default();

        app.dimensions_horizontal = String::from("500");
        app.dimensions_vertical = String::from("300");
        assert_eq!(app.get_mosaic_dimensions(), [500, 300]);

        app.dimensions_horizontal = String::from("500.8");
        app.dimensions_vertical = String::from("300.8");
        assert_eq!(app.get_mosaic_dimensions(), [1, 1]);

        app.dimensions_horizontal = String::from("werqwe");
        app.dimensions_vertical = String::from("asdf");
        assert_eq!(app.get_mosaic_dimensions(), [1, 1]);
    }
    #[test]
    fn get_tessera_size_yields_the_correct_values_or_defaults() {
        let mut app = MosaicneitorApp::default();

        app.size_side_a = String::from("10");
        app.size_side_b = String::from("20");
        assert_eq!(app.get_tessera_size(), [10, 20]);

        app.size_side_a = String::from("10.8");
        app.size_side_b = String::from("20.8");
        assert_eq!(app.get_tessera_size(), [1, 1]);

        app.size_side_a = String::from("eqwer");
        app.size_side_b = String::from("asdf");
        assert_eq!(app.get_tessera_size(), [1, 1]);
    }

    #[test]
    fn translation_from_mosaic_position_to_pixel_position_yieds_correct_positions() {
        let mut app = MosaicneitorApp::default();
        app.image_for_displaying = Some(egui::ColorImage::example());

        let image_dimensions = app.get_image_dimensions();
        app.dimensions_horizontal = image_dimensions[0].to_string();
        app.dimensions_vertical = image_dimensions[1].to_string();
        let point_on_the_mosaic = [5, 5];
        assert_eq!(
            app.get_pixel_position_on_the_image(point_on_the_mosaic),
            [5, 5]
        );

        app.dimensions_horizontal = String::from("500");
        app.dimensions_vertical = String::from("300");
        let point_on_the_mosaic = [10, 10];
        assert_eq!(
            app.get_pixel_position_on_the_image(point_on_the_mosaic),
            [2, 2]
        );

        app.dimensions_horizontal = String::from("400");
        app.dimensions_vertical = String::from("400");
        let point_on_the_mosaic = [10, 10];
        assert_eq!(
            app.get_pixel_position_on_the_image(point_on_the_mosaic),
            [3, 1]
        );
    }

    #[test]
    fn get_pixel_color_yields_error_if_image_is_none() {
        let app = MosaicneitorApp::default();
        assert_eq!(
            app.get_pixel_color([234, 567]),
            Err(String::from("no image"))
        );
    }

    #[test]
    fn get_pixel_color_yields_error_if_you_ask_for_pixels_outside_image_boundaries() {
        let mut app = MosaicneitorApp::default();
        app.image_for_displaying = Some(egui::ColorImage::example());
        let pixel_position = [
            egui::ColorImage::example().width() + 5,
            egui::ColorImage::example().height() + 5,
        ];
        assert_eq!(
            app.get_pixel_color(pixel_position),
            Err(String::from("no pixel"))
        );
    }

    #[test]
    fn get_mosaic_from_base_image_yields_correct_mosaic_dimensions_for_happy_path_case() {
        let mut app = MosaicneitorApp::default();
        app.dimensions_horizontal = String::from("500");
        app.dimensions_vertical = String::from("300");
        app.size_side_a = String::from("10");
        app.size_side_b = String::from("10");
        app.image_for_displaying = Some(egui::ColorImage::example());
        let mosaic = app.get_mosaic_from_base_image();
        assert_eq!(
            mosaic.get_number_of_rows(),
            300 / (10 + config::GAP_BETWEEN_TESSSELAE) + 1
        );
        assert_eq!(
            mosaic.get_number_of_tesserae_in_row(1),
            500 / (10 + config::GAP_BETWEEN_TESSSELAE) + 1
        );
    }
}
