use std::net::IpAddr;
use chrono::{DateTime, Utc};
use uuid::Uuid;

enum NodeType {
    GroundStation,
    Simulator,
    Satellite,
    ControlCenter,
    UavDrone,
}

enum NodeStatus {
    Live,
    Offline,
    Failure,
}

struct OrbitParameters {
    altitude_km: f64,
    inclination_deg: f64,
    period_min: f64,
}

struct Position {
    latitude: f64,
    longitude: f64,
    altitude: f64,
}

// generalizes a satellite, a ground station, or a simulator
pub struct Node {
    uuid: Uuid,
    name: String,
    server_type : NodeType,
    ip: IpAddr,
    status: NodeStatus,
    tags: Vec<String>,
    location: Option<String>,
    last_heartbeat: DateTime<Utc>,
    orbit: Option<OrbitParameters>,
    position: Option<Position>,
}