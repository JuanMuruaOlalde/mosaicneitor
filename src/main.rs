#[macro_use]
extern crate rust_i18n;
i18n!("locales", fallback = "en");

mod config;
mod mosaic;
mod user_interface;
mod utils;

fn main() -> eframe::Result<()> {
    user_interface::lauch_user_interface()
}
