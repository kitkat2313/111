pub struct SearchState {
    pub query: String,
    pub results: Vec<usize>,
    pub current: usize,
}

impl Default for SearchState {
    fn default() -> Self {
        Self {
            query: String::new(),
            results: vec![],
            current: 0,
        }
    }
}

impl SearchState {

    pub fn run_search(&mut self, rows:&Vec<Vec<String>>){

        self.results.clear();
        self.current = 0;

        if self.query.is_empty() { return; }

        let q = self.query.to_lowercase();

        for (i,row) in rows.iter().enumerate() {
            for cell in row {
                if cell.to_lowercase().contains(&q) {
                    self.results.push(i);
                    break;
                }
            }
        }
    }

    pub fn next(&mut self){
        if !self.results.is_empty(){
            self.current = (self.current+1)%self.results.len();
        }
    }

    pub fn prev(&mut self){
        if !self.results.is_empty(){
            if self.current==0{
                self.current=self.results.len()-1;
            }else{
                self.current-=1;
            }
        }
    }

    pub fn current_row(&self)->Option<usize>{
        if self.results.is_empty(){None}
        else{Some(self.results[self.current])}
    }
}
