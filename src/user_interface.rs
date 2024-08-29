use eframe::egui::{self, TextureOptions};
use egui_file_dialog::FileDialog;

use crate::config;

pub fn lauch_user_interface() -> eframe::Result<()> {
    rust_i18n::set_locale(crate::config::WORKING_LOCALE);
    // where to call  egui_extras::install_image_loaders(ctx);  ???
    let options_for_eframe = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(crate::config::default_viewport_dimensions())
            .with_icon(egui::IconData::default()),
        ..eframe::NativeOptions::default()
    };
    eframe::run_native(
        MosaicneitorApp::name(),
        options_for_eframe,
        Box::new(|ctx| Ok(Box::new(MosaicneitorApp::new(ctx)))),
    )
}

struct MosaicneitorApp {
    file_dialog: FileDialog,
    selected_file: Option<std::path::PathBuf>,
    image: Option<egui::ColorImage>,
    dimensions_horizontal: String,
    dimensions_vertical: String,
    size_side_a: String,
    size_side_b: String,
}

impl MosaicneitorApp {
    fn new(_cc: &eframe::CreationContext) -> Self {
        Self {
            file_dialog: FileDialog::new()
                .show_new_folder_button(false)
                .default_pos([20.0, 30.0])
                .initial_directory(crate::config::default_working_folder())
                .add_file_filter(
                    "PNG",
                    std::sync::Arc::new(|path| path.extension().unwrap_or_default() == "png"),
                )
                .default_file_filter("PNG"),
            selected_file: None,
            image: None,
            dimensions_horizontal: config::DEFAULT_OVERAL_MOSAIC_DIMENSIONS_HORIZONTAL_MM
                .to_string(),
            dimensions_vertical: config::DEFAULT_OVERAL_MOSAIC_DIMENSIONS_VERTICAL_MM.to_string(),
            size_side_a: config::DEFAULT_BASE_TESSELA_SIZE_SIDE1_MM.to_string(),
            size_side_b: config::DEFAULT_BASE_TESSELA_SIZE_SIDE2_MM.to_string(),
        }
    }
    fn name() -> &'static str {
        "Mosaicneitor"
    }

    fn load_image_from_selected_file(&mut self) {
        match &self.selected_file {
            Some(path) => {
                let loaded_image = image::ImageReader::open(path);
                match loaded_image {
                    Ok(img) => {
                        let decoded_image = img.decode();
                        match decoded_image {
                            Ok(img) => {
                                let buffered_image = img.to_rgb8();
                                let pixels = buffered_image.as_flat_samples();
                                let color_image = egui::ColorImage::from_rgb(
                                    [img.width() as usize, img.height() as usize],
                                    pixels.as_slice(),
                                );
                                self.image = Some(color_image);
                            }
                            Err(_) => self.image = None,
                        }
                    }
                    Err(_) => self.image = None,
                }
            }
            None => self.image = None,
        }
    }
}

impl eframe::App for MosaicneitorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.5);

        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            if ui.button(t!("button_choose_image")).clicked() {
                self.file_dialog.select_file();
            }
            self.file_dialog.update(ctx);
            if let Some(path) = self.file_dialog.take_selected() {
                self.selected_file = Some(path.to_path_buf());
                self.load_image_from_selected_file();
            }
            match &self.selected_file {
                Some(x) => {
                    ui.label(format!(
                        "{} ({}x{})",
                        x.as_path().display(),
                        match &self.image {
                            Some(img) => img.size[0],
                            None => 0,
                        },
                        match &self.image {
                            Some(img) => img.size[1],
                            None => 0,
                        },
                    ));
                    ui.horizontal(|ui| {
                        ui.label(t!("mosaic_size"));
                        ui.label(t!("horizontal"));
                        ui.text_edit_singleline(&mut self.dimensions_horizontal);
                        ui.label(t!("vertical"));
                        ui.text_edit_singleline(&mut self.dimensions_vertical);
                    });
                    ui.horizontal(|ui| {
                        ui.label(t!("tessela_size"));
                        ui.label(t!("A_side"));
                        ui.text_edit_singleline(&mut self.size_side_a);
                        ui.label(t!("B_side"));
                        ui.text_edit_singleline(&mut self.size_side_b);
                    });
                }
                None => (),
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| match &self.image {
            Some(img) => {
                let handle = ctx.load_texture(
                    "image-to-display",
                    egui::ImageData::from(img.clone()),
                    TextureOptions::default(),
                );
                let sized_texture = egui::load::SizedTexture::new(
                    handle.id(),
                    egui::vec2(
                        match &self.dimensions_horizontal.parse::<f32>() {
                            Ok(x) => *x,
                            Err(_error) => 1.0,
                        },
                        match &self.dimensions_vertical.parse::<f32>() {
                            Ok(x) => *x,
                            Err(_error) => 1.0,
                        },
                    ),
                );
                ui.add(egui::Image::new(sized_texture));
            }
            None => (),
        });
    }
}
