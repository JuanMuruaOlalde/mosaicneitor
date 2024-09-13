#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // eframe - hide console window on Windows in release
#[macro_use]
extern crate rust_i18n;
i18n!("locales", fallback = "en");

mod user_interface;
mod user_interface_app;
mod mosaic;
mod config;
mod utils;

fn main() -> eframe::Result<()> {
    user_interface::lauch_user_interface()
}
