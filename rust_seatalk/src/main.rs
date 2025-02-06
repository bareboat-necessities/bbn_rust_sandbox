use std::str;

#[derive(Debug)]
enum SteeringMode {
    Manual,
    Auto,
    Unknown(u8),
}

#[derive(Debug)]
enum HeadingStatus {
    Valid,
    Invalid,
}

#[derive(Debug)]
enum EngineAlarmType {
    Overheat,
    LowOilPressure,
    LowFuel,
    Custom(u8),
}

#[derive(Debug)]
enum SeatalkMessage {
    Depth {
        depth: f32, // in meters
    },
    Speed {
        speed: f32, // in knots
    },
    WaterTemperature {
        temperature: f32, // in degrees Celsius
    },
    WindData {
        apparent_wind_angle: f32, // in degrees
        apparent_wind_speed: f32, // in knots
    },
    AutopilotCommand {
        heading: f32, // in degrees
        mode: AutopilotMode,
    },
    GPSPosition {
        latitude: f32, // in degrees
        longitude: f32, // in degrees
    },
    GPSTimeDate {
        time: String, // HH:MM:SS
        date: String, // DD/MM/YYYY
    },
    RudderPosition {
        angle: f32, // in degrees
    },
    TripLog {
        trip_distance: f32, // in nautical miles
        total_distance: f32, // in nautical miles
    },
    Alarm {
        alarm_type: AlarmType,
    },
    WaypointLocation {
        latitude: f32, // in degrees
        longitude: f32, // in degrees
    },
    NavigationData {
        cross_track_error: f32, // in nautical miles
        bearing_to_waypoint: f32, // in degrees
        distance_to_waypoint: f32, // in nautical miles
    },
    RouteName {
        name: String,
    },
    WaypointName {
        name: String,
    },
    NavigationCommand {
        command: NavigationCommandType,
    },
    // Course-related messages
    CourseToSteer {
        heading: f32,        // Desired heading in degrees
        rudder_position: f32, // Rudder angle in degrees
        mode: SteeringMode,
    },
    VesselHeading {
        heading: f32,        // True heading in degrees
        status: HeadingStatus,
    },
    CrossTrackError {
        error: f32,          // Cross-track error in nautical miles
    },

    // Engine-related messages
    EngineRPM {
        rpm: u16,            // Engine RPM
        engine_number: u8,   // Engine identifier (e.g., 0=Port, 1=Starboard)
    },
    EngineTemperature {
        temperature: f32,    // Engine temperature in °C
        engine_number: u8,
    },
    EngineFuelData {
        fuel_flow: f32,      // Fuel flow rate (liters/hour)
        fuel_total: f32,     // Total fuel used (liters)
        engine_number: u8,
    },
    EngineAlarm {
        alarm_type: EngineAlarmType,
        engine_number: u8,
    },
    Unknown {
        message_type: u8,
        data: Vec<u8>,
    },
}

#[derive(Debug)]
enum AutopilotMode {
    Standby,
    Auto,
    Wind,
    Track,
    Unknown(u8),
}

#[derive(Debug)]
enum AlarmType {
    Depth,
    Anchor,
    Wind,
    Custom(u8),
}

#[derive(Debug)]
enum NavigationCommandType {
    GoToWaypoint,
    FollowRoute,
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

            // Water Temperature (Message Type 0x27)
            0x27 => {
                if data.len() >= 3 {
                    let temperature = ((u16::from(data[1]) & 0x7F) << 1) | ((u16::from(data[2]) & 0x80) >> 7);
                    let temperature = (temperature as f32) * 0.1; // Convert to degrees Celsius
                    Some(SeatalkMessage::WaterTemperature { temperature })
                } else {
                    None
                }
            }

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

            // GPS Position (Message Type 0x52)
            0x52 => {
                if data.len() >= 9 {
                    let lat_degrees = u16::from(data[1]);
                    let lat_minutes = u16::from(data[2]);
                    let lat_seconds = u16::from(data[3]);
                    let lat_direction = data[4];
                    let lon_degrees = u16::from(data[5]);
                    let lon_minutes = u16::from(data[6]);
                    let lon_seconds = u16::from(data[7]);
                    let lon_direction = data[8];

                    let latitude = (lat_degrees as f32) + (lat_minutes as f32) / 60.0 + (lat_seconds as f32) / 3600.0;
                    let latitude = if lat_direction == b'S' { -latitude } else { latitude };
                    let longitude = (lon_degrees as f32) + (lon_minutes as f32) / 60.0 + (lon_seconds as f32) / 3600.0;
                    let longitude = if lon_direction == b'W' { -longitude } else { longitude };

                    Some(SeatalkMessage::GPSPosition {
                        latitude,
                        longitude,
                    })
                } else {
                    None
                }
            }

            // GPS Time and Date (Message Type 0x53)
            0x53 => {
                if data.len() >= 7 {
                    let hour = data[1];
                    let minute = data[2];
                    let second = data[3];
                    let day = data[4];
                    let month = data[5];
                    let year = data[6];

                    let time = format!("{:02}:{:02}:{:02}", hour, minute, second);
                    let date = format!("{:02}/{:02}/{:02}", day, month, year);

                    Some(SeatalkMessage::GPSTimeDate { time, date })
                } else {
                    None
                }
            }

            // Rudder Position (Message Type 0x9C)
            0x9C => {
                if data.len() >= 3 {
                    let angle = ((u16::from(data[1]) & 0x7F) << 1) | ((u16::from(data[2]) & 0x80) >> 7);
                    let angle = (angle as f32) * 0.5; // Convert to degrees
                    Some(SeatalkMessage::RudderPosition { angle })
                } else {
                    None
                }
            }

            // Trip Log (Message Type 0x21)
            0x21 => {
                if data.len() >= 7 {
                    let trip_distance = ((u16::from(data[1]) & 0x7F) << 1) | ((u16::from(data[2]) & 0x80) >> 7);
                    let total_distance = ((u16::from(data[3]) & 0x7F) << 1) | ((u16::from(data[4]) & 0x80) >> 7);
                    let trip_distance = (trip_distance as f32) * 0.1; // Convert to nautical miles
                    let total_distance = (total_distance as f32) * 0.1; // Convert to nautical miles
                    Some(SeatalkMessage::TripLog {
                        trip_distance,
                        total_distance,
                    })
                } else {
                    None
                }
            }

            // Alarm (Message Type 0x86)
            0x86 => {
                if data.len() >= 2 {
                    let alarm_type = match data[1] {
                        0x01 => AlarmType::Depth,
                        0x02 => AlarmType::Anchor,
                        0x03 => AlarmType::Wind,
                        _ => AlarmType::Custom(data[1]),
                    };
                    Some(SeatalkMessage::Alarm { alarm_type })
                } else {
                    None
                }
            }

            // Waypoint Location (Message Type 0x5A)
            0x5A => {
                if data.len() >= 9 {
                    let lat_degrees = u16::from(data[1]);
                    let lat_minutes = u16::from(data[2]);
                    let lat_seconds = u16::from(data[3]);
                    let lat_direction = data[4];
                    let lon_degrees = u16::from(data[5]);
                    let lon_minutes = u16::from(data[6]);
                    let lon_seconds = u16::from(data[7]);
                    let lon_direction = data[8];

                    let latitude = (lat_degrees as f32) + (lat_minutes as f32) / 60.0 + (lat_seconds as f32) / 3600.0;
                    let latitude = if lat_direction == b'S' { -latitude } else { latitude };
                    let longitude = (lon_degrees as f32) + (lon_minutes as f32) / 60.0 + (lon_seconds as f32) / 3600.0;
                    let longitude = if lon_direction == b'W' { -longitude } else { longitude };

                    Some(SeatalkMessage::WaypointLocation {
                        latitude,
                        longitude,
                    })
                } else {
                    None
                }
            }

            // Navigation Data (Message Type 0x5B)
            0x5B => {
                if data.len() >= 7 {
                    let cross_track_error = ((u16::from(data[1]) & 0x7F) << 1) | ((u16::from(data[2]) & 0x80) >> 7);
                    let bearing_to_waypoint = ((u16::from(data[3]) & 0x7F) << 1) | ((u16::from(data[4]) & 0x80) >> 7);
                    let distance_to_waypoint = ((u16::from(data[5]) & 0x7F) << 1) | ((u16::from(data[6]) & 0x80) >> 7);

                    let cross_track_error = (cross_track_error as f32) * 0.1; // Convert to nautical miles
                    let bearing_to_waypoint = (bearing_to_waypoint as f32) * 0.5; // Convert to degrees
                    let distance_to_waypoint = (distance_to_waypoint as f32) * 0.1; // Convert to nautical miles

                    Some(SeatalkMessage::NavigationData {
                        cross_track_error,
                        bearing_to_waypoint,
                        distance_to_waypoint,
                    })
                } else {
                    None
                }
            }

            // Route Name (Message Type 0x5C)
            0x5C => {
                if data.len() >= 2 {
                    let name = String::from_utf8_lossy(&data[1..]).to_string();
                    Some(SeatalkMessage::RouteName { name })
                } else {
                    None
                }
            }

            // Waypoint Name (Message Type 0x5D)
            0x5D => {
                if data.len() >= 2 {
                    let name = String::from_utf8_lossy(&data[1..]).to_string();
                    Some(SeatalkMessage::WaypointName { name })
                } else {
                    None
                }
            }

            // Navigation Command (Message Type 0x5E)
            0x5E => {
                if data.len() >= 2 {
                    let command = match data[1] {
                        0x01 => NavigationCommandType::GoToWaypoint,
                        0x02 => NavigationCommandType::FollowRoute,
                        _ => NavigationCommandType::Unknown(data[1]),
                    };
                    Some(SeatalkMessage::NavigationCommand { command })
                } else {
                    None
                }
            }

            // Course to Steer (Message Type 0x85)
            0x85 => {
                if data.len() >= 5 {
                    let heading = ((u16::from(data[1]) << 8) | u16::from(data[2])) as f32 / 2.0;
                    let rudder_position = data[3] as i8 as f32; // Signed byte
                    let mode = match data[4] {
                        0x00 => SteeringMode::Manual,
                        0x01 => SteeringMode::Auto,
                        _ => SteeringMode::Unknown(data[4]),
                    };
                    Some(SeatalkMessage::CourseToSteer {
                        heading,
                        rudder_position,
                        mode,
                    })
                } else {
                    None
                }
            }

            // Vessel Heading (Message Type 0x9B)
            0x9B => {
                if data.len() >= 3 {
                    let heading = ((u16::from(data[1]) << 8) | u16::from(data[2])) as f32 / 2.0;
                    let status = match data[3] & 0x01 {
                        0x01 => HeadingStatus::Valid,
                        _ => HeadingStatus::Invalid,
                    };
                    Some(SeatalkMessage::VesselHeading { heading, status })
                } else {
                    None
                }
            }

            // Cross-Track Error (Message Type 0x8D)
            0x8D => {
                if data.len() >= 3 {
                    let error = ((u16::from(data[1]) << 8) | u16::from(data[2])) as f32 / 10.0;
                    Some(SeatalkMessage::CrossTrackError { error })
                } else {
                    None
                }
            }

            // Engine RPM (Message Type 0x30)
            0x30 => {
                if data.len() >= 3 {
                    let engine_number = data[1];
                    let rpm = (u16::from(data[2]) << 8) | u16::from(data[3]);
                    Some(SeatalkMessage::EngineRPM { rpm, engine_number })
                } else {
                    None
                }
            }

            // Engine Temperature (Message Type 0x31)
            0x31 => {
                if data.len() >= 3 {
                    let engine_number = data[1];
                    let temperature = data[2] as f32;
                    Some(SeatalkMessage::EngineTemperature {
                        temperature,
                        engine_number,
                    })
                } else {
                    None
                }
            }

            // Engine Fuel Data (Message Type 0x32)
            0x32 => {
                if data.len() >= 5 {
                    let engine_number = data[1];
                    let fuel_flow = ((u16::from(data[2]) << 8) | u16::from(data[3])) as f32 / 10.0;
                    let fuel_total = ((u16::from(data[4]) << 8) | u16::from(data[5])) as f32;
                    Some(SeatalkMessage::EngineFuelData {
                        fuel_flow,
                        fuel_total,
                        engine_number,
                    })
                } else {
                    None
                }
            }

            // Engine Alarm (Message Type 0x87)
            0x87 => {
                if data.len() >= 3 {
                    let engine_number = data[1];
                    let alarm_type = match data[2] {
                        0x01 => EngineAlarmType::Overheat,
                        0x02 => EngineAlarmType::LowOilPressure,
                        0x03 => EngineAlarmType::LowFuel,
                        _ => EngineAlarmType::Custom(data[2]),
                    };
                    Some(SeatalkMessage::EngineAlarm {
                        alarm_type,
                        engine_number,
                    })
                } else {
                    None
                }
            }

            // Unknown message type
            _ => Some(SeatalkMessage::Unknown {
                message_type: data[0],
                data: data.to_vec(),
            }),
        }
    }
}

fn main() {
    // Example NMEA sentences containing Seatalk data
    let nmea_sentences = [
        "$STALK,00010203", // Depth
        "$STALK,20040506", // Speed
        "$STALK,27070809", // Water Temperature
        "$STALK,100A0B0C", // Wind Data
        "$STALK,840D0E0F", // Autopilot Command
        "$STALK,521112131415161718", // GPS Position
        "$STALK,531A1B1C1D1E1F", // GPS Time and Date
        "$STALK,9C202122", // Rudder Position
        "$STALK,212324252627", // Trip Log
        "$STALK,862829", // Alarm
        "$STALK,5A2A2B2C2D2E2F303132", // Waypoint Location
        "$STALK,5B333435363738", // Navigation Data
        "$STALK,5C526F7574652031", // Route Name ("Route 1")
        "$STALK,5D576179706F696E742031", // Waypoint Name ("Waypoint 1")
        "$STALK,5E01", // Navigation Command (Go To Waypoint)
        "$STALK,8501F20300", // Course to Steer (heading=241.5°, rudder=-3°, manual)
        "$STALK,9B03E801",   // Vessel Heading (heading=244.0°, valid)
        "$STALK,8D0019",     // Cross-Track Error (2.5 nm)
        "$STALK,30011234",   // Engine RPM (engine 1, RPM=4660)
        "$STALK,310245",     // Engine Temperature (engine 2, 69°C)
        "$STALK,3203006401F4", // Engine Fuel (engine 3, 10.0 L/h, 500 L total)
        "$STALK,870201",     // Engine Alarm (engine 2, low oil pressure)
        "$STALK,FF2A2B2C", // Unknown message
    ];

    for sentence in nmea_sentences {
        if let Some(seatalk_message) = SeatalkMessage::parse_nmea_sentence(sentence) {
            println!("Parsed Seatalk Message: {:?}", seatalk_message);
        } else {
            println!("Failed to parse NMEA sentence: {}", sentence);
        }
    }
}
