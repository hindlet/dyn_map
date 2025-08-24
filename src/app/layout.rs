use std::fs;

use eframe::egui::{self, ComboBox, RichText};

use crate::{app::{pop_up_menus, DynamicMapApp}, data_structs::GameMap, db_helper};





pub fn draw_app(
    ctx: &egui::Context,
    app: &mut DynamicMapApp
) {

    egui::SidePanel::left("Map Panel").min_width(200.0).resizable(false).show(ctx, |ui| {

        ui.horizontal(|ui| {
            ui.label("Open File:");
            let selected = if app.selected_map.0 {&app.maps[app.selected_map.1].0.name} else {"None"};
            let change_check = app.selected_map.clone();
            ComboBox::from_id_salt("map_select")
                .selected_text(selected)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut app.selected_map, (false, 0), "None");
                    for (index, map) in app.maps.iter().enumerate() {
                        ui.horizontal(|ui| {
                            ui.selectable_value(&mut app.selected_map, (true, index), &map.0.name);
                            if ui.button("❌").on_hover_text("Delete Map").clicked() {
                                app.delete_map = (true, index, map.0.name.clone());
                            }
                        });
                    }
                });
            if app.selected_map != change_check && app.selected_map.0 { // map changed
                app.database = Some(db_helper::open_database(app.maps[app.selected_map.1].1.clone())); // open database
            }
            if ui.button("➕").on_hover_text("Create New Map").clicked() {
                app.new_map = (true, "New Map".to_string());
            }
        })
    });

    if app.selected_map.0 {
        egui::SidePanel::right("Player Panel").min_width(200.0).resizable(false).show(ctx, |ui| {
            ui.heading("Players");

        });
    }
    



    egui::CentralPanel::default().show(ctx, |ui| {

    });


    ////// pop up windows
    

    if app.new_map.0 {
        let mut result = None;
        pop_up_menus::new_map_menu(ctx, &mut result, &mut app.new_map.1);
        if let Some(create) = result {
            if create {
                let new_map_data = GameMap::new(app.new_map.1.clone()); // initialises database too
                app.maps.push(new_map_data);
            }
            app.new_map = (false, "".to_string());
            app.selected_map = (true, app.maps.len() - 1);
        }
    }

    if app.delete_map.0 {
        let mut result = None;
        pop_up_menus::delete_map_menu(ctx, &mut result, &app.delete_map.2);
        if let Some(delete) = result {
            if delete {
                let _ = fs::remove_dir_all(app.maps[app.delete_map.1].1.clone());
                app.maps.remove(app.delete_map.1);
            }
            app.delete_map = (false, 0, "".to_string());
        }
    }


}