use crate::bento_grid::BentoGrid;
use directories::ProjectDirs;
use std::fs;

pub fn save_bento_grid(grid: &BentoGrid) {
    if let Some(proj_dirs) = ProjectDirs::from("com", "jobshaman", "grid") {
        let path = proj_dirs.config_dir().join("bento_layout.ron");
        let data = ron::to_string(&grid.to_save_data()).unwrap();
        let _ = fs::create_dir_all(proj_dirs.config_dir());
        let _ = fs::write(path, data);
    }
}

pub fn load_bento_grid() -> Option<BentoGrid> {
    let proj_dirs = ProjectDirs::from("com", "jobshaman", "grid")?;
    let path = proj_dirs.config_dir().join("bento_layout.ron");
    if path.exists() {
        let contents = fs::read_to_string(path).ok()?;
        let data: Vec<(usize, usize, usize, usize, usize)> = ron::from_str(&contents).ok()?;
        Some(BentoGrid::from_save_data(data))
    } else {
        None
    }
}
