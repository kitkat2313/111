mod gui;
mod data;

use gui::VidarshanApp;

fn main() -> Result<(), eframe::Error> {

    let mut options = eframe::NativeOptions::default();

    options.viewport = egui::ViewportBuilder::default()
        .with_inner_size([1400.0, 900.0])
        .with_title("VIDARSHAN by AORVIS");

    eframe::run_native(
        "VIDARSHAN",
        options,
        Box::new(|cc| {

            //  CRITICAL FIX: disable egui debug painter
            cc.egui_ctx.set_debug_on_hover(false);

            Ok(Box::new(VidarshanApp::new(cc)))
        }),
    )
}
