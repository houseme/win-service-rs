#[derive(Clone, Debug)]
pub struct ServiceConfig {
    pub exe_path: String,
    pub display_name: String,
    pub description: String,
    pub start_type: StartType,
    pub dependencies: Vec<String>,
}

#[derive(Clone, Debug)]
pub enum StartType {
    Auto,
    Manual,
    Demand,
    Disabled,
}

impl ServiceConfig {
    pub fn new(exe_path: String, display_name: String) -> Self {
        ServiceConfig {
            exe_path,
            display_name,
            description: String::new(),
            start_type: StartType::Demand,
            dependencies: Vec::new(),
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn with_start_type(mut self, start_type: StartType) -> Self {
        self.start_type = start_type;
        self
    }

    pub fn with_dependencies(mut self, dependencies: Vec<String>) -> Self {
        self.dependencies = dependencies;
        self
    }
}
