use anyhow::Result;

// This is using the RLE algo
// for example hellloo
// equals to he3l2o

#[derive(Debug)]
pub struct CompressedPart {
    pub letter: char,
    pub times: u8,
}

pub fn get_compressed(text: String) -> Result<String> {
    let mut counter: Vec<CompressedPart> = Vec::new();

    for letter in text.chars() {
        if let Some(last) = counter.last_mut() {
            if last.letter == letter {
                last.times += 1;
            } else {
                counter.push(CompressedPart { letter, times: 1 });
            }
        } else {
            counter.push(CompressedPart { letter, times: 1 });
        }
    }

    let mut text_compressed = String::new();

    for compressed in counter {
        if compressed.times == 1 {
            text_compressed += &compressed.letter.to_string();
            continue;
        }

        let text = format!("{}{}", compressed.times, compressed.letter);

        text_compressed += text.as_str();
    }

    Ok(text_compressed)
}
