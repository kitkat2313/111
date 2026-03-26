use std::path::Path;
use serde::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum Rd7Machine {
    IDS,
    Mala,
    GSSI,
    Other,
}

// ===== MAIN RD7 LOADER =====
pub fn load_rd7(_path:&Path, machine: Rd7Machine)->(Vec<String>,Vec<Vec<String>>){

    match machine {
        Rd7Machine::IDS => load_ids(),
        Rd7Machine::Mala => load_mala(),
        Rd7Machine::GSSI => load_gssi(),
        Rd7Machine::Other => load_other(),
    }
}

// ===== IDS MACHINE =====
fn load_ids()->(Vec<String>,Vec<Vec<String>>){
    let headers = vec![
        "Distance".to_string(),
        "Depth".to_string(),
        "Amplitude".to_string(),
    ];

    let mut rows = Vec::new();
    for i in 0..5000 {
        rows.push(vec![
            format!("{}",i),
            format!("{}",i as f32 * 0.04),
            format!("{}",(i*7)%120),
        ]);
    }
    (headers,rows)
}

// ===== MALA MACHINE =====
fn load_mala()->(Vec<String>,Vec<Vec<String>>){
    let headers = vec![
        "Trace".to_string(),
        "Time(ns)".to_string(),
        "Signal".to_string(),
    ];

    let mut rows = Vec::new();
    for i in 0..4000 {
        rows.push(vec![
            format!("{}",i),
            format!("{}",i as f32 * 0.6),
            format!("{}",(i*5)%90),
        ]);
    }
    (headers,rows)
}

// ===== GSSI MACHINE =====
fn load_gssi()->(Vec<String>,Vec<Vec<String>>){
    let headers = vec![
        "Scan".to_string(),
        "Depth".to_string(),
        "Return".to_string(),
    ];

    let mut rows = Vec::new();
    for i in 0..3500 {
        rows.push(vec![
            format!("{}",i),
            format!("{}",i as f32 * 0.03),
            format!("{}",(i*9)%150),
        ]);
    }
    (headers,rows)
}

// ===== OTHER MACHINE =====
fn load_other()->(Vec<String>,Vec<Vec<String>>){
    let headers = vec![
        "X".to_string(),
        "Y".to_string(),
        "Signal".to_string(),
    ];

    let mut rows = Vec::new();
    for i in 0..2000 {
        rows.push(vec![
            format!("{}",i),
            format!("{}",i*2),
            format!("{}",(i*2)%50),
        ]);
    }
    (headers,rows)
}
