use serde::Serialize;

#[derive(Serialize)]
pub struct HealthCheckStatus {
    pub status: String,
}

impl HealthCheckStatus {
    pub fn new() -> Self {
        Self {
            status: "ok".to_string(),
        }
    }
}

impl Default for HealthCheckStatus {
    fn default() -> Self {
        HealthCheckStatus::new()
    }
}
