pub struct GrubEnv {
    root: String,
    work_dir: String
}

impl GrubEnv {
    pub fn new(wd: &str) -> Self {
        Self { root: ".".to_string(), work_dir: wd.to_string() } 
    }

    pub fn set_root_path(&mut self, root: String) {
        self.root = root;
    }

    pub fn set_root_path_str(&mut self, root: &str) {
        self.root = root.to_string();
    }

    pub fn get_work_dir(&self) -> String {
        self.root + "\\" + self.work_dir.as_str()
    }
}