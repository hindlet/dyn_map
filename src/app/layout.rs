use std::{fs, sync::{Arc, Mutex}};

use eframe::egui::{self, ComboBox, RichText};
use egui_extras::{Column, TableBuilder};

use crate::{app::{helper, pop_up_menus, DynamicMapApp}, data_structs::GameMap, db_helper};





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
                                app.delete_map = Some((map.0.name.clone(), index));
                            }
                        });
                    }
                });
            if app.selected_map != change_check && app.selected_map.0 { // map changed
                app.database = Some(Arc::new(Mutex::new(db_helper::open_database(app.maps[app.selected_map.1].1.clone())))); // open database
            }
            if ui.button("➕").on_hover_text("Create New Map").clicked() {
                app.new_map = Some("New Map".to_string());
            }
        })
    });

    if app.selected_map.0 {
        egui::SidePanel::right("Player Panel").min_width(300.0).resizable(false).show(ctx, |ui| {
            ui.heading("Players");
            
            TableBuilder::new(ui).id_salt("Player Table")
                .striped(true)
                .resizable(false)
                .columns(Column::auto().at_least(50.0), 4)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        if ui.button("➕").on_hover_text("Add Player").clicked() {
                            // app.new_map = (true, "New Map".to_string());
                        }
                    });
                    for col_header in ["Name", "Faction", "Colour"] {
                        header.col(|ui| {
                            ui.strong(col_header);
                        });
                    }
                })
                .body(|mut body| {
                    for player in db_helper::player_funcs::get_players_from_db(app.database.as_ref().unwrap().clone()).unwrap().iter() {
                        body.row(20.0, |mut row| {
                            row.col(|ui| {
                                ui.horizontal(|ui| {
                                    if ui.button("❌").on_hover_text("Remove Player").clicked() {
                                        // app.new_map = (true, "New Map".to_string());
                                    }
                                    if ui.button("✏").on_hover_text("Edit Player").clicked() {
                                        // app.new_map = (true, "New Map".to_string());
                                    }
                                });
                            });
                            row.col(|ui| {
                                ui.label(&player.name);
                            });
                            row.col(|ui| {
                                ui.label(&player.faction);
                            });
                            row.col(|ui| {
                                helper::colour_display_box(ui, player.colour);
                            });
                        });
                    }
                });
        });
    }
    



    egui::CentralPanel::default().show(ctx, |ui| {

    });


    ////// pop up windows
    
    if let Some(map_name) = app.new_map.as_mut() {
        let mut result = None;
        pop_up_menus::new_map_menu(ctx, &mut result, map_name);
        if let Some(create) = result {
            if create {
                let new_map_data = GameMap::new(map_name.clone()); // initialises database too
                app.maps.push(new_map_data);
            }
            app.new_map = None;
            app.selected_map = (true, app.maps.len() - 1);
        }
    }

    if let Some((map_name, index)) = &app.delete_map {
        let mut result = None;
        pop_up_menus::delete_map_menu(ctx, &mut result, &map_name);
        if let Some(delete) = result {
            if delete {
                let _ = fs::remove_dir_all(app.maps[*index].1.clone());
                app.maps.remove(*index);
            }
            app.delete_map = None;
        }
    }


}