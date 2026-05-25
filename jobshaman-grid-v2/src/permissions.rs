#[derive(Debug, Deserialize)]
pub struct AgentPermissions {
    pub agent: AgentConfig,
    pub permissions: Permissions,
    pub watcher: WatcherConfig,
}

#[derive(Debug, Deserialize)]
pub struct AgentConfig {
    pub enabled: bool,
    pub backend: String,    // "ollama" nebo "llama"
    pub model: String,
}

#[derive(Debug, Deserialize)]
pub struct Permissions {
    pub access_web: bool,
    pub access_user_files: bool,
    pub execute_commands: bool,
    pub modify_grid: bool,
}

#[derive(Debug, Deserialize)]
pub struct WatcherConfig {
    pub vault_path: String,
    pub enable_watcher: bool,
}

impl AgentPermissions {
    pub fn load() -> Self {
        let proj_dirs = directories::ProjectDirs::from("com", "jobshaman", "grid").unwrap();
        let path = proj_dirs.config_dir().join("agent_permissions.toml");
        if path.exists() {
            let contents = std::fs::read_to_string(path).unwrap();
            toml::from_str(&contents).unwrap()
        } else {
            let default = Self {
                agent: AgentConfig { enabled: true, backend: "ollama".into(), model: "qwen2.5:3b".into() },
                permissions: Permissions { access_web: false, access_user_files: true, execute_commands: false, modify_grid: true },
                watcher: WatcherConfig { vault_path: "/home/shami/Documents/Obsidian".into(), enable_watcher: true },
            };
            let _ = std::fs::create_dir_all(proj_dirs.config_dir());
            let _ = std::fs::write(path, toml::to_string(&default).unwrap());
            default
        }
    }
}
