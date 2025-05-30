
#[derive(Debug)]
pub struct NmeaSentence {
    pub sentence_type: char,
    // '$' for standard NMEA, '!' for AIS, '/' for Inmarsat-C
    pub talker_id: String,
    pub message_type: String,
    pub data_fields: Vec<String>,
    pub checksum: Option<String>,
}

#[derive(Debug)]
pub struct InmarsatHeader {
    pub group_id: String,
    pub sequence_number: Option<String>,
    pub station_id: Option<String>,
    pub timestamp: Option<String>,
}

#[derive(Debug)]
pub struct InmarsatMessage {
    pub header: InmarsatHeader,
    pub payload: NmeaSentence,
}

#[derive(Debug)]
pub struct SeatalkMessage {
    pub data_fields: Vec<String>,
    pub checksum: String,
}

#[derive(Debug)]
pub struct DscMessage {
    pub format_specifier: String,
    pub address: String,
    pub category: String,
    pub self_id: String,
    pub subsequent_communications: String,
    pub data_fields: Vec<String>,
    pub checksum: Option<String>,
}

#[derive(Debug)]
pub enum MessageType {
    Nmea(NmeaSentence),
    Ais(NmeaSentence),
    Inmarsat(InmarsatMessage),
    Seatalk(SeatalkMessage),
    Dsc(DscMessage),
}

pub fn parse_inmarsat_header(header: &str) -> Result<InmarsatHeader, &'static str> {
    let parts: Vec<&str> = header.split(',').collect();
    if parts.is_empty() {
        return Err("Invalid Inmarsat header");
    }

    let group_id = parts[0].to_string();
    let mut sequence_number = None;
    let mut station_id = None;
    let mut timestamp = None;

    for part in parts.iter().skip(1) {
        if part.starts_with("s:") {
            station_id = Some(part[2..].to_string());
        } else if part.starts_with("n:") {
            sequence_number = Some(part[2..].to_string());
        } else if part.starts_with("c:") {
            timestamp = Some(part[2..].to_string());
        }
    }

    Ok(InmarsatHeader {
        group_id,
        sequence_number,
        station_id,
        timestamp,
    })
}

pub fn parse_nmea_sentence(sentence: &str) -> Result<NmeaSentence, &'static str> {
    // Check if the sentence starts with '$', '!', or '/'
    if !sentence.starts_with('$') && !sentence.starts_with('!') && !sentence.starts_with('/') {
        return Err("Sentence must start with '$', '!', or '/'");
    }

    let sentence_type = sentence.chars().next().unwrap(); // Extract the first character
    let content = &sentence[1..]; // Remove the leading '$', '!', or '/'

    // Split into data part and checksum part
    let (data_part, checksum_part) = match content.split_once('*') {
        Some((data, checksum)) => (data, Some(checksum)),
        None => (content, None),
    };

    // Split data part into fields
    let fields: Vec<&str> = data_part.split(',').collect();
    if fields.is_empty() {
        return Err("No data fields found");
    }

    // Extract talker ID and message type from the first field
    let (talker_id, message_type) = {
        let mt_field = fields[0];
        (
            mt_field.get(0..2).unwrap_or_default().to_string(),
            mt_field.get(2..).unwrap_or_default().to_string(),
        )
    };

    // Process remaining data fields
    let data_fields = fields[1..]
        .iter()
        .map(|field| field.to_string())
        .collect();

    // Process checksum if present
    let checksum = checksum_part.and_then(|cs| {
        if cs.len() >= 2 {
            Some(cs[0..2].to_string())
        } else {
            None
        }
    });

    Ok(NmeaSentence {
        sentence_type,
        talker_id,
        message_type,
        data_fields,
        checksum,
    })
}

pub fn parse_dsc_message(message: &str) -> Result<DscMessage, &'static str> {
    // DSC messages start with $ and have DSC as talker ID
    if !message.starts_with('$') {
        return Err("DSC message must start with '$'");
    }

    let content = &message[1..]; // Remove the leading '$'

    // Split into data part and checksum part
    let (data_part, checksum_part) = match content.split_once('*') {
        Some((data, checksum)) => (data, Some(checksum)),
        None => (content, None),
    };

    // Split data part into fields
    let fields: Vec<&str> = data_part.split(',').collect();
    if fields.len() < 6 {
        return Err("DSC message must have at least 6 fields");
    }

    // Verify talker ID is DSC
    if fields[0] != "DSC" {
        return Err("DSC message must have 'DSC' as talker ID");
    }

    // Extract required DSC fields
    let format_specifier = fields[1].to_string();
    let address = fields[2].to_string();
    let category = fields[3].to_string();
    let self_id = fields[4].to_string();
    let subsequent_communications = fields[5].to_string();

    // Process remaining data fields
    let data_fields = fields[6..]
        .iter()
        .map(|field| field.to_string())
        .collect();

    // Process checksum if present
    let checksum = checksum_part.and_then(|cs| {
        if cs.len() >= 2 {
            Some(cs[0..2].to_string())
        } else {
            None
        }
    });

    Ok(DscMessage {
        format_specifier,
        address,
        category,
        self_id,
        subsequent_communications,
        data_fields,
        checksum,
    })
}

pub fn parse_inmarsat_message(message: &str) -> Result<InmarsatMessage, &'static str> {
    // Split the message into header and payload
    let (header_part, payload_part) = match message.split_once('/') {
        Some((header, payload)) => (header, payload),
        None => return Err("Invalid Inmarsat message format"),
    };

    // Parse the header
    let header = parse_inmarsat_header(header_part)?;

    // Parse the payload as an NMEA sentence
    let payload = parse_nmea_sentence(&format!("/{}", payload_part))?;

    Ok(InmarsatMessage { header, payload })
}

pub fn parse_seatalk_message(message: &str) -> Result<SeatalkMessage, &'static str> {
    // Check if the sentence starts with '$'
    if !message.starts_with('$') {
        return Err("SeaTalk sentence must start with '$'");
    }

    let content = &message[1..]; // Remove the leading '$'

    // Split into data part and checksum part
    let (data_part, checksum_part) = match content.split_once('*') {
        Some((data, checksum)) => (data, checksum.to_string()),
        None => return Err("Invalid SeaTalk sentence format"),
    };

    // Split data part into fields
    let fields: Vec<&str> = data_part.split(',').collect();
    if fields.is_empty() {
        return Err("No data fields found");
    }

    // Process remaining data fields
    let data_fields = fields[1..]
        .iter()
        .map(|field| field.to_string())
        .collect();

    Ok(SeatalkMessage {
        data_fields,
        checksum: checksum_part,
    })
}

pub fn detect_and_parse_message(message: &str) -> Result<MessageType, &'static str> {
    if message.starts_with('$') {
        if message.starts_with("$STALK") {
            // SeaTalk message
            let seatalk_message = parse_seatalk_message(message)?;
            Ok(MessageType::Seatalk(seatalk_message))
        } else if message.starts_with("$DSC") {
            // DSC message
            let dsc_message = parse_dsc_message(message)?;
            Ok(MessageType::Dsc(dsc_message))
        } else {
            // NMEA message
            let nmea_sentence = parse_nmea_sentence(message)?;
            Ok(MessageType::Nmea(nmea_sentence))
        }
    } else if message.starts_with('!') {
        // AIS message
        let nmea_sentence = parse_nmea_sentence(message)?;
        Ok(MessageType::Ais(nmea_sentence))
    } else if message.starts_with('/') {
        // Inmarsat-C message
        let inmarsat_message = parse_inmarsat_message(message)?;
        Ok(MessageType::Inmarsat(inmarsat_message))
    } else {
        Err("Unknown message format")
    }
}

// Example usage
fn main() {
    let examples = vec![
        // NMEA example
        "$GPGGA,002153.000,3342.6618,N,11751.3858,W,1,10,1.2,27.0,M,-34.2,M,,0000*5A",
        // AIS example
        "!AIVDM,1,1,,A,13HOI:0P0000VOHLCnHQKwvL05Ip,0*23",
        // Inmarsat-C example
        "/g:1-9-1234,s:egcterm1,n:213,c:1333636200*hh/$CSSM3,123456,005213,798,0,3,14,00,2012,04,05,14,30,3400,N,076,W,300*hh",
        // SeaTalk example
        "$STALK,84,56,e,0,0,0,0,0,8*0F",
        // DSC example
        "$DSC,12,3380405810,00,1234567890,72,1234567890,00,00,00,00,00,00,00*3B",
    ];

    for example in examples {
        println!("Parsing message: {}", example);
        match detect_and_parse_message(example) {
            Ok(message) => match message {
                MessageType::Nmea(nmea) => println!("NMEA Sentence: {:?}", nmea),
                MessageType::Ais(ais) => println!("AIS Sentence: {:?}", ais),
                MessageType::Inmarsat(inmarsat) => println!("Inmarsat Message: {:?}", inmarsat),
                MessageType::Seatalk(seatalk) => println!("SeaTalk Message: {:?}", seatalk),
                MessageType::Dsc(dsc) => println!("DSC Message: {:?}", dsc),
            },
            Err(e) => eprintln!("Error parsing message: {}", e),
        }
        println!();
    }
}
