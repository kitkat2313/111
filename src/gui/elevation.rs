use egui_plot::{Plot, Line, PlotPoints};
use egui::Color32;

pub struct ElevationState {
    pub show: bool,
    pub col_x: usize,
    pub col_y: usize,
}

impl Default for ElevationState {
    fn default() -> Self {
        Self {
            show: false,
            col_x: 0,
            col_y: 1,
        }
    }
}

impl ElevationState {

    pub fn draw(
        &mut self,
        ctx:&egui::Context,
        headers:&Vec<String>,
        rows:&Vec<Vec<String>>,
    ){

        if !self.show { return; }

        egui::Window::new("ELEVATION PROFILE")
            .default_size([700.0,500.0])
            .resizable(true)
            .show(ctx, |ui|{

                if headers.len()<2 {
                    ui.label("Need numeric columns");
                    return;
                }

                ui.horizontal(|ui|{

                    ui.label("Distance:");

                    egui::ComboBox::from_id_salt("ex")
                        .selected_text(headers[self.col_x].clone())
                        .show_ui(ui, |ui|{
                            for (i,h) in headers.iter().enumerate(){
                                ui.selectable_value(&mut self.col_x,i,h);
                            }
                        });

                    ui.label("Elevation:");

                    egui::ComboBox::from_id_salt("ey")
                        .selected_text(headers[self.col_y].clone())
                        .show_ui(ui, |ui|{
                            for (i,h) in headers.iter().enumerate(){
                                ui.selectable_value(&mut self.col_y,i,h);
                            }
                        });
                });

                ui.separator();

                let mut pts:Vec<[f64;2]> = vec![];

                for r in rows {
                    if self.col_x < r.len() && self.col_y < r.len() {
                        if let (Ok(x),Ok(y)) = (
                            r[self.col_x].parse::<f64>(),
                            r[self.col_y].parse::<f64>()
                        ){
                            pts.push([x,y]);
                        }
                    }
                }

                Plot::new("elevation")
                    .height(350.0)
                    .show(ui, |plot_ui|{
                        let line = Line::new(PlotPoints::from(pts))
                            .color(Color32::LIGHT_GREEN);
                        plot_ui.line(line);
                    });

                if ui.button("Close").clicked(){
                    self.show=false;
                }
            });
    }
}
