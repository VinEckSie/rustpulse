use chrono::{DateTime, Utc};
use std::net::IpAddr;
use uuid::Uuid;

#[allow(dead_code)]
pub enum NodeType {
    GroundStation,
    Simulator,
    Satellite,
    ControlCenter,
    UavDrone,
}

#[allow(dead_code)]
#[derive(Debug)]
enum NodeStatus {
    Live,
    Offline,
    Failure,
}

#[allow(dead_code)]
struct OrbitParameters {
    altitude_km: f64,
    inclination_deg: f64,
    period_min: f64,
}

#[allow(dead_code)]
struct Position {
    latitude: f64,
    longitude: f64,
    altitude: f64,
}

// generalizes a satellite, a ground station, or a simulator
#[allow(dead_code)]
pub struct Node {
    uuid: Uuid,
    name: String,
    pub server_type: NodeType,
    ip: IpAddr,
    status: NodeStatus,
    tags: Vec<String>,
    location: Option<String>,
    last_heartbeat: DateTime<Utc>,
    orbit: Option<OrbitParameters>,
    position: Option<Position>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_initialization() {
        let node = Node {
            uuid: Uuid::new_v4(),
            name: "Test Node".to_string(),
            server_type: NodeType::GroundStation,
            ip: "127.0.0.1".parse().unwrap(),
            status: NodeStatus::Live,
            tags: vec!["test".to_string()],
            location: Some("Earth".to_string()),
            last_heartbeat: Utc::now(),
            orbit: None,
            position: None,
        };

        assert_eq!(node.name, "Test Node");
        //assert_eq!(node.status, NodeStatus::Live);
        assert!(node.location.is_some());
    }
}
