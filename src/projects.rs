use std::path::PathBuf;
use std::fs;

pub struct Project {
    pub name: String,
    pub path: String,
}

impl Project {
    pub fn new(name: String, path: String) -> Self {
        Project { name, path }
    }

    pub fn open(&self, ide_cmd: &str) {
        #[cfg(target_os = "windows")]
        {
            std::process::Command::new(ide_cmd)
                .arg( &self.path)
                .spawn()
                .expect("Failed to open project");
        }

        #[cfg(target_os = "macos")]
        {
            std::process::Command::new(ide_cmd)
                .arg(&self.path)
                .spawn()
                .expect("Failed to open project");
        }

        #[cfg(target_os = "linux")]
        {
            std::process::Command::new(ide_cmd)
                .arg(&self.path)
                .spawn()
                .expect("Failed to open project");
        }
    }   
}

pub struct Projects {
    pub projects: Vec<Project>,
    pub user_config: bentobox::BentoBoxConfig,
}



fn find_folder_with_git_recursively(path: &PathBuf, projects: &mut Vec<Project>) {
    if path.join(".git").exists() {
        let project = Project::new(
            path.file_name().unwrap().to_string_lossy().into_owned(),
            path.to_string_lossy().into_owned(),
        );
        projects.push(project);
    }

    if let Ok(entries) = fs::read_dir(&path) {
        for entry in entries.filter_map(Result::ok) {
            if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                let current_path = entry.path();
                find_folder_with_git_recursively(&current_path, projects);
            }
        }
    }
}

pub fn get_projects() -> Projects {
    let user_config = bentobox::get_config_content();
    let mut dev_path = PathBuf::from(user_config.bentobox.user_dev_folder.clone());
    if dev_path.starts_with("~") {
        if let Some(home_dir) = dirs::home_dir() {
            dev_path = home_dir.join(dev_path.strip_prefix("~").unwrap());
        }
    }
    println!("Dev folder: {:?}", dev_path);
    let mut projects = Vec::new();
    find_folder_with_git_recursively(&dev_path, &mut projects);  

    Projects { projects, user_config }
}

