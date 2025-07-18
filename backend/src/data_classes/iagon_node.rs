use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum IagonNodeStatus {
    NodeStatusUp,
    NodeStatusDown,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IagonNodeResponse {
    pub status: Option<IagonNodeStatus>,
    pub info: Option<IagonNodeInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IagonNodeInfo {
    pub node_id: String,
    pub port: String,
    pub device: String,
    pub cpu: String,
    pub os: String,
    pub path: String,
    pub storage: String,
    pub country: String
}