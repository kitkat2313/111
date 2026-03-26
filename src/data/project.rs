use std::path::Path;
use std::fs::{File};
use std::io::{Read};
use crate::data::rd7::Rd7Machine;

use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct GpvProject {
    pub project_name:String,
    pub headers:Vec<String>,
    pub rows:Vec<Vec<String>>,
    pub dark_mode:bool,
    pub radar_zoom:f32,
    pub machine:Rd7Machine,

    pub graph_img:String,
    pub radar_img:String,
}

// ================= SAVE GPV =================
pub fn save_project(
    path:&Path,
    name:&String,
    headers:&Vec<String>,
    rows:&Vec<Vec<String>>,
    dark:bool,
    zoom:f32,
    machine:&Rd7Machine,
    graph:&Path,
    radar:&Path
){
    let proj = GpvProject{
        project_name:name.clone(),
        headers:headers.clone(),
        rows:rows.clone(),
        dark_mode:dark,
        radar_zoom:zoom,
        machine:machine.clone(),
        graph_img:graph.to_string_lossy().to_string(),
        radar_img:radar.to_string_lossy().to_string(),
    };

    let json = serde_json::to_string_pretty(&proj).unwrap();
    std::fs::write(path,json).unwrap();
}

// ================= LOAD GPV =================
pub fn load_project(path:&Path)->GpvProject{

    let mut txt = String::new();
    File::open(path)
        .unwrap()
        .read_to_string(&mut txt)
        .unwrap();

    let proj:GpvProject = serde_json::from_str(&txt).unwrap();

    proj
}
