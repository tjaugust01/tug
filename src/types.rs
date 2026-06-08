use std::time::Duration;

pub struct ContainerInfo {
    pub id: String,
    pub name: String,
    pub image: String,
    pub status: ContainerStatus,
    pub uptime: Option<Duration>,
    pub ports: Vec<String>,
    pub stats: Option<ContainerStats>,
    pub compose_project: Option<String>,
}
pub struct ContainerStats {
    pub cpu_percent: f64,
    pub mem_usage: u64,
    pub mem_limit: u64,
}
enum ContainerStatus {
    Created,
    Restarting,
    Running,
    Removing,
    Paused,
    Exited,
    Dead
}

pub struct ImageInfo {
    pub id: String,
    pub repository: String,
    pub tags: Vec<String>,
    pub size: u64,
    pub in_use: bool
}

pub struct ComposeStack {
    pub id: String,
    pub containers: Vec<ContainerInfo>,
    pub images: Vec<ImageInfo>,
}