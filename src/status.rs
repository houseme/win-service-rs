#[derive(Debug, Clone, PartialEq)]
pub enum ServiceStatus {
    Stopped,
    StartPending,
    StopPending,
    Running,
    ContinuePending,
    PausePending,
    Paused,
    Unknown,
}

impl From<u32> for ServiceStatus {
    fn from(state: u32) -> Self {
        match state {
            1 => ServiceStatus::Stopped,
            2 => ServiceStatus::StartPending,
            3 => ServiceStatus::StopPending,
            4 => ServiceStatus::Running,
            5 => ServiceStatus::ContinuePending,
            6 => ServiceStatus::PausePending,
            7 => ServiceStatus::Paused,
            _ => ServiceStatus::Unknown,
        }
    }
}
