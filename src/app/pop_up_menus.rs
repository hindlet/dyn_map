use eframe::egui::{self, Context};




pub fn new_map_menu(ctx: &Context, result: &mut Option<bool>, name: &mut String) {
    egui::Window::new("New Map")
    .collapsible(false)
    .resizable(false)
    .show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label("Name:");
            ui.text_edit_singleline(name);
        });
        ui.horizontal(|ui| {
            if ui.button("✅ Confirm").clicked() {
                *result = Some(true);
            };
            if ui.button("❌ Cancel").clicked() {
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
            if ui.button("❌ Cancel").clicked() {
                *result = Some(false);
            };
        });
    });
}