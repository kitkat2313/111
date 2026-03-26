use eframe::egui;

#[derive(Default)]
pub struct RadarState {
    pub show: bool,
    pub zoom: f32,
    texture: Option<egui::TextureHandle>,
}

impl RadarState {

    pub fn draw(
        &mut self,
        ctx:&egui::Context,
        _headers:&Vec<String>,
        rows:&Vec<Vec<String>>
    ){

        if !self.show { return; }

        egui::Window::new("RADAR VIEW (Underground Scan)")
            .default_size([900.0,500.0])
            .resizable(true)
            .vscroll(true)
            .show(ctx, |ui|{

            if rows.is_empty(){
                ui.label("No RD7 data loaded");
                return;
            }

            // ===== HEADER =====
            ui.horizontal(|ui|{
                ui.heading("RADAR SCAN");
                ui.separator();
                ui.label("Zoom");
                ui.add(egui::Slider::new(&mut self.zoom,0.5..=6.0));
                if ui.button("Refresh").clicked(){
                    self.texture = None;
                }
            });

            ui.separator();

            // ===== CREATE IMAGE ONLY ONCE =====
            if self.texture.is_none() {

                let width = rows[0].len();
                let height = rows.len();

                let mut pixels: Vec<egui::Color32> =
                    Vec::with_capacity(width * height);

                for r in 0..height {
                    for c in 0..width {

                        let val:f32 = rows[r][c].parse().unwrap_or(0.0);

                        // professional radar colors
                        let color = if val < 20.0 {
                            egui::Color32::BLACK
                        }
                        else if val < 40.0 {
                            egui::Color32::from_rgb(40,40,40)
                        }
                        else if val < 60.0 {
                            egui::Color32::from_rgb(255,220,0)
                        }
                        else if val < 90.0 {
                            egui::Color32::from_rgb(255,120,0)
                        }
                        else {
                            egui::Color32::from_rgb(255,0,0)
                        };

                        pixels.push(color);
                    }
                }

                let image = egui::ColorImage {
                    size: [width, height],
                    pixels,
                };

                self.texture = Some(
                    ctx.load_texture(
                        "radar_texture",
                        image,
                        egui::TextureOptions::NEAREST
                    )
                );
            }

            // ===== DRAW TEXTURE =====
            if let Some(tex) = &self.texture {

                let size = tex.size_vec2() * self.zoom;

                egui::ScrollArea::both().show(ui, |ui|{

                    ui.add(
                        egui::Image::new(tex)
                            .fit_to_exact_size(size)
                    );

                });
            }

        });
    }
}
