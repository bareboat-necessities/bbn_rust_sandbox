struct TransducerMeasurement {
    value: f32,
    transducer_type: char,
    unit: char,
    name: String,
}

fn generate_xdr(measurements: &[TransducerMeasurement]) -> String {
    let mut data = String::from("IIXDR");
    for m in measurements {
        data.push_str(
            &format!(",{:.1},{},{},{}",
                     m.value,
                     m.transducer_type,
                     m.unit,
                     m.name
            )
        );
    }
    let checksum = calculate_checksum(&data);
    format!("${}*{:02X}", data, checksum)
}

fn calculate_checksum(data: &str) -> u8 {
    data.bytes().fold(0, |sum, b| sum ^ b)
}

fn main() {
    let measurements = vec![
        TransducerMeasurement {
            value: 29.5,
            transducer_type: 'P',
            unit: 'B',
            name: "ENGINE".to_string(),
        },
        TransducerMeasurement {
            value: 25.0,
            transducer_type: 'T',
            unit: 'C',
            name: "COOLING".to_string(),
        },
    ];

    let sentence = generate_xdr(&measurements);
    println!("Generated XDR sentence: {}", sentence);
}
