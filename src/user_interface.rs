use eframe::egui;
use palette::convert::FromColor;

use crate::{
    config,
    user_interface_app::{MosaicneitorApp, Zoom},
    utils,
};

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
                        ui.label(format!("{} ->", t!("tessera_size")));
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
                        ui.label(format!("{}: ", t!("show")));
                        ui.checkbox(&mut self.show_image, t!("image"));
                        ui.checkbox(&mut self.show_tesserae_grid, t!("tesserae_grid"));
                        ui.checkbox(&mut self.show_actual_tesserae, t!("actual_tesserae"));
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
                match &self.image_for_displaying {
                    None => (),
                    Some(img) => {
                        let mosaic_dimensions = [
                            self.get_mosaic_dimensions()[0] * self.get_zoom_factor(),
                            self.get_mosaic_dimensions()[1] * self.get_zoom_factor(),
                        ];
                        let tessera_size = [
                            self.get_tessera_size()[0] * self.get_zoom_factor(),
                            self.get_tessera_size()[1] * self.get_zoom_factor(),
                        ];
                        let gap_between_tesserae =
                            config::GAP_BETWEEN_TESSSELAE * self.get_zoom_factor();
                        let display_size = egui::Vec2::new(
                            mosaic_dimensions[0] as f32,
                            mosaic_dimensions[1] as f32,
                        );
                        let start_position = egui::Pos2 {
                            x: ui.next_widget_position().x + 1.0,
                            y: ui.next_widget_position().y + 1.0,
                        };
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
                        if self.show_tesserae_grid {
                            let tesserae_grid = generate_shapes_to_paint_tesserae_grid(
                                start_position,
                                end_position,
                                tessera_size,
                                gap_between_tesserae,
                            );
                            painter.extend(tesserae_grid);
                        };
                        if self.show_actual_tesserae {
                            let actual_tesserae = generate_shapes_to_paint_mosaic(
                                self.get_base_mosaic_with_base_colors(),
                                start_position,
                                self.get_zoom_factor(),
                                gap_between_tesserae,
                            );
                            painter.extend(actual_tesserae);
                        }
                    }
                };
            });
        });
    }
}

fn generate_shapes_to_paint_tesserae_grid(
    start_position: egui::Pos2,
    end_position: egui::Pos2,
    tessera_size: [usize; 2],
    gap_between_tesserae: usize,
) -> Vec<egui::epaint::Shape> {
    let mut shapes = Vec::new();
    let stroke_width = 1.0;
    let stroke_color = egui::Color32::GREEN;
    for tessera_origin_x in ((start_position.x as usize)..(end_position.x as usize))
        .step_by(tessera_size[0] + gap_between_tesserae)
    {
        for tessera_origin_y in ((start_position.y as usize)..(end_position.y as usize))
            .step_by(tessera_size[1] + gap_between_tesserae)
        {
            let start_point = egui::Pos2 {
                x: (tessera_origin_x) as f32,
                y: (tessera_origin_y) as f32,
            };
            let end_point = egui::Pos2 {
                x: (tessera_origin_x + tessera_size[0]) as f32,
                y: (tessera_origin_y + tessera_size[1]) as f32,
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

fn generate_shapes_to_paint_mosaic(
    mosaic: crate::mosaic::Mosaic,
    start_position: egui::Pos2,
    zoom_factor: usize,
    gap_between_tesserae: usize,
) -> Vec<egui::Shape> {
    let mut shapes = Vec::new();
    let tessera_size = [
        mosaic.general_tessera_size.horizontal * zoom_factor,
        mosaic.general_tessera_size.vertical * zoom_factor,
    ];
    let mut y = start_position.y;
    for row in mosaic.contents {
        let mut x = start_position.x;
        for tessera in row {
            let rgbcolor_for_tessera: palette::Srgba<u8> =
                palette::Srgba::from_color(tessera.color).into();
            let egui_color_for_tessera = egui::Color32::from_rgb(
                rgbcolor_for_tessera.red,
                rgbcolor_for_tessera.green,
                rgbcolor_for_tessera.blue,
            );
            shapes.push(egui::epaint::Shape::Rect(egui::epaint::RectShape {
                rect: egui::Rect {
                    min: egui::pos2(x, y),
                    max: egui::pos2(x + tessera_size[0] as f32, y + tessera_size[1] as f32),
                },
                rounding: eframe::egui::Rounding::ZERO,
                fill: egui_color_for_tessera,
                stroke: egui::epaint::Stroke::new(1.0, egui_color_for_tessera),
                blur_width: 0.0,
                fill_texture_id: egui::TextureId::default(),
                uv: egui::Rect::ZERO,
            }));
            x = x + (tessera_size[0] + gap_between_tesserae) as f32;
        }
        y = y + (tessera_size[1] + gap_between_tesserae) as f32;
    }
    shapes
}
