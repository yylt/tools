#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{self,egui};
use eframe::egui::viewport::*;
use tools::Nosleep;
use std::path::PathBuf;
use log::LevelFilter;
use std::fs;

fn set_font(cc: &eframe::CreationContext<'_>) {
    let mut fonts = egui::FontDefinitions::default();
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push("data");
    root.push("hanyi.ttf");

    let data: Vec<u8> = fs::read(root).unwrap();
    fonts.font_data.insert(
        "hanyi".to_owned(),
        egui::FontData::from_owned(data),
    ); 
    fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap()
        .insert(0, "hanyi".to_owned());
    fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap()
        .push("hanyi".to_owned());


    cc.egui_ctx.set_fonts(fonts);
}

fn main() -> eframe::Result {
    env_logger::Builder::new().filter_level(LevelFilter::Info).init(); 

    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push("data");
    root.push("favicon.ico");
    let img = image::open(root).unwrap();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([450.0, 100.0]) // wide enough for the drag-drop overlay text
            .with_icon(IconData {  width: img.width(), height: img.height(),rgba: img.into_bytes()})
            .with_title("tools")
            .with_drag_and_drop(true),
        ..Default::default()
    };
    eframe::run_native(
        "Native file dialogs and drag-and-drop files",
        options,
        Box::new(|cc| {
            set_font(cc);
            Ok(Box::new(Nosleep::new(cc)))
        }),
    )
}