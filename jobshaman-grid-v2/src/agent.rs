use crate::permissions::AgentPermissions;
use crate::bento_grid::BentoGrid;
use egui::Pos2;

#[cfg(feature = "llama")]
use llama_cpp_rs::{LlamaModel, LlamaInference};

pub struct Agent {
    perm: AgentPermissions,
    #[cfg(feature = "ollama")]
    ollama: Option<ollama_rs::Ollama>,
    #[cfg(feature = "llama")]
    llama: Option<LlamaModel>,
}

impl Agent {
    pub fn new() -> Self {
        let perm = AgentPermissions::load();
        #[cfg(feature = "ollama")]
        let ollama = if perm.agent.enabled && perm.agent.backend == "ollama" {
            Some(ollama_rs::Ollama::default())
        } else { None };
        #[cfg(feature = "llama")]
        let llama = if perm.agent.enabled && perm.agent.backend == "llama" {
            // Předpokládáme cestu k modelu v permissions.agent.model
            LlamaModel::load_from_file(&perm.agent.model, Default::default()).ok()
        } else { None };
        Self {
            perm,
            #[cfg(feature = "ollama")]
            ollama,
            #[cfg(feature = "llama")]
            llama,
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.perm.agent.enabled && (
            #[cfg(feature = "ollama")] self.ollama.is_some() ||
            #[cfg(feature = "llama")] self.llama.is_some()
        )
    }

    pub fn process_command(&self, cmd: &str, grid: &mut BentoGrid) -> String {
        if !self.is_enabled() {
            return "Agent disabled.".to_string();
        }
        if let Some(resp) = self.try_direct(cmd, grid) {
            return resp;
        }
        // Zde by se volalo LLM (Ollama nebo llama.cpp) podle backendu
        format!("AI processing not fully implemented. Command: {}", cmd)
    }

    fn try_direct(&self, cmd: &str, grid: &mut BentoGrid) -> Option<String> {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        match parts.as_slice() {
            ["move", id_str, row, col] => {
                if !self.perm.permissions.modify_grid { return Some("No permission".into()); }
                let id = id_str.parse::<usize>().ok()?;
                let row = row.parse::<usize>().ok()?;
                let col = col.parse::<usize>().ok()?;
                // Toto je zjednodušené; bento_grid by mělo mít metodu move_widget_to
                // grid.move_widget_to(id, row, col);
                Some(format!("Moved widget {} to row {}, col {}", id, row, col))
            }
            _ => None,
        }
    }
}
