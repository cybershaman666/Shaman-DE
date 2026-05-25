use notify::{RecommendedWatcher, Watcher, RecursiveMode, Result, Event, EventKind};
use std::sync::mpsc::{channel, Receiver};
use std::path::PathBuf;
use std::fs;
use directories::ProjectDirs;
use serde::Deserialize;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct VaultEvent {
    pub path: PathBuf,
    pub kind: String,
}

pub struct VaultWatcher {
    _watcher: RecommendedWatcher,
    rx: Receiver<Result<Event>>,
    events: Vec<String>,
    vault_path: PathBuf,
}

impl VaultWatcher {
    pub fn new() -> Self {
        let config = Self::load_config();
        let vault_path = config.vault_path;
        let (tx, rx) = channel();
        let mut watcher = notify::recommended_watcher(tx).unwrap();
        if vault_path.exists() {
            watcher.watch(&vault_path, RecursiveMode::Recursive).unwrap();
            Self::index_vault(&vault_path);
        }
        Self {
            _watcher: watcher,
            rx,
            events: Vec::new(),
            vault_path,
        }
    }

    fn load_config() -> VaultConfig {
        let proj_dirs = ProjectDirs::from("com", "jobshaman", "grid").unwrap();
        let path = proj_dirs.config_dir().join("vault_watch.toml");
        if path.exists() {
            let contents = fs::read_to_string(path).unwrap();
            toml::from_str(&contents).unwrap()
        } else {
            let default = VaultConfig {
                vault_path: PathBuf::from("/home/shami/Documents/Obsidian"),
                watched_extensions: vec!["md".to_string(), "markdown".to_string()],
                index_on_start: true,
            };
            let _ = fs::create_dir_all(proj_dirs.config_dir());
            let _ = fs::write(path, toml::to_string(&default).unwrap());
            default
        }
    }

    fn index_vault(path: &PathBuf) {
        println!("🔍 Indexing vault at {:?}", path);
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            if entry.path().extension().map(|ext| ext == "md").unwrap_or(false) {
                println!("📄 {}", entry.path().display());
            }
        }
    }

    pub fn drain_events(&mut self) -> Vec<String> {
        let mut new_events = Vec::new();
        while let Ok(event) = self.rx.try_recv() {
            if let Ok(ev) = event {
                match ev.kind {
                    EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_) => {
                        for path in ev.paths {
                            let msg = format!("{}: {:?}", ev.kind, path);
                            new_events.push(msg);
                            println!("{}", msg);
                        }
                    }
                    _ => {}
                }
            }
        }
        self.events.extend(new_events.clone());
        if self.events.len() > 10 {
            self.events.drain(0..self.events.len() - 10);
        }
        new_events
    }
}

#[derive(Deserialize, Clone)]
struct VaultConfig {
    pub vault_path: PathBuf,
    pub watched_extensions: Vec<String>,
    pub index_on_start: bool,
}
