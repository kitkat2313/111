use std::path::Path;
use calamine::{Reader, open_workbook_auto};

pub fn load_csv(path:&Path)->(Vec<String>,Vec<Vec<String>>){

    let mut headers = vec![];
    let mut rows = vec![];

    let mut rdr = csv::Reader::from_path(path).unwrap();

    headers = rdr.headers()
        .unwrap()
        .iter()
        .map(|s|s.to_string())
        .collect();

    for result in rdr.records(){
        let rec = result.unwrap();
        rows.push(rec.iter().map(|s|s.to_string()).collect());
    }

    (headers,rows)
}

pub fn load_excel(path:&Path)->(Vec<String>,Vec<Vec<String>>){

    let mut headers = vec![];
    let mut rows = vec![];

    let mut workbook = open_workbook_auto(path).unwrap();
    let sheet = workbook.sheet_names()[0].clone();
    let range = workbook.worksheet_range(&sheet).unwrap();

    let mut iter = range.rows();

    if let Some(first_row) = iter.next() {
        headers = first_row.iter().map(|c|c.to_string()).collect();
    }

    for r in iter {
        rows.push(r.iter().map(|c|c.to_string()).collect());
    }

    (headers,rows)
}
