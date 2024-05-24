use serde::Serialize;

#[derive(Serialize)]
pub struct HealthCheck {
    pub status: String,
}

impl HealthCheck {
    pub fn new() -> Self {
        Self {
            status: "ok".to_string(),
        }
    }
}

impl Default for HealthCheck {
    fn default() -> Self {
        HealthCheck::new()
    }
}
