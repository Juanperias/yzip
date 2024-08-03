use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn decompress_command(file: String, dist: String) -> Result<()> {
    let path = Path::new(file.as_str());

    if !path.exists() {
        let message = format!("{} not found", file);
        return Err(anyhow!(message));
    }

    let text = std::fs::read_to_string(path)?;

    let mut times = String::from("1");
    let mut descompressed_text = String::new();

    for char in text.chars() {
        if char.is_digit(10) {
            if times == "1" {
                times.clear();
            }
            times.push(char);
        } else {
            let times_int: usize = times.parse()?;
            descompressed_text.push_str(&char.to_string().repeat(times_int));
            times = String::from("1");
        }
    }

    let mut file = File::create(dist)?;
    file.write_all(descompressed_text.as_bytes())?;
    println!("Descompressed file");

    Ok(())
}
