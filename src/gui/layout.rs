use eframe::{egui, App};
use std::time::{Duration, Instant};
use std::path::PathBuf;

use crate::gui::radargram::RadargramState;
use crate::gui::radar::RadarState;

use crate::gui::table::TableState;
use crate::gui::search::SearchState;
use crate::gui::filter::FilterState;
use crate::gui::graph::GraphState;
use crate::gui::elevation::ElevationState;
use crate::gui::stats::StatsState;
use crate::gui::export::ExportState;
use crate::data::rd7::Rd7Machine;

pub struct VidarshanApp {
    start: Instant,
    splash: bool,

    table: TableState,
    search: SearchState,
    filter: FilterState,
    graph: GraphState,
    elevation: ElevationState,
    stats: StatsState,
    export: ExportState,
    radar: RadarState,
    radargram: RadargramState,

    headers: Vec<String>,
    rows: Vec<Vec<String>>,

    show_database: bool,
    show_main: bool,
    main_max: bool,
    show_tools: bool,
    show_filter: bool,

    dark_mode: bool,

    rd7_popup: bool,
    rd7_path: Option<PathBuf>,
    rd7_machine: Rd7Machine,

    current_file: Option<PathBuf>,
    project_name: String,
}

impl VidarshanApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            start: Instant::now(),
            splash: true,

            table: TableState::default(),
            search: SearchState::default(),
            filter: FilterState::default(),
            graph: GraphState::default(),
            elevation: ElevationState::default(),
            stats: StatsState::default(),
            export: ExportState::default(),
            radar: RadarState::default(),
            radargram: RadargramState::default(),

            headers: vec![],
            rows: vec![],

            show_database: true,
            show_main: true,
            main_max: false,
            show_tools: true,
            show_filter: false,

            dark_mode: false,

            rd7_popup: false,
            rd7_path: None,
            rd7_machine: Rd7Machine::IDS,

            current_file: None,
            project_name: "Untitled".to_string(),
        }
    }
}

impl App for VidarshanApp {
fn update(&mut self, ctx:&egui::Context, _:&mut eframe::Frame){

// THEME
if self.dark_mode { ctx.set_visuals(egui::Visuals::dark()); }
else { ctx.set_visuals(egui::Visuals::light()); }

// SPLASH
if self.splash {
    if self.start.elapsed() > Duration::from_secs(2) { self.splash = false; }
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.centered_and_justified(|ui| { ui.heading("VIDARSHAN ENGINE"); });
    });
    return;
}

// MENU
egui::TopBottomPanel::top("menu").show(ctx, |ui| {
ui.horizontal(|ui|{

// FILE MENU
ui.menu_button("FILE", |ui| {

    if ui.button("🆕 New Project").clicked() {
        self.headers.clear();
        self.rows.clear();
        self.current_file=None;
        self.project_name="Untitled".to_string();
        ui.close_menu();
    }

    if ui.button("📂 Open").clicked() {
    if let Some(path)=rfd::FileDialog::new()
        .add_filter("All",&["csv","xlsx","xls","rd7","asc","cor","gpv"])
        .pick_file()
    {
        let ext=path.extension().and_then(|e|e.to_str()).unwrap_or("").to_lowercase();

        

        if ext=="csv"{
            let (h,r)=crate::data::csv_excel::load_csv(&path);
            self.headers=h; self.rows=r;
        }

        if ext=="xlsx"||ext=="xls"{
            let (h,r)=crate::data::csv_excel::load_excel(&path);
            self.headers=h; self.rows=r;
        }

        if ext=="rd7"||ext=="asc"||ext=="cor"{
            self.rd7_popup=true;
            self.rd7_path=Some(path.clone());
        }
        if ext=="gpv"{
    let proj = crate::data::project::load_project(&path);

    self.project_name = proj.project_name;
    self.headers = proj.headers;
    self.rows = proj.rows;
    self.dark_mode = proj.dark_mode;
    self.radar.zoom = proj.radar_zoom;
    self.rd7_machine = proj.machine;
}

    }
    ui.close_menu();
}

    if ui.button("💾 Save").clicked() {
        if let Some(path)=&self.current_file {
            crate::data::export::save_csv(path,&self.headers,&self.rows);
        } else {
            if let Some(path)=rfd::FileDialog::new().set_file_name("project.csv").save_file(){
                crate::data::export::save_csv(&path,&self.headers,&self.rows);
                self.current_file=Some(path);
            }
        }
        ui.close_menu();
    }

  if ui.button("💾 Save Project").clicked() {

    if let Some(folder)=rfd::FileDialog::new().pick_folder(){

        let graph_path = folder.join("graph.jpg");
        let radar_path = folder.join("radar.jpg");
        let project_path = folder.join("project.gpv");

        // save images
        crate::data::save_image::save_dummy_graph(&graph_path);
        crate::data::save_image::save_dummy_graph(&radar_path);

        // save project file
        crate::data::project::save_project(
            &project_path,
            &self.project_name,
            &self.headers,
            &self.rows,
            self.dark_mode,
            self.radar.zoom,
            &self.rd7_machine,
            &graph_path,
            &radar_path
        );
    }

    ui.close_menu();
}


    ui.separator();
    if ui.button("Exit").clicked(){ std::process::exit(0); }
});

// EDIT
ui.menu_button("EDIT", |ui| {
    if ui.button("Edit Mode").clicked(){ self.table.edit_mode=true; ui.close_menu();}
    if ui.button("View Mode").clicked(){ self.table.edit_mode=false; ui.close_menu();}
});

// VIEW
ui.menu_button("VIEW", |ui| {
if ui.button("Dark Mode").clicked(){ self.dark_mode=true; }
if ui.button("Light Mode").clicked(){ self.dark_mode=false; }

ui.separator();
if ui.button("Toggle Main").clicked(){ self.show_main=!self.show_main; }
if ui.button("Toggle Database").clicked(){ self.show_database=!self.show_database; }
if ui.button("Toggle Tools").clicked(){ self.show_tools=!self.show_tools; }
if ui.button("Toggle Graph").clicked(){ self.graph.show=!self.graph.show; }
if ui.button("Toggle Elevation").clicked(){ self.elevation.show=!self.elevation.show; }
if ui.button("Toggle Stats").clicked(){ self.stats.show=!self.stats.show; }
if ui.button("Toggle Export").clicked(){ self.export.show=!self.export.show; }
if ui.button("Toggle Radar").clicked(){ self.radar.show=!self.radar.show; }
if ui.button("Toggle RadarGram").clicked(){ self.radargram.show=!self.radargram.show; }
});

ui.separator();
ui.label("Search:");
let resp=ui.text_edit_singleline(&mut self.search.query);
if resp.changed(){ self.search.run_search(&self.rows); }
if ui.button("◀").clicked(){ self.search.prev(); }
if ui.button("▶").clicked(){ self.search.next(); }
ui.label(format!("{} results",self.search.results.len()));
ui.separator();
ui.label(format!("Rows:{} Cols:{}",self.rows.len(),self.headers.len()));
});
});

// BACKGROUND
// ===== GREY BACKGROUND =====
egui::CentralPanel::default()
    .frame(
        egui::Frame::none()
            .fill(egui::Color32::from_gray(180))   // grey color
    )
    .show(ctx, |_| {});


 // ===== MAIN WINDOW WITH MIN MAX  =====
if self.show_main {

    let screen = ctx.available_rect();

    let mut win = egui::Window::new("MAIN DATA")
        .resizable(true)
        .movable(true)
        .collapsible(false)
        .id(egui::Id::new("main_data_window"));

    // ===== MAX MODE =====
    if self.main_max {
        win = win
            .fixed_pos(egui::pos2(
                screen.left()+5.0,
                screen.top()+35.0
            ))
            .fixed_size(egui::vec2(
                screen.width()-10.0,
                screen.height()-45.0
            ));
    }
    // ===== NORMAL MODE =====
    else {
        win = win
            .default_pos(egui::pos2(250.0,120.0))
            .default_size(egui::vec2(750.0,500.0));
    }

    win.show(ctx, |ui|{

        ui.horizontal(|ui|{
            ui.heading("DATA VIEW");

            ui.with_layout(
                egui::Layout::right_to_left(egui::Align::Center),
                |ui|{

                    if ui.button("❌").clicked(){
                        self.show_main=false;
                    }

                    // MAX BUTTON
                    if ui.button("🗖").clicked(){
                        self.main_max=true;
                    }

                    // BACK TO NORMAL
                    if ui.button("🗗").clicked(){
                        self.main_max = false;

    //  real reset window position + size
                   ctx.memory_mut(|mem| mem.reset_areas());
}
              
                }
            );
        });

        ui.separator();

        let highlight=self.search.current_row();
        let data_rows=if self.filter.active{
            &mut self.filter.filtered_rows
        }else{
            &mut self.rows
        };

        self.table.draw_table_with_highlight(
            ui,&self.headers,data_rows,highlight
        );
    });
}

// DATABASE
if self.show_database{
egui::Window::new("DATABASE")
.default_pos([20.0,80.0])
.default_size([260.0,400.0])
.show(ctx, |ui|{
ui.heading("Columns");
egui::ScrollArea::vertical().show(ui, |ui|{
for h in &self.headers{ ui.label(h); }
});
});
}

// TOOLS
if self.show_tools{
egui::Window::new("TOOLS")
.default_pos([1100.0,80.0])
.default_size([260.0,420.0])
.show(ctx, |ui|{
ui.heading("TOOLS PANEL");
ui.separator();
if ui.button("Column Filter").clicked(){ self.show_filter=true; }
if ui.button("Graph").clicked(){ self.graph.show=true; }
if ui.button("Elevation").clicked(){ self.elevation.show=true; }
if ui.button("Statistics").clicked(){ self.stats.show=true; }
if ui.button("Export").clicked(){ self.export.show=true; }
if ui.button("Radar View").clicked(){ self.radar.show=true; }
if ui.button("Radargram").clicked(){ self.radargram.show=true; }
});
}

// FILTER
if self.show_filter{
egui::Window::new("COLUMN FILTER").show(ctx, |ui|{
if !self.headers.is_empty(){
egui::ComboBox::from_id_salt("filter_col")
.selected_text(self.headers.get(self.filter.column).unwrap_or(&"Select".to_string()))
.show_ui(ui, |ui|{
for (i,h) in self.headers.iter().enumerate(){
ui.selectable_value(&mut self.filter.column,i,h);
}
});
}
ui.text_edit_singleline(&mut self.filter.query);
if ui.button("Apply").clicked(){ self.filter.apply(&self.headers,&self.rows); }
if ui.button("Clear").clicked(){ self.filter.clear(); }
if ui.button("Close").clicked(){ self.show_filter=false; }
});
}

// RD7 POPUP
if self.rd7_popup {
egui::Window::new("Select RD7 Machine").show(ctx, |ui|{
ui.radio_value(&mut self.rd7_machine, Rd7Machine::IDS, "IDS");
ui.radio_value(&mut self.rd7_machine, Rd7Machine::Mala, "MALA");
ui.radio_value(&mut self.rd7_machine, Rd7Machine::GSSI, "GSSI");
ui.radio_value(&mut self.rd7_machine, Rd7Machine::Other, "Other");

if ui.button("Load RD7").clicked(){
if let Some(p)=&self.rd7_path{
let (h,r)=crate::data::rd7::load_rd7(p,self.rd7_machine.clone());
self.headers=h; self.rows=r;
}
self.rd7_popup=false;
}
if ui.button("Cancel").clicked(){ self.rd7_popup=false; }
});
}

// DRAW PANELS
self.graph.draw(ctx,&self.headers,&self.rows);
self.elevation.draw(ctx,&self.headers,&self.rows);
self.stats.draw(ctx,&self.headers,&self.rows);
self.export.draw(ctx,&self.headers,&self.rows);
self.radar.draw(ctx,&self.headers,&self.rows);
self.radargram.draw(ctx,&self.rows);

}
}


