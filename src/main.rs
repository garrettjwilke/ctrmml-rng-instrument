use std::collections::HashMap;
use std::io::Read;
use flate2::read::GzDecoder;
use rand::Rng;

const COMPRESSED_DATA: &[u8] = include_bytes!("instruments.txt.gz");

fn main() {
    let decompressed = decompress_gzip(COMPRESSED_DATA).expect("Failed to decompress instrument data");
    let instruments = parse_instruments(&decompressed);

    let keys: Vec<&u32> = instruments.keys().collect();
    if keys.is_empty() {
        println!("No instruments found.");
        return;
    }

    let mut rng = rand::thread_rng();
    let random_key = keys[rng.gen_range(0..keys.len())];
    println!("Instrument {}:\n{}", random_key, instruments[random_key]);
}

fn decompress_gzip(data: &[u8]) -> Result<String, std::io::Error> {
    let mut decoder = GzDecoder::new(data);
    let mut output = String::new();
    decoder.read_to_string(&mut output)?;
    Ok(output)
}

fn parse_instruments(data: &str) -> HashMap<u32, String> {
    let mut instruments = HashMap::new();
    let mut current_id = None;
    let mut current_lines = Vec::new();

    for line in data.lines() {
        if let Some(start_idx) = line.find("START INSTRUMENT") {
            if let Some(id_str) = line[start_idx..].split_whitespace().last() {
                if let Ok(id) = id_str.parse::<u32>() {
                    current_id = Some(id);
                    current_lines.clear();
                }
            }
        } else if line.contains("END INSTRUMENT") {
            if let Some(id) = current_id {
                instruments.insert(id, current_lines.join("\n"));
                current_id = None;
            }
        } else if current_id.is_some() {
            current_lines.push(line.to_string());
        }
    }

    instruments
}

