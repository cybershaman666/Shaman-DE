use egui::Ui;

pub trait Widget {
    fn title(&self) -> String;
    fn ui(&mut self, ui: &mut Ui);
}

pub mod terminal;
pub mod system;
