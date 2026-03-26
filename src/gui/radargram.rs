use eframe::egui;

#[derive(Default)]
pub struct RadargramState {
    pub show: bool,
    zoom: f32,
}

impl RadargramState {

    pub fn draw(
        &mut self,
        ctx:&egui::Context,
        rows:&Vec<Vec<String>>
    ){

        if !self.show { return; }

        egui::Window::new("RADARGRAM — Underground Scan")
            .id(egui::Id::new("radargram_window_unique"))
            .default_size([1000.0,600.0])
            .resizable(true)
            .show(ctx, |ui|{

            if rows.is_empty(){
                ui.label("Load RD7 data first");
                return;
            }

            // ===== controls =====
            ui.horizontal(|ui|{
                ui.label("Zoom");
                ui.add(egui::Slider::new(&mut self.zoom,0.5..=6.0));
            });

            ui.separator();

            let cell_w = 3.0 * self.zoom;
            let cell_h = 2.0 * self.zoom;

            // ===== radar view =====
            egui::ScrollArea::both()
                .id_salt("radar_scroll_unique")   //  updated method
                .show(ui, |ui|{

                for r in 0..rows.len(){

                    ui.horizontal(|ui|{

                        for c in 0..rows[r].len(){

                            let val:f32 = rows[r][c].parse().unwrap_or(0.0);

                            // grayscale radar
                            let gray = ((val.abs() * 2.0) % 255.0) as u8;
                            let color = egui::Color32::from_gray(gray);

                            let (rect,_) = ui.allocate_exact_size(
                                egui::vec2(cell_w,cell_h),
                                egui::Sense::hover()
                            );

                            ui.painter().rect_filled(rect,0.0,color);
                        }

                    });

                }

            });

        });
    }
}
