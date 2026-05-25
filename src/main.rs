mod bento_grid;
mod agent;
mod permissions;
mod watcher;
mod widgets;
mod storage;

use eframe::egui;
use bento_grid::BentoGrid;
use agent::Agent;
use watcher::VaultWatcher;
use std::sync::Arc;
use std::sync::Mutex;

struct JobShamanApp {
    grid: BentoGrid,
    agent: Agent,
    watcher: Arc<Mutex<VaultWatcher>>,
    chat_input: String,
    agent_enabled: bool,
    last_watcher_events: Vec<String>,
}

impl Default for JobShamanApp {
    fn default() -> Self {
        let agent = Agent::new();
        let agent_enabled = agent.is_enabled();
        let watcher = VaultWatcher::new();
        Self {
            grid: BentoGrid::load_or_default(),
            agent,
            watcher: Arc::new(Mutex::new(watcher)),
            chat_input: String::new(),
            agent_enabled,
            last_watcher_events: Vec::new(),
        }
    }
}

impl eframe::App for JobShamanApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Zpracování agenta
        if self.agent_enabled && !self.chat_input.is_empty() && self.chat_input.ends_with('\n') {
            let input = self.chat_input.trim().to_string();
            self.chat_input.clear();
            let response = self.agent.process_command(&input, &mut self.grid);
            println!("Agent: {}", response);
        }

        // Přečtení událostí z watcheru
        if let Ok(mut w) = self.watcher.lock() {
            let events = w.drain_events();
            if !events.is_empty() {
                self.last_watcher_events = events;
                ctx.request_repaint();
            }
        }

        // Hlavní panel – bento grid
        egui::CentralPanel::default().show(ctx, |ui| {
            self.grid.ui(ui);
        });

        // Spodní lišta s agentem a stavem watcheru
        egui::TopBottomPanel::bottom("agent_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if self.agent_enabled {
                    ui.label("🧙 Ask Shaman:");
                    let resp = ui.text_edit_singleline(&mut self.chat_input);
                    if resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        self.chat_input.push('\n');
                    }
                    if ui.button("Send").clicked() {
                        self.chat_input.push('\n');
                    }
                } else {
                    ui.colored_label(egui::Color32::GRAY, "Agent disabled");
                }
                ui.separator();
                if !self.last_watcher_events.is_empty() {
                    ui.label(format!("📁 Vault change: {}", self.last_watcher_events.last().unwrap()));
                }
            });
        });

        ctx.request_repaint();
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1920.0, 1080.0])
            .with_maximized(true),
        ..Default::default()
    };
    eframe::run_native("JobShaman Grid v2", options, Box::new(|_cc| Box::new(JobShamanApp::default())))
}
