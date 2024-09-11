use eframe::egui;
use egui_file_dialog::FileDialog;

use crate::{config, utils};

pub fn lauch_user_interface() -> eframe::Result<()> {
    rust_i18n::set_locale(crate::config::WORKING_LOCALE);
    let options_for_eframe = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(crate::config::default_viewport_dimensions())
            .with_icon(egui::IconData::default()),
        ..eframe::NativeOptions::default()
    };
    let title = format!("{} {}", MosaicneitorApp::name(), utils::get_version_text());
    eframe::run_native(
        &title,
        options_for_eframe,
        Box::new(|ctx| {
            egui_extras::install_image_loaders(&ctx.egui_ctx);
            Ok(Box::new(MosaicneitorApp::new(ctx)))
        }),
    )
}

impl eframe::App for MosaicneitorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.5);

        egui::TopBottomPanel::top("controls").show(ctx, |ui| {
            if ui.button(t!("btn_choose_image")).clicked() {
                self.file_dialog.select_file();
            }
            self.file_dialog.update(ctx);
            if let Some(path) = self.file_dialog.take_selected() {
                self.selected_file = Some(path.to_path_buf());
                self.load_image_from_selected_file();
            }
            match &self.selected_file {
                None => (),
                Some(file) => {
                    let image_dimensions = self.get_image_dimensions();
                    ui.label(format!(
                        "{} ({}x{})(px)",
                        file.as_path().display(),
                        image_dimensions[0],
                        image_dimensions[1],
                    ));
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label(format!("{} ->", t!("mosaic_size")));
                        ui.label(format!("{} (mm):", t!("horizontal")));
                        ui.add(
                            egui::TextEdit::singleline(&mut self.dimensions_horizontal)
                                .desired_width(75.0),
                        );
                        ui.label(format!("{} (mm):", t!("vertical")));
                        ui.add(
                            egui::TextEdit::singleline(&mut self.dimensions_vertical)
                                .desired_width(75.0),
                        );
                        if ui.button(t!("btn_adjust_mosaic_to_image")).clicked() {
                            self.adjust_mosaic_dimensions_to_image_aspect_ratio();
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label(format!("{} ->", t!("tessela_size")));
                        ui.label(format!("{} (mm):", t!("A_side")));
                        ui.add(
                            egui::TextEdit::singleline(&mut self.size_side_a).desired_width(75.0),
                        );
                        ui.label(format!("{} (mm):", t!("B_side")));
                        ui.add(
                            egui::TextEdit::singleline(&mut self.size_side_b).desired_width(75.0),
                        );
                    });
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label(format!("{}: ", t!("Show")));
                        ui.checkbox(&mut self.show_image, t!("Image"));
                        ui.checkbox(&mut self.show_tesselae, t!("Tesselae"));
                        ui.add_space(45.0);
                        ui.label("Zoom: ");
                        ui.selectable_value(&mut self.zoom_level, Zoom::X1, "x1");
                        ui.selectable_value(&mut self.zoom_level, Zoom::X2, "x2");
                        ui.selectable_value(&mut self.zoom_level, Zoom::X3, "x3");
                        ui.selectable_value(&mut self.zoom_level, Zoom::X4, "x4");
                        ui.selectable_value(&mut self.zoom_level, Zoom::X5, "x5");
                    });
                }
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                match &self.image {
                    None => (),
                    Some(img) => {
                        let mosaic_dimensions = self.get_mosaic_dimensions();
                        let display_size = egui::Vec2::new(
                            mosaic_dimensions[0] as f32,
                            mosaic_dimensions[1] as f32,
                        );
                        let start_position = ui.next_widget_position();
                        let end_position = egui::Pos2 {
                            x: start_position.x + display_size.x,
                            y: start_position.y + display_size.y,
                        };
                        let (_response, painter) =
                            ui.allocate_painter(display_size, egui::Sense::hover());
                        if self.show_image {
                            let handle = ctx.load_texture(
                                "image-to-display",
                                egui::ImageData::from(img.clone()),
                                egui::TextureOptions::default(),
                            );
                            painter.image(
                                handle.id(),
                                egui::Rect::from_min_max(start_position, end_position),
                                egui::Rect::from_min_max(
                                    egui::pos2(0.0, 0.0),
                                    egui::pos2(1.0, 1.0),
                                ),
                                egui::Color32::WHITE,
                            );
                        };
                        if self.show_tesselae {
                            let tessela_size = self.get_tessela_size();
                            let gap_between_tesselae = 1 * self.get_zoom_factor();
                            let tesselae_grid = generate_shapes_to_paint_tesselae_grid(
                                start_position,
                                end_position,
                                tessela_size,
                                gap_between_tesselae,
                            );
                            painter.extend(tesselae_grid);
                        };
                    }
                };
            });
        });
    }
}

fn generate_shapes_to_paint_tesselae_grid(
    start_position: egui::Pos2,
    end_position: egui::Pos2,
    tessela_size: [usize; 2],
    gap_between_tesselae: usize,
) -> Vec<egui::epaint::Shape> {
    let mut shapes = Vec::new();
    let stroke_width = 1.0;
    let stroke_color = egui::Color32::GREEN;
    for tessela_origin_x in ((start_position.x as usize)..(end_position.x as usize))
        .step_by(tessela_size[0] + gap_between_tesselae * 2)
    {
        for tessela_origin_y in ((start_position.y as usize)..(end_position.y as usize))
            .step_by(tessela_size[1] + gap_between_tesselae * 2)
        {
            let start_point = egui::Pos2 {
                x: (tessela_origin_x + gap_between_tesselae) as f32,
                y: (tessela_origin_y + gap_between_tesselae) as f32,
            };
            let end_point = egui::Pos2 {
                x: (tessela_origin_x + gap_between_tesselae + tessela_size[0]) as f32,
                y: (tessela_origin_y + gap_between_tesselae + tessela_size[1]) as f32,
            };
            shapes.push(egui::epaint::Shape::Rect(egui::epaint::RectShape {
                rect: egui::Rect {
                    min: start_point,
                    max: end_point,
                },
                rounding: eframe::egui::Rounding::ZERO,
                fill: egui::Color32::TRANSPARENT,
                stroke: egui::epaint::Stroke::new(stroke_width, stroke_color),
                blur_width: 0.0,
                fill_texture_id: egui::TextureId::default(),
                uv: egui::Rect::ZERO,
            }));
        }
    }
    shapes
}

struct MosaicneitorApp {
    file_dialog: FileDialog,
    selected_file: Option<std::path::PathBuf>,
    image: Option<egui::ColorImage>,
    dimensions_horizontal: String,
    dimensions_vertical: String,
    size_side_a: String,
    size_side_b: String,
    zoom_level: Zoom,
    show_image: bool,
    show_tesselae: bool,
}

#[derive(Debug, PartialEq)]
enum Zoom {
    X1,
    X2,
    X3,
    X4,
    X5,
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
            image: None,
            dimensions_horizontal: config::DEFAULT_OVERAL_MOSAIC_DIMENSIONS_HORIZONTAL_MM
                .to_string(),
            dimensions_vertical: config::DEFAULT_OVERAL_MOSAIC_DIMENSIONS_VERTICAL_MM.to_string(),
            size_side_a: config::DEFAULT_BASE_TESSELA_SIZE_HORIZONTAL_MM.to_string(),
            size_side_b: config::DEFAULT_BASE_TESSELA_SIZE_VERTICAL_MM.to_string(),
            zoom_level: Zoom::X1,
            show_image: true,
            show_tesselae: true,
        }
    }
}

impl MosaicneitorApp {
    fn name() -> &'static str {
        "Mosaicneitor"
    }

    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        Default::default()
    }

    fn get_zoom_factor(&self) -> usize {
        match self.zoom_level {
            Zoom::X1 => 1,
            Zoom::X2 => 2,
            Zoom::X3 => 3,
            Zoom::X4 => 4,
            Zoom::X5 => 5,
        }
    }

    fn load_image_from_selected_file(&mut self) {
        match &self.selected_file {
            None => self.image = None,
            Some(path) => {
                let loaded_image = image::ImageReader::open(path);
                match loaded_image {
                    Err(_) => self.image = None,
                    Ok(img) => {
                        let decoded_image = img.decode();
                        match decoded_image {
                            Err(_) => self.image = None,
                            Ok(img) => {
                                let buffered_image = img.to_rgb8();
                                let pixels = buffered_image.as_flat_samples();
                                let color_image = egui::ColorImage::from_rgb(
                                    [img.width() as usize, img.height() as usize],
                                    pixels.as_slice(),
                                );
                                self.image = Some(color_image);
                                self.adjust_mosaic_dimensions_to_image_aspect_ratio();
                            }
                        }
                    }
                }
            }
        }
    }

    fn get_image_dimensions(&self) -> [usize; 2] {
        match &self.image {
            Some(img) => img.size,
            None => [1, 1],
        }
    }

    fn get_mosaic_dimensions(&self) -> [usize; 2] {
        [
            match self.dimensions_horizontal.parse::<usize>() {
                Ok(dimension) => dimension * self.get_zoom_factor(),
                Err(_error) => 1,
            },
            match self.dimensions_vertical.parse::<usize>() {
                Ok(dimension) => dimension * self.get_zoom_factor(),
                Err(_error) => 1,
            },
        ]
    }

    fn adjust_mosaic_dimensions_to_image_aspect_ratio(&mut self) {
        let adjusted_dimensions = utils::round_preserving_aspect_ratio(
            self.get_mosaic_dimensions(),
            self.get_image_dimensions(),
        );
        self.dimensions_horizontal = adjusted_dimensions[0].to_string();
        self.dimensions_vertical = adjusted_dimensions[1].to_string();
    }

    fn get_tessela_size(&self) -> [usize; 2] {
        [
            match self.size_side_a.parse::<usize>() {
                Ok(size) => size * self.get_zoom_factor(),
                Err(_error) => 1,
            },
            match self.size_side_b.parse::<usize>() {
                Ok(size) => size * self.get_zoom_factor(),
                Err(_error) => 1,
            },
        ]
    }
}
