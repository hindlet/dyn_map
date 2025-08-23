use eframe::egui::{self, ComboBox, RichText};

use crate::{app::{pop_up_menus, DynamicMapApp}, data_structs::GameMap};





pub fn draw_app(
    ctx: &egui::Context,
    app: &mut DynamicMapApp
) {

    // map info
    egui::SidePanel::left("Left Panel").min_width(200.0).show(ctx, |ui| {

        ui.horizontal(|ui| {
            ui.label("Open File:");
            let selected = if app.selected_map.0 {&app.maps[app.selected_map.1].0} else {"None"};
            ComboBox::from_id_salt("map_select")
                .selected_text(selected)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut app.selected_map, (false, 0), "None");
                    for (index, map) in app.maps.iter().enumerate() {
                        ui.selectable_value(&mut app.selected_map, (true, index), &map.0);
                    }
                });
            if ui.button("➕").on_hover_text("Create New Map").clicked() {
                app.new_map = (true, "New Map".to_string());
            }
        })
    

    });


    // player key
    if app.open_map.is_some() {
        egui::SidePanel::right("Right Panel").min_width(200.0).show(ctx, |ui| {

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
                let map = GameMap::new(app.new_map.1.clone());
            }
            app.new_map = (false, "".to_string());
        }
    }


}