use egui::{Ui, Color32, FontId, RichText};
use super::Widget;

pub struct TerminalWidget {
    content: String,
}

impl TerminalWidget {
    pub fn new() -> Self {
        Self { content: String::from("$ ") }
    }
}

impl Widget for TerminalWidget {
    fn title(&self) -> String { "Terminal".to_string() }

    fn ui(&mut self, ui: &mut Ui) {
        let text = RichText::new(&self.content)
            .font(FontId::monospace(14.0))
            .color(Color32::WHITE);
        ui.add(egui::TextEdit::multiline(&mut self.content)
            .desired_width(f32::INFINITY)
            .desired_rows(20)
            .background_color(Color32::from_rgb(30,30,30)));
    }
}
