use chrono::{DateTime, Utc};
use std::net::IpAddr;
use uuid::Uuid;

#[allow(dead_code)]
pub enum NodeType {
    //Aerospace
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: Uuid,
    pub name: String,
    pub kind: NodeType,
    pub ip: Option<IpAddr>,
    pub status: NodeStatus,
    pub tags: Vec<String>,
    pub location: Option<String>,
    pub last_heartbeat: Option<DateTime<Utc>>,
    pub orbit: Option<Orbit>,
    pub position: Option<Position>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_initialization() {
        let uuid_local = Uuid::parse_str("a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8");

        let node = Node {
            uuid: uuid_local.unwrap(),
            // uuid: Uuid::new_v4(),
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
