use include_assets::{include_dir, NamedArchive};
use anyhow::Error;
use eframe::egui::{self, IconData};

use crate::app::DynamicMapApp;

mod data_structs;
mod db_helper;
mod app;
mod export;

const APP_INFO: app_dirs::AppInfo = app_dirs::AppInfo{name: "ViTenGriiDynMap", author: "hindlet"};


fn main() -> Result<(), Error> {
    env_logger::init();

    // let db = Arc::new(Mutex::new(db_helper::init_database()));


    // println!("{:?}", db_helper::tile_funcs::get_tile_from_db(db.clone(), 1));
    // println!("{:?}", db_helper::tile_funcs::get_tile_from_db(db.clone(), 2));

    // println!("{:?}", db_helper::control_funcs::get_highest_tile_control(db.clone(), 1));
    // println!("{:?}", db_helper::control_funcs::get_highest_tile_control(db.clone(), 2));

    // println!("{:?}", db_helper::control_funcs::get_player_controlled_tiles(db.clone(), 1));
    // println!("{:?}", db_helper::control_funcs::get_player_controlled_tiles(db.clone(), 2));
    // println!("{:?}", db_helper::control_funcs::get_max_control_levels(db.clone()));

    let archive = NamedArchive::load(include_dir!("assets"));
    let icon_data = archive.get("Logo_128.png").unwrap();
    let icon = image::load_from_memory(icon_data).unwrap().to_rgba8();
    let (icon_width, icon_height) = icon.dimensions();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1350.0, 900.0]).with_icon(IconData {
            rgba: icon.to_vec(),
            width: icon_width,
            height: icon_height
        }),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "Vi'Ten'Grii Dynamic Map", 
        options,
        Box::new(|_cc| {
            let mut app = DynamicMapApp::default();
            app.init()?;
            Ok(Box::new(app))
        })
    );

    Ok(())
}
