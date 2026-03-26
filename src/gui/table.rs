use eframe::egui;

pub struct TableState {
    pub row_height: f32,
    pub col_width: f32,
    pub edit_mode: bool,
    pub visible_col_start: usize,
}

impl Default for TableState {
    fn default() -> Self {
        Self {
            row_height: 22.0,
            col_width: 120.0,
            edit_mode: false,
            visible_col_start: 0,
        }
    }
}

impl TableState {

    pub fn draw_table_with_highlight(
        &mut self,
        ui: &mut egui::Ui,
        headers: &Vec<String>,
        rows: &mut Vec<Vec<String>>,
        highlight: Option<usize>,
    ){

        let total_rows = rows.len();
        let total_cols = headers.len();

        egui::ScrollArea::both().show(ui, |ui| {

            ui.horizontal(|ui| {
                for c in self.visible_col_start..
                    (self.visible_col_start+50).min(total_cols)
                {
                    ui.add_sized(
                        [self.col_width,20.0],
                        egui::Label::new(
                            egui::RichText::new(&headers[c]).strong()
                        )
                    );
                }
            });

            ui.separator();

            egui::ScrollArea::vertical()
                .auto_shrink([false;2])
                .show_rows(ui,self.row_height,total_rows,|ui,row_range|{

                    for r in row_range {

                        let is_highlight = highlight == Some(r);

                        ui.horizontal(|ui|{

                            if is_highlight {
                                ui.visuals_mut().override_text_color =
                                    Some(egui::Color32::YELLOW);
                            }

                            for c in self.visible_col_start..
                                (self.visible_col_start+50).min(total_cols)
                            {
                                if self.edit_mode {
                                    ui.add_sized(
                                        [self.col_width,self.row_height],
                                        egui::TextEdit::singleline(&mut rows[r][c])
                                    );
                                } else {
                                    ui.add_sized(
                                        [self.col_width,self.row_height],
                                        egui::Label::new(&rows[r][c])
                                    );
                                }
                            }

                            ui.visuals_mut().override_text_color = None;
                        });

                    }
                });

        });

        if ui.input(|i| i.raw_scroll_delta.x != 0.0) {
            if ui.input(|i| i.raw_scroll_delta.x) > 0.0 {
                self.visible_col_start =
                    (self.visible_col_start + 3).min(total_cols.saturating_sub(1));
            } else {
                self.visible_col_start =
                    self.visible_col_start.saturating_sub(3);
            }
        }
    }
}
