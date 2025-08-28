use std::{fs, sync::{Arc, Mutex}};

use eframe::egui::{self, pos2, response, Color32, ComboBox, RichText};
use egui_extras::{Column, TableBuilder};

use crate::{app::{helper::{self, draw_tile}, map_render, pop_up_menus, DynamicMapApp}, data_structs::{GameMap, Player, Tile, TilePos, TileType}, db_helper};





pub fn draw_app(
    ctx: &egui::Context,
    app: &mut DynamicMapApp
) {

    egui::SidePanel::left("Map Panel").min_width(200.0).resizable(false).show(ctx, |ui| {

        ui.horizontal(|ui| {
            ui.label("Open File:");
            let selected = if app.selected_map.is_some() {&app.maps[app.selected_map.unwrap()].0.name} else {"None"};
            let change_check = app.selected_map.clone();
            ComboBox::from_id_salt("map_select")
                .selected_text(selected)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut app.selected_map, None, "None");
                    for (index, map) in app.maps.iter().enumerate() {
                        ui.horizontal(|ui| {
                            ui.selectable_value(&mut app.selected_map, Some(index), &map.0.name);
                            if ui.button("❌").on_hover_text("Delete Map").clicked() {
                                app.delete_map = Some((map.0.name.clone(), index));
                            }
                        });
                    }
                });
            if app.selected_map != change_check && app.selected_map.is_some() { // map changed
                app.database = Some(Arc::new(Mutex::new(db_helper::open_database(app.maps[app.selected_map.unwrap()].1.clone())))); // open database
                app.logged_in_player = (0, false);
                app.log_in_details = (true, "".to_string(), "".to_string(), false);
            }
            if ui.button("➕").on_hover_text("Create New Map").clicked() {
                app.new_map = Some(("New Map".to_string(), "Your Name".to_string(), "Your Password".to_string()));
            }
        });

        ui.horizontal(|ui| {
            ui.label("Edit Map");
            ui.checkbox(&mut app.edit_map_mode, "");
        });
    });

    if let Some(_map_index) = app.selected_map {
        egui::SidePanel::right("Player Panel").min_width(300.0).resizable(false).show(ctx, |ui| {
            if app.logged_in_player.0 == 0 {
                return;
            }
            ui.heading("Players");
            
            TableBuilder::new(ui).id_salt("Player Table")
                .striped(true)
                .resizable(false)
                .columns(Column::auto().at_least(50.0), 4)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        if ui.button("➕").on_hover_text("Add Player").clicked() {
                            app.add_player = Some(Player {
                                id: db_helper::player_funcs::get_next_player_id(app.database.as_ref().unwrap().clone()).unwrap(),
                                name: "New Player".to_string(),
                                faction: "Faction".to_string(),
                                colour: Color32::WHITE,
                                admin: false,
                                password: "Password".to_string()
                            });
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
                                    if app.logged_in_player.1 {
                                        if ui.button("❌").on_hover_text("Remove Player").clicked() {
                                            app.delete_player = Some((player.name.clone(), player.id));
                                        }
                                    }
                                    if app.logged_in_player.1 || player.id == app.logged_in_player.0 {
                                        if ui.button("✏").on_hover_text("Edit Player").clicked() {
                                            app.edit_player = Some(player.clone());
                                        }
                                    }
                                });
                            });
                            row.col(|ui| {
                                ui.horizontal(|ui| {
                                    if player.admin {
                                        ui.label(RichText::new("👑").color(Color32::YELLOW));
                                    }
                                    ui.label(&player.name);
                                });
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

        if app.database.is_none() || app.logged_in_player.0 == 0 {
            return;
        }
        map_render::render_map(app, ui);
    });

    ////// pop up windows
    
    if let Some(new_map_info) = app.new_map.as_mut() {
        let mut result = None;
        pop_up_menus::new_map_menu(ctx, &mut result, new_map_info);
        if let Some(create) = result {
            if create {
                let new_map_data = GameMap::new(new_map_info.0.clone()); // initialises database too
                app.database = Some(Arc::new(Mutex::new(db_helper::open_database(new_map_data.1.clone())))); // open database
                app.maps.push(new_map_data);
                app.selected_map = Some(app.maps.len() - 1);
                app.edit_map_mode = true;
                let res = db_helper::player_funcs::insert_player_to_db(app.database.as_ref().unwrap().clone(), Player {
                    id: 1,
                    name: new_map_info.1.clone(),
                    faction: "N/A".to_string(),
                    colour: Color32::WHITE,
                    admin: true,
                    password: new_map_info.2.clone()
                });
                app.logged_in_player = (1, true);
            }
            app.new_map = None;
            // let _ = db_helper::tile_funcs::add_creation_space_to_db(app.database.as_ref().unwrap().clone(), TilePos{x: 0, y: 0, top_row: true});
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
            app.selected_map = None;
        }
    }

    if let Some(new_player) = app.add_player.as_mut() {
        let mut result = None;
        pop_up_menus::new_player_menu(ctx, &mut result, new_player);
        if let Some(create) = result {
            if create {
                let _ = db_helper::player_funcs::insert_player_to_db(app.database.as_ref().unwrap().clone(), new_player.clone());
            }
            app.add_player = None;
        }
    }

    if let Some(edit_player) = app.edit_player.as_mut() {
        let mut result = None;
        pop_up_menus::edit_player_menu(ctx, &mut result, edit_player, app.logged_in_player.1 || app.logged_in_player.0 == edit_player.id);
        if let Some(create) = result {
            if create {
                let _ = db_helper::player_funcs::update_player_in_db(app.database.as_ref().unwrap().clone(), edit_player.clone());
            }
            app.edit_player = None;
        }
    }

    if let Some((name, id)) = app.delete_player.as_mut() {
        let mut result = None;
        pop_up_menus::delete_player_menu(ctx, &mut result, &name);
        if let Some(create) = result {
            if create {
                let _ = db_helper::player_funcs::delete_player_from_db(app.database.as_ref().unwrap().clone(), *id);
            }
            app.delete_player = None;
        }
    }

    if app.log_in_details.0 {
        let mut result = None;
        pop_up_menus::log_in_menu(ctx, &mut result, &mut app.log_in_details);
        if let Some(log_in) = result {
            if log_in {
                if app.log_in_details.3 {
                    app.logged_in_player = (-1, false);
                } else {
                    let player = db_helper::player_funcs::get_player_from_login(app.database.as_ref().unwrap().clone(), &app.log_in_details.1, &app.log_in_details.2);
                    println!("{:?}", player);
                    if player.is_ok() {
                        app.logged_in_player = player.unwrap();
                    }
                }
            }
            app.log_in_details = (false, "".to_string(), "".to_string(), false);
        }
    }
}