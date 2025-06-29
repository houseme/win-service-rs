use config::Config;
use serde::Deserialize;
use win_service_rs::StartType;

#[derive(Debug, Deserialize)]
pub struct WindConfig {
    pub service: ServiceConfig,
    pub manager: ManagerConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServiceConfig {
    pub name: String,
    pub exe_path: String,
    pub display_name: String,
    pub description: String,
    pub start_type: String,
}

#[derive(Debug, Deserialize)]
pub struct ManagerConfig {
    pub log_level: String,
}

impl ServiceConfig {
    pub fn to_win_service_config(&self) -> win_service_rs::ServiceConfig {
        let start_type = match self.start_type.to_lowercase().as_str() {
            "auto" => StartType::Auto,
            "manual" => StartType::Manual,
            "demand" => StartType::Demand,
            "disabled" => StartType::Disabled,
            _ => StartType::Demand,
        };
        win_service_rs::ServiceConfig::new(self.exe_path.clone(), self.display_name.clone())
            .with_description(self.description.clone())
            .with_start_type(start_type)
    }
}

pub fn load_config() -> anyhow::Result<WindConfig> {
    let cfg = Config::builder()
        .add_source(config::File::with_name("config/config"))
        .build()?;
    cfg.try_deserialize().map_err(anyhow::Error::from)
}
