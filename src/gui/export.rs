use std::fs::File;
use std::io::Write;
use rfd::FileDialog;

pub struct ExportState {
    pub show: bool,
}

impl Default for ExportState {
    fn default() -> Self {
        Self{ show:false }
    }
}

impl ExportState {

    // EXPORT CSV
    pub fn export_csv(headers:&Vec<String>, rows:&Vec<Vec<String>>){

        if let Some(path)=FileDialog::new()
            .set_file_name("export.csv")
            .save_file()
        {
            if let Ok(mut w)=csv::Writer::from_path(path){
                let _=w.write_record(headers);

                for r in rows{
                    let _=w.write_record(r);
                }

                let _=w.flush();
            }
        }
    }

    // EXPORT TXT
    pub fn export_txt(headers:&Vec<String>, rows:&Vec<Vec<String>>){

        if let Some(path)=FileDialog::new()
            .set_file_name("export.txt")
            .save_file()
        {
            if let Ok(mut f)=File::create(path){

                let head=headers.join(",");
                let _=writeln!(f,"{}",head);

                for r in rows{
                    let line=r.join(",");
                    let _=writeln!(f,"{}",line);
                }
            }
        }
    }

    pub fn draw(
        &mut self,
        ctx:&egui::Context,
        headers:&Vec<String>,
        rows:&Vec<Vec<String>>,
    ){

        if !self.show {return;}

        egui::Window::new("EXPORT / CONVERT")
            .default_size([300.0,250.0])
            .show(ctx, |ui|{

                ui.heading("Export Data");
                ui.separator();

                if ui.button("Export CSV").clicked(){
                    Self::export_csv(headers,rows);
                }

                if ui.button("Export TXT").clicked(){
                    Self::export_txt(headers,rows);
                }

                ui.separator();
                ui.label("More formats coming:");
                ui.label("Excel export");
                ui.label("Graph PNG export");
                ui.label("Elevation PNG");

                ui.separator();

                if ui.button("Close").clicked(){
                    self.show=false;
                }
            });
    }
}
