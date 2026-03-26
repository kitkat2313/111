pub struct FilterState {
    pub column: usize,
    pub query: String,
    pub active: bool,

    // filtered result storage
    pub filtered_rows: Vec<Vec<String>>,
}

impl Default for FilterState {
    fn default() -> Self {
        Self {
            column: 0,
            query: String::new(),
            active: false,
            filtered_rows: vec![],
        }
    }
}

impl FilterState {

    // apply filter
    pub fn apply(
        &mut self,
        _headers:&Vec<String>,
        rows:&Vec<Vec<String>>
    ){
        self.filtered_rows.clear();

        if self.query.trim().is_empty() {
            self.active = false;
            return;
        }

        let q = self.query.to_lowercase();

        for r in rows {

            if self.column < r.len() {
                let cell = r[self.column].to_lowercase();

                if cell.contains(&q) {
                    self.filtered_rows.push(r.clone());
                }
            }
        }

        self.active = true;
    }

    // clear filter
    pub fn clear(&mut self){
        self.active = false;
        self.filtered_rows.clear();
        self.query.clear();
    }
}
