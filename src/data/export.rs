use std::fs::File;
use std::io::Write;
use std::path::Path;

// ⭐ MAIN SAVE FUNCTION USED BY LAYOUT
pub fn save_csv(path:&Path, headers:&Vec<String>, rows:&Vec<Vec<String>>) {

    let mut file = File::create(path).expect("Unable to create file");

    // ===== write headers =====
    if !headers.is_empty() {
        let header_line = headers.join(",");
        writeln!(file,"{}",header_line).ok();
    }

    // ===== write rows =====
    for r in rows {
        let line = r.join(",");
        writeln!(file,"{}",line).ok();
    }

    println!("File saved successfully");
}
