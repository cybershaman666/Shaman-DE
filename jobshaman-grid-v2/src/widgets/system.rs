use egui::{Ui, Color32};
use sysinfo::{System, SystemExt, CpuExt};
use super::Widget;

pub struct SystemWidget {
    sys: System,
}

impl SystemWidget {
    pub fn new() -> Self {
        Self { sys: System::new_all() }
    }
}

impl Widget for SystemWidget {
    fn title(&self) -> String { "System Monitor".to_string() }

    fn ui(&mut self, ui: &mut Ui) {
        self.sys.refresh_all();
        ui.label(format!("CPU: {:.1}%", self.sys.global_cpu_info().cpu_usage()));
        let used = self.sys.used_memory() as f32 / 1024.0;
        let total = self.sys.total_memory() as f32 / 1024.0;
        ui.label(format!("RAM: {:.1} / {:.0} GB", used, total));
    }
}
