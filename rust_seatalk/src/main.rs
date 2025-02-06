use std::str;

#[derive(Debug)]
enum SeatalkMessage {
    WindData {
        apparent_wind_angle: f32, // in degrees
        apparent_wind_speed: f32, // in knots
    },
    AutopilotCommand {
        heading: f32, // in degrees
        mode: AutopilotMode,
    },
    Depth {
        depth: f32, // in meters
    },
    Speed {
        speed: f32, // in knots
    },
    Unknown(Vec<u8>), // For unrecognized messages
}

#[derive(Debug)]
enum AutopilotMode {
    Standby,
    Auto,
    Wind,
    Track,
    Unknown(u8),
}

impl SeatalkMessage {
    fn parse_nmea_sentence(sentence: &str) -> Option<Self> {
        // Ensure the sentence starts with $STALK
        if !sentence.starts_with("$STALK") {
            return None;
        }

        // Split the sentence into parts
        let parts: Vec<&str> = sentence.split(',').collect();
        if parts.len() < 2 {
            return None;
        }

        // Extract the Seatalk data in hexadecimal format
        let hex_data = parts[1];
        let data = hex::decode(hex_data).ok()?;

        // Parse the Seatalk message
        Self::parse_seatalk_data(&data)
    }

    fn parse_seatalk_data(data: &[u8]) -> Option<Self> {
        if data.is_empty() {
            return None;
        }

        match data[0] {
            // Wind Data (Message Type 0x10)
            0x10 => {
                if data.len() >= 5 {
                    let angle = ((u16::from(data[1]) & 0x7F) << 1) | ((u16::from(data[2]) & 0x80) >> 7);
                    let speed = ((u16::from(data[2]) & 0x7F) << 1) | ((u16::from(data[3]) & 0x80) >> 7);
                    let apparent_wind_angle = (angle as f32) * 0.5; // Convert to degrees
                    let apparent_wind_speed = (speed as f32) * 0.1; // Convert to knots
                    Some(SeatalkMessage::WindData {
                        apparent_wind_angle,
                        apparent_wind_speed,
                    })
                } else {
                    None
                }
            }

            // Autopilot Command (Message Type 0x84)
            0x84 => {
                if data.len() >= 4 {
                    let heading = ((u16::from(data[1]) & 0x7F) << 1) | ((u16::from(data[2]) & 0x80) >> 7);
                    let heading = (heading as f32) * 0.5; // Convert to degrees
                    let mode = match data[3] {
                        0x00 => AutopilotMode::Standby,
                        0x01 => AutopilotMode::Auto,
                        0x02 => AutopilotMode::Wind,
                        0x03 => AutopilotMode::Track,
                        _ => AutopilotMode::Unknown(data[3]),
                    };
                    Some(SeatalkMessage::AutopilotCommand { heading, mode })
                } else {
                    None
                }
            }

            // Depth (Message Type 0x00)
            0x00 => {
                if data.len() >= 4 {
                    let depth = ((u16::from(data[1]) & 0x7F) << 1) | ((u16::from(data[2]) & 0x80) >> 7);
                    let depth = (depth as f32) * 0.1; // Convert to meters
                    Some(SeatalkMessage::Depth { depth })
                } else {
                    None
                }
            }

            // Speed (Message Type 0x20)
            0x20 => {
                if data.len() >= 3 {
                    let speed = ((u16::from(data[1]) & 0x7F) << 1) | ((u16::from(data[2]) & 0x80) >> 7);
                    let speed = (speed as f32) * 0.1; // Convert to knots
                    Some(SeatalkMessage::Speed { speed })
                } else {
                    None
                }
            }

            // Unknown message type
            _ => Some(SeatalkMessage::Unknown(data.to_vec())),
        }
    }
}

fn main() {
    // Example NMEA sentences containing Seatalk data
    let nmea_sentences = [
        "$STALK,10010203", // Wind Data
        "$STALK,84040506", // Autopilot Command
        "$STALK,00070809", // Depth
        "$STALK,200A0B0C", // Speed
        "$STALK,FF0D0E0F", // Unknown message
    ];

    for sentence in nmea_sentences {
        if let Some(seatalk_message) = SeatalkMessage::parse_nmea_sentence(sentence) {
            println!("Parsed Seatalk Message: {:?}", seatalk_message);
        } else {
            println!("Failed to parse NMEA sentence: {}", sentence);
        }
    }
}
