use egui::{Ui, Pos2, Vec2, Rect, Id, Response, Color32, Stroke};
use std::collections::HashMap;
use crate::widgets::{Widget, terminal::TerminalWidget, system::SystemWidget};
use crate::storage;

const CELL_SIZE: f32 = 160.0;   // velikost základní buňky
const GAP: f32 = 12.0;

#[derive(Clone)]
struct WidgetPlacement {
    row: usize,
    col: usize,
    row_span: usize,
    col_span: usize,
}

pub struct BentoGrid {
    widgets: Vec<Box<dyn Widget>>,
    placements: HashMap<usize, WidgetPlacement>,
    drag_start: Option<(usize, Pos2)>,
}

impl BentoGrid {
    pub fn load_or_default() -> Self {
        if let Some(loaded) = storage::load_bento_grid() {
            return loaded;
        }
        // Výchozí rozložení: terminál 2x2, systém 1x1
        let mut placements = HashMap::new();
        placements.insert(0, WidgetPlacement { row: 0, col: 0, row_span: 2, col_span: 2 });
        placements.insert(1, WidgetPlacement { row: 0, col: 2, row_span: 1, col_span: 1 });
        Self {
            widgets: vec![
                Box::new(TerminalWidget::new()),
                Box::new(SystemWidget::new()),
            ],
            placements,
            drag_start: None,
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        let available = ui.available_size();
        let (grid_w, grid_h) = self.calc_grid_size(available);
        let origin = Pos2::new(ui.min_rect().min.x + GAP, ui.min_rect().min.y + GAP);

        for (id, widget) in self.widgets.iter_mut().enumerate() {
            let placement = self.placements.get(&id).unwrap();
            let rect = self.placement_to_rect(placement, origin, grid_w, grid_h);
            let area_id = Id::new(id);

            let area_response = egui::Area::new(area_id)
                .fixed_pos(rect.min)
                .fixed_size(rect.size())
                .show(ui.ctx(), |ui| {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(widget.title());
                            if ui.button("✖").clicked() {
                                // zavření widgetu – zatím pouze print
                                println!("Close widget {}", id);
                            }
                        });
                        widget.ui(ui);
                    });
                });

            // Drag & drop – přesouvání mezi buňkami
            if let Some(pointer_pos) = ui.ctx().pointer_interact_pos() {
                if area_response.response.hovered() && ui.input(|i| i.pointer.primary_down()) {
                    if self.drag_start.is_none() {
                        self.drag_start = Some((id, pointer_pos - rect.min));
                    }
                }
            }
            if let Some((drag_id, offset)) = self.drag_start {
                if let Some(pointer) = ui.ctx().pointer_interact_pos() {
                    let new_min = pointer - offset;
                    let new_cell = self.point_to_cell(new_min, origin, grid_w, grid_h);
                    let (row, col) = (new_cell.0 as usize, new_cell.1 as usize);
                    if row < grid_h && col < grid_w {
                        // přesun widgetu do nové buňky, pokus o umístění bez kolize
                        self.move_widget_to(drag_id, row, col);
                        self.drag_start = None;
                    }
                } else {
                    self.drag_start = None;
                }
            }
        }
        storage::save_bento_grid(self);
    }

    fn calc_grid_size(&self, available: Vec2) -> (usize, usize) {
        let cols = ((available.x - GAP) / (CELL_SIZE + GAP)).floor() as usize;
        let rows = ((available.y - GAP) / (CELL_SIZE + GAP)).floor() as usize;
        (cols.max(4), rows.max(3))
    }

    fn placement_to_rect(&self, p: &WidgetPlacement, origin: Pos2, grid_w: usize, grid_h: usize) -> Rect {
        let cell_w = CELL_SIZE + GAP;
        let cell_h = CELL_SIZE + GAP;
        let x = origin.x + p.col as f32 * cell_w;
        let y = origin.y + p.row as f32 * cell_h;
        let w = p.col_span as f32 * cell_w - GAP;
        let h = p.row_span as f32 * cell_h - GAP;
        Rect::from_min_size(Pos2::new(x, y), Vec2::new(w, h))
    }

    fn point_to_cell(&self, point: Pos2, origin: Pos2, grid_w: usize, grid_h: usize) -> (i32, i32) {
        let cell_w = CELL_SIZE + GAP;
        let cell_h = CELL_SIZE + GAP;
        let col = ((point.x - origin.x) / cell_w).floor() as i32;
        let row = ((point.y - origin.y) / cell_h).floor() as i32;
        (row, col)
    }

    fn move_widget_to(&mut self, id: usize, target_row: usize, target_col: usize) {
        // Zjednodušená verze – přesune widget na cílovou buňku, pokud je volná
        // Kontrola kolize s ostatními widgety
        let mut new_placement = WidgetPlacement {
            row: target_row,
            col: target_col,
            row_span: 1,
            col_span: 1,
        };
        // Zde by měla být detekce kolizí a komprese layoutu
        self.placements.insert(id, new_placement);
    }

    // Serializace pro storage
    pub fn to_save_data(&self) -> Vec<(usize, usize, usize, usize, usize)> {
        self.placements.iter()
            .map(|(id, p)| (*id, p.row, p.col, p.row_span, p.col_span))
            .collect()
    }

    pub fn from_save_data(data: Vec<(usize, usize, usize, usize, usize)>) -> Self {
        let mut placements = HashMap::new();
        for (id, row, col, row_span, col_span) in data {
            placements.insert(id, WidgetPlacement { row, col, row_span, col_span });
        }
        Self {
            widgets: vec![
                Box::new(TerminalWidget::new()),
                Box::new(SystemWidget::new()),
            ],
            placements,
            drag_start: None,
        }
    }
}
