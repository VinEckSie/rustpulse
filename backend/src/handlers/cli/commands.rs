// use crate::app::metrics_service;
// use crate::core::domains::node::NodeType;
// use super::args::{CliArgs, Commands};

// pub async fn handle_command(args: CliArgs) {
//     match args.command {
//         Commands::Metrics { target, output } => {
//             let target = match target.to_lowercase().as_str() {
//                 "groundstation" => NodeType::GroundStation,
//                 "simulator" => NodeType::Simulator,
//                 "satellite" => NodeType::Satellite,
//                 "controlcenter" => NodeType::ControlCenter,
//                 "uavdrone" => NodeType::UavDrone,
//                 _ => {
//                     eprintln!("Unknown target type: {}", target);
//                     return;
//                 }
//             };
//
//             let telemetry = metrics_service::get_mock_metrics(target).await; // Or real call
//             match output.as_str() {
//                 "json" => println!("{}", serde_json::to_string_pretty(&telemetry).unwrap()),
//                 "text" => println!("{:#?}", telemetry),
//                 _ => eprintln!("Unknown output format: {}", output),
//             }
//         }
//         Commands::Health => {
//             println!("RustPulse is operational. ✅");
//         }
//     }
// }
