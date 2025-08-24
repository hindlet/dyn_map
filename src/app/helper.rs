use eframe::egui::{Color32, Sense, Ui};



pub fn colour_display_box(ui: &mut Ui, colour: [u8; 3]) {
    let size = ui.spacing().interact_size;
    let (rect, _response) = ui.allocate_exact_size(size, Sense::all());

    ui.painter().rect_filled(rect, 0.0, Color32::from_rgb(colour[0], colour[1], colour[2]));
}