use std::sync::{Arc, Mutex};
use sysinfo::System;
use serde::Serialize;
use axum::{Json, http::StatusCode};
use crate::auth::claims::ErrorResponse;

#[derive(Serialize)]
pub struct SystemInfo {
    pub cpu_usage: Vec<f32>,
    pub memory_usage: f32,
}

pub struct ResourceMonitor(pub Mutex<System>);

impl ResourceMonitor {

    pub fn get_instance() -> Arc<Self> {
        static INSTANCE: Mutex<Option<Arc<ResourceMonitor>>> = Mutex::new(None);
        
        let mut instance = INSTANCE.lock().unwrap();
        if let Some(ref instance) = *instance {
            instance.clone()
        } else {
            let sys = System::new_all();
            let new_instance = ResourceMonitor(Mutex::new(sys));
            let arc_instance = Arc::new(new_instance);
            *instance = Some(arc_instance.clone());
            arc_instance
        }
    }

    pub async fn get_system_info() -> Result<Json<SystemInfo>, ErrorResponse> {
        let instance = Self::get_instance();
        let mut system = instance.0.lock().map_err(|_| ErrorResponse(StatusCode::INTERNAL_SERVER_ERROR, "Failed to lock system info".to_string()))?;
        system.refresh_all();
        let info = SystemInfo {
            cpu_usage: get_cpu_usage_percent(&system),
            memory_usage: get_memory_usage_mb(&system),
        };
        Ok(Json(info))
    }
}

fn get_cpu_usage_percent(sys: &System) -> Vec<f32> {
    sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect()
}

fn get_memory_usage_mb(sys: &System) -> f32 {
    let total_memory = sys.total_memory() as f32 / 1024.0; // Convert to MB
    let used_memory = sys.used_memory() as f32 / 1024.0; // Convert to MB
    (used_memory / total_memory) * 100.0 // Return percentage
}
