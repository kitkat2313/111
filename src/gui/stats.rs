pub struct StatsState {
    pub show: bool,
    pub col: usize,

    pub min: f64,
    pub max: f64,
    pub avg: f64,
    pub sum: f64,
    pub count: usize,
}

impl Default for StatsState {
    fn default() -> Self {
        Self {
            show:false,
            col:0,
            min:0.0,
            max:0.0,
            avg:0.0,
            sum:0.0,
            count:0,
        }
    }
}

impl StatsState {

    pub fn calculate(&mut self, rows:&Vec<Vec<String>>) {

        let mut values:Vec<f64> = vec![];

        for r in rows {
            if self.col < r.len() {
                if let Ok(v)=r[self.col].parse::<f64>() {
                    values.push(v);
                }
            }
        }

        if values.is_empty(){ return; }

        self.count = values.len();
        self.sum = values.iter().sum();
        self.min = values.iter().cloned().fold(f64::INFINITY,f64::min);
        self.max = values.iter().cloned().fold(f64::NEG_INFINITY,f64::max);
        self.avg = self.sum / self.count as f64;
    }

    pub fn draw(
        &mut self,
        ctx:&egui::Context,
        headers:&Vec<String>,
        rows:&Vec<Vec<String>>,
    ){

        if !self.show {return;}

        egui::Window::new("STATISTICS")
            .default_size([300.0,300.0])
            .show(ctx, |ui|{

                if headers.is_empty(){
                    ui.label("No data loaded");
                    return;
                }

                ui.label("Select Column:");

                egui::ComboBox::from_id_salt("stat_col")
                    .selected_text(headers[self.col].clone())
                    .show_ui(ui, |ui|{
                        for (i,h) in headers.iter().enumerate(){
                            ui.selectable_value(&mut self.col,i,h);
                        }
                    });

                if ui.button("Calculate").clicked(){
                    self.calculate(rows);
                }

                ui.separator();
                ui.label(format!("Count: {}",self.count));
                ui.label(format!("Sum: {:.3}",self.sum));
                ui.label(format!("Average: {:.3}",self.avg));
                ui.label(format!("Min: {:.3}",self.min));
                ui.label(format!("Max: {:.3}",self.max));

                if ui.button("Close").clicked(){
                    self.show=false;
                }
            });
    }
}
