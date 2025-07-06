// use crate::core::domains::{node::Node, telemetry::NodeTelemetry};
// use crate::core::port::SimulatedTelemetryRepository;

#[allow(dead_code)]
pub struct SimulatedTelemetrySource {
    pub rng: rand::rngs::ThreadRng,
}

// impl TelemetrySourceRepository for simTelemetrySource {
//     fn poll(&mut self, node: &Node) -> NodeTelemetry {
//         match node.server_type {
//             NodeType::Satellite => self.sim_satellite(node),
//             NodeType::UavDrone => self.sim_uav(node),
//             NodeType::Simulator => self.sim_simulator(node),
//             NodeType::ControlCenter => self.sim_control_center(node),
//             NodeType::GroundStation => self.sim_ground(node),
//         }
//     }
// }
//
// ✅ sim_satellite(&self, node: &Node) -> NodeTelemetry
// cpu: random between 15–60%
//
// battery_level: 40–100%
//
// signal_strength: simulate fade-outs (0–1, drops to 0 every N cycles)
//
// orientation: random (pitch, roll, yaw)
//
// anomaly: 1 in 50 chance true
//
// ✅ sim_uav(&self, node: &Node) -> NodeTelemetry
// cpu: bursts to 90–100% occasionally
//
// battery_level: drops linearly if not recharged (simulate “battery drain”)
//
// connected_users: 0–2
//
// disk_usage: static
//
// temperature: 30–80°C
//
// ✅ sim_simulator(...)
// Simulate perfect values:
//
// cpu: 25%
//
// ram: 4 GB fixed
//
// anomaly: always false
//
// Used to test the system itself
//
// ✅ sim_ground(...)
// High network usage (20–90 Mbps)
//
// CPU steady around 50%
//
// uptime: always increasing
//
// ✅ sim_control_center(...)
// Rare telemetry changes
//
// Logs anomalies (simulate many errors in dev mode)
//
// Low CPU, high memory
//
// 💣 Bonus Challenge: Implement stateful drift
// Instead of generating completely random numbers every time:
//
// Add an internal state like battery_level
//
// Each poll() drifts from last value: +/– delta
//
// rust
// Copy
// Edit
// self.battery_level -= 0.5;
// if self.battery_level < 20.0 { /* flag anomaly */ }
// ➡️ Makes your simulation more realistic (and testable over time)
//
// 🧪 Example Output for Satellite Node
// json
// Copy
// Edit
// {
// "id": "uuid",
// "server_id": "node-uuid",
// "cpu": 43.2,
// "ram": 6.2,
// "timestamp": "2025-07-03T14:53:00Z",
// "connected_users": 0,
// "disk_usage": 31.2,
// "uptime": "PT17200S",
// "battery_level": 78.5,
// "temperature": 67.3,
// "signal_strength": 0.91,
// "orientation": [1.2, 0.5, -0.1],
// "anomaly": false
// }
