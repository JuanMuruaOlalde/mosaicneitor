use eframe::egui;
use egui_file_dialog::FileDialog;

pub fn lauch_user_interface() -> eframe::Result<()> {
    rust_i18n::set_locale(crate::config::WORKING_LOCALE);
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
        }
    }
    fn name() -> &'static str {
        "Mosaicneitor"
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
            }
            match &self.selected_file {
                Some(x) => {
                    ui.label(format!("{:?}", x.as_path()));
                    ui.label(t!("dummy_text_mosaic_size"));
                    ui.label(t!("dummy_text_tessela_size"))
                }
                None => ui.label(""),
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| match &self.selected_file {
            Some(x) => ui.label(t!("dummy_text_here_goes_the_painting")),
            None => ui.label(""),
        });
    }
}
