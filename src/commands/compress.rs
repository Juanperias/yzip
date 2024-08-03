use anyhow::{anyhow, Result};
use std::{
    fs::{self, metadata, File},
    io::Write,
    path::Path,
};

use crate::utils;

pub fn compress_command(file: String) -> Result<()> {
    let path = Path::new(file.as_str());

    if !path.exists() {
        let message = format!("{} not found", file);
        return Err(anyhow!(message));
    }

    let old_file_len = metadata(file.clone())?.len();

    let file_name_split: Vec<String> = file.split('.').map(|s| s.to_string()).collect();
    let compress_file_name = format!("{}.yzip", file_name_split[0].clone());
    let content = fs::read_to_string(path)?;
    let compressed = utils::get_compressed::get_compressed(content)?;
    let mut file = File::create(compress_file_name.clone())?;
    file.write_all(compressed.as_bytes())?;
    let compressed_len = metadata(compress_file_name.clone())?.len();
    println!("old: {}, new: {}", old_file_len, compressed_len);
    println!("Reduced {} bytes!", (old_file_len - compressed_len));

    Ok(())
}
