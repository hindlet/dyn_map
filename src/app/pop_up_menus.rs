use eframe::egui::{self, Context, Key};

use crate::data_structs::Player;



pub fn log_in_menu(ctx: &Context, result: &mut Option<bool>, info: &mut (bool, String, String, bool)) {
    egui::Window::new("Log In")
    .collapsible(false)
    .resizable(false)
    .show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label("Username:");
            ui.text_edit_singleline(&mut info.1);
        });
        ui.horizontal(|ui| {
            ui.label("Password:");
            ui.text_edit_singleline(&mut info.2);
        });
        ui.horizontal(|ui| {
            if ui.button("✅ Confirm").clicked() {
                *result = Some(true);
            };
            if ui.button("❌ Cancel").clicked() || ctx.input(|i| {i.key_pressed(Key::Escape)}) {
                *result = Some(false);
            };
            if ui.button("🔍 View Map").clicked() {
                info.3 = true;
                *result = Some(true);
            }
        });
    });
}


//// Maps

pub fn new_map_menu(ctx: &Context, result: &mut Option<bool>, info: &mut (String, String, String)) {
    egui::Window::new("New Map")
    .collapsible(false)
    .resizable(false)
    .show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label("Map Name:");
            ui.text_edit_singleline(&mut info.0);
        });
        ui.horizontal(|ui| {
            ui.label("Your Username:");
            ui.text_edit_singleline(&mut info.1);
        });
        ui.horizontal(|ui| {
            ui.label("Your Password:");
            ui.text_edit_singleline(&mut info.2);
        });
        ui.horizontal(|ui| {
            if ui.button("✅ Confirm").clicked() {
                *result = Some(true);
            };
            if ui.button("❌ Cancel").clicked() || ctx.input(|i| {i.key_pressed(Key::Escape)}) {
                *result = Some(false);
            };
        });
    });
}

pub fn delete_map_menu(ctx: &Context, result: &mut Option<bool>, name: &str) {
    egui::Window::new("Delete Map?")
    .collapsible(false)
    .resizable(false)
    .show(ctx, |ui| {
        ui.label(format!("Are you sure you want to delete the map \"{}\"?", name));
        ui.label("This action cannot be undone.");
        ui.horizontal(|ui| {
            if ui.button("✅ Confirm").clicked() {
                *result = Some(true);
            };
            if ui.button("❌ Cancel").clicked() || ctx.input(|i| {i.key_pressed(Key::Escape)}) {
                *result = Some(false);
            };
        });
    });
}


//// Players

pub fn new_player_menu(ctx: &Context, result: &mut Option<bool>, player: &mut Player) {
    egui::Window::new("New Player")
    .collapsible(false)
    .resizable(false)
    .show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label("Name:");
            ui.text_edit_singleline(&mut player.name);
        });
        ui.horizontal(|ui| {
            ui.label("Faction:");
            ui.text_edit_singleline(&mut player.faction);
        });
        ui.horizontal(|ui| {
            ui.label("Colour:");
            ui.color_edit_button_srgba(&mut player.colour);
        });
        ui.horizontal(|ui| {
            ui.label("Admin:");
            ui.checkbox(&mut player.admin, "");
        });
        ui.horizontal(|ui| {
            ui.label("Password:");
            ui.text_edit_singleline(&mut player.password);
        });
        ui.horizontal(|ui| {
            if ui.button("✅ Confirm").clicked() {
                *result = Some(true);
            };
            if ui.button("❌ Cancel").clicked() || ctx.input(|i| {i.key_pressed(Key::Escape)}) {
                *result = Some(false);
            };
        });
    });
}

pub fn edit_player_menu(ctx: &Context, result: &mut Option<bool>, player: &mut Player, edit_password: bool) {
    egui::Window::new("Edit Player")
    .collapsible(false)
    .resizable(false)
    .show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label("Name:");
            ui.text_edit_singleline(&mut player.name);
        });
        ui.horizontal(|ui| {
            ui.label("Faction:");
            ui.text_edit_singleline(&mut player.faction);
        });
        ui.horizontal(|ui| {
            ui.label("Colour:");
            ui.color_edit_button_srgba(&mut player.colour);
        });
        ui.horizontal(|ui| {
            ui.label("Admin:");
            ui.checkbox(&mut player.admin, "");
        });
        if edit_password {
            ui.horizontal(|ui| {
                ui.label("Password:");
                ui.text_edit_singleline(&mut player.password);
            });
        }
        ui.horizontal(|ui| {
            if ui.button("✅ Confirm").clicked() {
                *result = Some(true);
            };
            if ui.button("❌ Cancel").clicked() || ctx.input(|i| {i.key_pressed(Key::Escape)}) {
                *result = Some(false);
            };
        });
    });
}

pub fn delete_player_menu(ctx: &Context, result: &mut Option<bool>, name: &str) {
    egui::Window::new("Delete Player?")
    .collapsible(false)
    .resizable(false)
    .show(ctx, |ui| {
        ui.label(format!("Are you sure you want to delete the player \"{}\"?", name));
        ui.label("This action cannot be undone.");
        ui.horizontal(|ui| {
            if ui.button("✅ Confirm").clicked() {
                *result = Some(true);
            };
            if ui.button("❌ Cancel").clicked() || ctx.input(|i| {i.key_pressed(Key::Escape)}) {
                *result = Some(false);
            };
        });
    });
}