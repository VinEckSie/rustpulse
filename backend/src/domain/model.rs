//domain entities
// 🧩 Server Model Breakdown
// Field	Why it’s great
// id	Needed for DB/relations (use uuid)
// name	Human-readable (e.g. “GroundStation-Kigali-01”)
// type	Key to generalize AND specialize (e.g. Simulator, GroundStation)
// ip	Makes it network-addressable
// status	Lets you track live/offline/failure/etc. — perfect for monitoring UI
//
// ✅ Optional upgrades later:
//
// tags: Vec<String> (for filtering/grouping)
//
// location: Option<String> (e.g. for ground stations)
//
// last_heartbeat: DateTime (for "is it still alive?" checks)
//

use std::net::IpAddr;
use uuid::{Timestamp, Uuid};

enum ServerType {
    Virtual,
    Simulator,
    GroundStation,
}

enum ServerStatus {
    Live,
    Offline,
    Failure,
}
pub struct Server {
    uuid: Uuid,
    name: String,
    server_type : ServerType,
    ip: IpAddr,
    status: ServerStatus,
    tags: Vec<String>,
    location: Option<String>,
    last_heartbeat: Timestamp,
}

pub struct Metric {
    id: Uuid,
    server_id: Uuid,
    cpu: usize,
    ram: usize,
    timestamp: Timestamp,
    connected_users: u16,
    network_usage: usize,
    disk_usage: usize,
    uptime: Timestamp,
    errors_detected: Option<Vec<String>>,
    anomaly: bool,
}

// 📈 Metric Model Breakdown
// Field	Why it’s great
// id	Unique per metric snapshot
// server_id	Foreign key reference to Server
// cpu, ram	Classic metrics
// timestamp	Crucial for graphs/alerts
// connected_users	Gives insight into load / service use
// network_usage	Good for telemetry simulation, I/O load, or anomaly detection
//
// ✅ Optional upgrades later:
//
// Add disk_usage, uptime, or errors_detected
//
// Flag anomalies: is_anomalous: bool
